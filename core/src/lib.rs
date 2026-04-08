use wasm_bindgen::prelude::*;
use agents::{
    Agent, AgentContext, LLMClient, AgentResult, 
    planner::PlannerAgent, 
    tutor::TutorAgent, 
    critic::CriticAgent, 
    synthesizer::SynthesizerAgent,
    llm::GenericLLMClient
};
use github::GithubClient;
use wiki::{WikiEngine, WikiGraph};
use pedagogy::{PedagogyEngine, TeachingStrategy};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use futures::future::join_all;

#[wasm_bindgen]
pub struct DeepTutorEngine {
    github: GithubClient,
    wiki: WikiEngine,
    llm: Box<dyn LLMClient>,
    session_id: String,
    current_session_frames: Vec<memvid_core::types::Frame>,
}

#[wasm_bindgen]
impl DeepTutorEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(
        token: String, 
        owner: String, 
        repo: String, 
        llm_api_key: String, 
        llm_base_url: String, 
        llm_model: String
    ) -> Result<DeepTutorEngine, JsValue> {
        let github = GithubClient::new(token, owner, repo)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let wiki = WikiEngine::new()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let llm = Box::new(GenericLLMClient::new(llm_api_key, llm_base_url, llm_model));

        Ok(DeepTutorEngine {
            github,
            wiki,
            llm,
            session_id: chrono::Utc::now().to_rfc3339().replace(":", "-"),
            current_session_frames: Vec::new(),
        })
    }

    /// Build and return the knowledge graph for the entire wiki
    pub async fn get_graph_data(&self) -> Result<JsValue, JsValue> {
        let tree = self.github.get_recursive_tree("main").await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Filter for markdown files in the wiki directory
        let md_files: Vec<_> = tree.tree.into_iter()
            .filter(|e| e.path.starts_with("wiki/") && e.path.ends_with(".md"))
            .collect();

        // Fetch all contents in parallel
        let futures: Vec<_> = md_files.iter()
            .map(|e| self.github.get_content(&e.path))
            .collect();

        let results = join_all(futures).await;
        let mut pages = HashMap::new();

        for (i, res) in results.into_iter().enumerate() {
            if let Ok(content) = res {
                pages.insert(md_files[i].path.clone(), content.content.unwrap_or_default());
            }
        }

        let graph = self.wiki.build_graph(pages);
        serde_wasm_bindgen::to_value(&graph)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Fetch raw content of a specific wiki page
    pub async fn get_wiki_content(&self, path: String) -> Result<String, JsValue> {
        self.github.get_content(&path).await
            .map(|c| c.content.unwrap_or_default())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }


    pub async fn chat(&mut self, input: String) -> Result<String, JsValue> {
        let wiki_index = self.github.get_content("wiki/index.md").await
            .map(|c| c.content.unwrap_or_default())
            .unwrap_or_else(|_| "# Wiki Index\nNo pages found.".to_string());

        let user_profile = self.github.get_content("wiki/user_profile.md").await
            .map(|c| c.content.unwrap_or_default())
            .unwrap_or_else(|_| "New student profile.".to_string());

        let mut context = AgentContext {
            session_id: self.session_id.clone(),
            user_input: input,
            wiki_index: wiki_index.clone(),
            user_profile,
            metadata: HashMap::new(),
        };

        // --- REASONING LOOP ---
        let planner = PlannerAgent::new();
        let plan_res = planner.run(&context, self.llm.as_ref()).await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        context.metadata.insert("plan".to_string(), plan_res.content.clone());
        
        let strategy = PedagogyEngine::select_strategy(&context.user_input);
        context.metadata.insert("strategy".to_string(), format!("{:?}", strategy));

        let tutor = TutorAgent::new();
        let tutor_res = tutor.run(&context, self.llm.as_ref()).await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        context.metadata.insert("tutor_response".to_string(), tutor_res.content.clone());
        let critic = CriticAgent::new();
        let _critic_res = critic.run(&context, self.llm.as_ref()).await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let synthesizer = SynthesizerAgent::new();
        let synth_res = synthesizer.run(&context, self.llm.as_ref()).await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // --- ATOMIC PERSISTENCE ---
        let mut updates = HashMap::new();

        let frame = memvid_core::types::Frame {
            id: self.current_session_frames.len() as u64,
            timestamp: chrono::Utc::now().timestamp(),
            role: memvid_core::types::FrameRole::Assistant,
            metadata: context.metadata.clone(),
            ..Default::default()
        };
        self.current_session_frames.push(frame);

        let session_json = serde_json::to_string_pretty(&self.current_session_frames)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        updates.insert(format!("memvid/session-{}.json", self.session_id), session_json);

        let mut new_concepts = Vec::new();
        for update in synth_res.wiki_updates {
            updates.insert(update.path.clone(), update.content.clone());
            if update.path.contains("wiki/concepts/") {
                new_concepts.push(update.path);
            }
        }

        if !new_concepts.is_empty() {
            let mut updated_index = wiki_index.clone();
            for concept_path in new_concepts {
                let name = concept_path.split('/').last().unwrap_or("unknown").replace(".md", "");
                if !updated_index.contains(&name) {
                    updated_index.push_str(&format!("\n- [[{}]]", name));
                }
            }
            updates.insert("wiki/index.md".to_string(), updated_index);
        }

        let commit_msg = format!("DeepTutor Session Update: {}", self.session_id);
        let _ = self.github.atomic_batch_commit("main", &commit_msg, updates).await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(tutor_res.content)
    }
}
