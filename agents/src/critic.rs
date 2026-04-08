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
        let system_prompt = format!(
            "You are the Grounding Critic for Decentralized DeepTutor. \
            Your goal is to analyze the Tutor's response against the Wiki Knowledge and User Profile. \
             \
            CHECKLIST: \
            1. Fact-check: Is the information consistent with the Wiki? \
            2. Misconceptions: Did the user express a misunderstanding that wasn't corrected? \
            3. Alignment: Does the response follow the Planner's strategy? \
             \
            Wiki Context:\n{}\n\nUser Profile:\n{}",
            context.wiki_index, context.user_profile
        );

        let user_prompt = format!(
            "Interaction to Critique:\nUser: {}\nTutor: {}\n\nEvaluate and provide brief feedback.",
            context.user_input,
            context.metadata.get("tutor_response").cloned().unwrap_or_default()
        );

        let critique = llm.chat_completion(&system_prompt, &user_prompt).await?;

        Ok(AgentResult {
            content: critique,
            wiki_updates: vec![],
            metadata: HashMap::from([("agent".to_string(), "critic".to_string())]),
        })
    }
}
