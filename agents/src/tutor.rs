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

        let system_prompt = crate::prompts::TUTOR_SYSTEM_PROMPT
            .replace("{strategy}", &strategy)
            .replace("{plan}", &plan)
            .replace("{wiki_index}", &context.wiki_index)
            .replace("{user_profile}", &context.user_profile);

        let user_prompt = crate::prompts::TUTOR_USER_PROMPT
            .replace("{user_input}", &context.user_input);

        let response = llm.chat_completion(&system_prompt, &user_prompt).await?;

        Ok(AgentResult {
            content: response,
            wiki_updates: vec![], // Synthesizer determines updates
            metadata: HashMap::from([("agent".to_string(), "tutor".to_string())]),
        })
    }
}
