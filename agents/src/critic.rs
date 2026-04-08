use async_trait::async_trait;
use crate::{Agent, AgentContext, AgentResult, LLMClient};
use anyhow::Result;
use std::collections::HashMap;

pub struct CriticAgent;

impl CriticAgent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Agent for CriticAgent {
    fn name(&self) -> &'static str {
        "Critic"
    }

    async fn run(&self, context: &AgentContext, llm: &dyn LLMClient) -> Result<AgentResult> {
        let system_prompt = crate::prompts::CRITIC_SYSTEM_PROMPT
            .replace("{wiki_index}", &context.wiki_index)
            .replace("{user_profile}", &context.user_profile);

        let user_prompt = crate::prompts::CRITIC_USER_PROMPT
            .replace("{user_input}", &context.user_input)
            .replace("{tutor_response}", &context.metadata.get("tutor_response").cloned().unwrap_or_default());

        let critique = llm.chat_completion(&system_prompt, &user_prompt).await?;

        Ok(AgentResult {
            content: critique,
            wiki_updates: vec![],
            metadata: HashMap::from([("agent".to_string(), "critic".to_string())]),
        })
    }
}
