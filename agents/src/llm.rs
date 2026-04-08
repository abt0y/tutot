use async_trait::async_trait;
use crate::LLMClient;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: String,
}

pub struct GenericLLMClient {
    api_key: String,
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl GenericLLMClient {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            api_key,
            base_url,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLMClient for GenericLLMClient {
    async fn chat_completion(&self, system: &str, user: &str) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let request_body = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user.to_string(),
                },
            ],
        };

        let op = || async {
            let response = self.client.post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request_body)
                .send()
                .await
                .map_err(backoff::Error::transient)?;

            if response.status().is_server_error() || response.status().as_u16() == 429 {
                let error_text = response.text().await.unwrap_or_default();
                return Err(backoff::Error::transient(anyhow!("LLM API transient error: {} - {}", response.status(), error_text)));
            }

            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(backoff::Error::permanent(anyhow!("LLM API permanent error: {} - {}", response.status(), error_text)));
            }

            let body: ChatCompletionResponse = response.json().await.map_err(backoff::Error::permanent)?;
            
            body.choices.first()
                .map(|c| c.message.content.clone())
                .ok_or_else(|| backoff::Error::permanent(anyhow!("No response from LLM")))
        };

        backoff::future::retry(backoff::ExponentialBackoff::default(), op).await
    }
}
