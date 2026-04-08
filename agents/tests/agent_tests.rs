use agents::{Agent, AgentContext, LLMClient, planner::PlannerAgent};
use async_trait::async_trait;
use anyhow::Result;
use std::collections::HashMap;

struct MockLLM;

#[async_trait]
impl LLMClient for MockLLM {
    async fn chat_completion(&self, _system: &str, _user: &str) -> Result<String> {
        Ok("Mocked response".to_string())
    }
}

#[tokio::test]
async fn test_planner_agent() {
    let agent = PlannerAgent::new();
    let context = AgentContext {
        session_id: "test".to_string(),
        user_input: "How does photosynthesis work?".to_string(),
        wiki_index: "Photosynthesis: [Link]".to_string(),
        user_profile: "New student".to_string(),
        metadata: HashMap::new(),
    };
    let llm = MockLLM;

    let result = agent.run(&context, &llm).await.unwrap();
    assert_eq!(result.content, "Mocked response");
    assert_eq!(result.metadata.get("agent").unwrap(), "planner");
}
