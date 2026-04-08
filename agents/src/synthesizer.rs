use async_trait::async_trait;
use crate::{Agent, AgentContext, AgentResult, LLMClient, WikiUpdate, WikiAction};
use anyhow::Result;
use std::collections::HashMap;

pub struct SynthesizerAgent;

impl SynthesizerAgent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Agent for SynthesizerAgent {
    fn name(&self) -> &'static str {
        "Synthesizer"
    }

    async fn run(&self, context: &AgentContext, llm: &dyn LLMClient) -> Result<AgentResult> {
        let system_prompt = format!(
            "You are the Knowledge Synthesizer for Decentralized DeepTutor. \
            Your goal is to formalize the current learning interaction into a structured wiki update. \
             \
            INSTRUCTIONS: \
            1. Extract the core concept learned or the primary misconception identified. \
            2. Format the output as a Markdown file with YAML frontmatter. \
            3. Frontmatter fields: title, tags, citations (wiki links), last_updated. \
            \nWiki Context:\n{}\n\nUser Profile:\n{}",
            context.wiki_index, context.user_profile
        );

        let user_prompt = format!(
            "Interaction to Synthesize:\nUser Input: {}\n\nDecide if we should create a new concept page or update the user_profile.md. \
            Output the full markdown content.",
            context.user_input
        );

        let synthesis = llm.chat_completion(&system_prompt, &user_prompt).await?;

        // Extract a title for the filename (naive approach for now)
        let first_line = synthesis.lines().next().unwrap_or("untitled");
        let filename = first_line.to_lowercase().replace("# ", "").replace(" ", "_").chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>();
        let path = if synthesis.to_lowercase().contains("profile") {
            "wiki/user_profile.md".to_string()
        } else {
            format!("wiki/concepts/{}.md", if filename.is_empty() { "new_concept" } else { &filename })
        };

        let action = if path == "wiki/user_profile.md" { WikiAction::Update } else { WikiAction::Create };

        Ok(AgentResult {
            content: synthesis.clone(),
            wiki_updates: vec![WikiUpdate {
                path,
                content: synthesis,
                action,
            }],
            metadata: HashMap::from([("agent".to_string(), "synthesizer".to_string())]),
        })
    }
}
