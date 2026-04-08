use async_trait::async_trait;
use crate::{Agent, AgentContext, AgentResult, LLMClient};
use anyhow::Result;
use std::collections::HashMap;

pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Agent for PlannerAgent {
    fn name(&self) -> &'static str {
        "Planner"
    }

    async fn run(&self, context: &AgentContext, llm: &dyn LLMClient) -> Result<AgentResult> {
        let system_prompt = crate::prompts::PLANNER_SYSTEM_PROMPT
            .replace("{wiki_index}", &context.wiki_index)
            .replace("{user_profile}", &context.user_profile);

        let user_prompt = crate::prompts::PLANNER_USER_PROMPT
            .replace("{user_input}", &context.user_input);

        let plan_output = llm.chat_completion(&system_prompt, &user_prompt).await?;

        Ok(AgentResult {
            content: plan_output,
            wiki_updates: vec![],
            metadata: HashMap::from([
                ("agent".to_string(), "planner".to_string()),
                ("timestamp".to_string(), chrono::Utc::now().to_rfc3339()),
            ]),
        })
    }
}
