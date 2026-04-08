use async_trait::async_trait;
use crate::{Agent, AgentContext, AgentResult, LLMClient};
use anyhow::Result;
use std::collections::HashMap;

pub struct TutorAgent;

impl TutorAgent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Agent for TutorAgent {
    fn name(&self) -> &'static str {
        "Tutor"
    }

    async fn run(&self, context: &AgentContext, llm: &dyn LLMClient) -> Result<AgentResult> {
        let strategy = context.metadata.get("strategy").cloned().unwrap_or_else(|| "DIRECT".to_string());
        let plan = context.metadata.get("plan").cloned().unwrap_or_else(|| "Provide a clear and helpful explanation.".to_string());

        let system_prompt = format!(
            "You are the Pedagogical Tutor Agent for Decentralized DeepTutor. \
            Your mission is to execute the following Teaching Strategy: {}\
            \nFollowing this Plan: {}\n\n\
            RESOURCES:\n- Wiki Knowledge: {}\n- Student History: {}\n\n\
            INSTRUCTIONS:\n\
            - If Socratic, ask one deep question instead of giving the answer. \
            - If Direct, use analogies and examples. \
            - Always cite related wiki concepts using [[Concept Name]] syntax. \
            - Keep your tone supportive yet intellectually challenging.",
            strategy, plan, context.wiki_index, context.user_profile
        );

        let user_prompt = format!("User Current Input: {}", context.user_input);

        let response = llm.chat_completion(&system_prompt, &user_prompt).await?;

        Ok(AgentResult {
            content: response,
            wiki_updates: vec![], // Synthesizer determines updates
            metadata: HashMap::from([("agent".to_string(), "tutor".to_string())]),
        })
    }
}
