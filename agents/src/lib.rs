use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

pub mod planner;
pub mod tutor;
pub mod critic;
pub mod synthesizer;
pub mod llm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub session_id: String,
    pub user_input: String,
    pub wiki_index: String, // Path or content of index.md
    pub user_profile: String, // Content of user_profile.md
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub content: String,
    pub wiki_updates: Vec<WikiUpdate>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiUpdate {
    pub path: String,
    pub content: String,
    pub action: WikiAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WikiAction {
    Create,
    Update,
    Append,
}

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run(&self, context: &AgentContext, llm: &dyn LLMClient) -> Result<AgentResult>;
}

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn chat_completion(&self, system: &str, user: &str) -> Result<String>;
}
