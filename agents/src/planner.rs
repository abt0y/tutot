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
        let system_prompt = format!(
            "You are the Strategic Planner for Decentralized DeepTutor. \
            Your goal is to analyze the User Input vs. their User Profile and the Wiki Index to identify: \
            1. Knowledge Gaps: What does the user not know yet? \
            2. Misconceptions: What have they misunderstood in the past? \
            3. Contextual Relevance: Which wiki nodes are relevant to this request? \
             \
            Select a Teaching Strategy: \
            - SOCRATIC: Asking questions to lead the user to an answer. \
            - DIRECT: Providing a clear explanation of a new concept. \
            - RECAP: Summarizing recent learning. \
            - ASSESSMENT: Quizzing the user on a specific topic. \
            \n\nWiki Index:\n{}\n\nUser Profile:\n{}",
            context.wiki_index, context.user_profile
        );

        let user_prompt = format!(
            "User Input: {}\n\nDetermine the strategy and a step-by-step plan for the Tutor Agent. \
            Provide your reasoning clearly.",
            context.user_input
        );

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
