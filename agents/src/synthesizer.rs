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
        let system_prompt = crate::prompts::SYNTHESIZER_SYSTEM_PROMPT
            .replace("{wiki_index}", &context.wiki_index)
            .replace("{user_profile}", &context.user_profile);

        let user_prompt = crate::prompts::SYNTHESIZER_USER_PROMPT
            .replace("{user_input}", &context.user_input);

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
