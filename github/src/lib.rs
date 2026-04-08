//! GitHub API Integration for DeepTutor
//! 
//! This module provides the bridge between the agent system and GitHub,
//! handling all content operations with proper rate limiting, error handling,
//! and conflict resolution.

use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use std::collections::HashMap;

/// GitHub content response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct GithubContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub content_type: String,
    pub content: Option<String>, // Base64 encoded string
    pub encoding: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitBlobResponse {
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitTreeEntry {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitTreeResponse {
    pub sha: String,
    pub tree: Vec<GitTreeEntry>,
    pub truncated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommitResponse {
    pub sha: String,
    pub tree: GitTreeResponseStub,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitTreeResponseStub {
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitRefResponse {
    r#ref: String,
    object: GitRefObject,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitRefObject {
    sha: String,
    #[serde(rename = "type")]
    r#type: String,
    url: String,
}

/// GitHub API client with WASM support
pub struct GithubClient {
    token: String,
    owner: String,
    repo: String,
    client: reqwest::Client,
}

impl GithubClient {
    /// Create a new GitHub client with authentication
    pub fn new(token: String, owner: String, repo: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION, 
            HeaderValue::from_str(&format!("token {}", token))?
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("DeepTutor-WASM"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(GithubClient { token, owner, repo, client })
    }

    /// Retrieve content from a file in the repository
    pub async fn get_content(&self, path: &str) -> Result<GithubContent> {
        let url = format!("https://api.github.com/repos/{}/{}/contents/{}", 
            self.owner, self.repo, path);
        let resp = self.client.get(&url).send().await?;
        
        if resp.status() == 404 {
            return Err(anyhow!("File not found: {}", path));
        }
        if !resp.status().is_success() {
            return Err(anyhow!("GitHub API error: {}", resp.status()));
        }

        let content: GithubContent = resp.json().await?;
        
        // Decode base64 content
        if let Some(encoded) = &content.content {
            use base64::engine::general_purpose::STANDARD;
            let decoded = STANDARD.decode(encoded.replace("\n", ""))
                .map_err(|e| anyhow!("Base64 decode error: {}", e))?;
            let content_str = String::from_utf8(decoded)
                .map_err(|_| anyhow!("Content is not valid UTF-8"))?;
            
            Ok(GithubContent {
                content: Some(content_str),
                ..content
            })
        } else {
            Ok(content)
        }
    }

    /// Create or update a file in the repository (non-atomic)
    pub async fn upsert_content(&self, path: &str, message: &str, content: &str, sha: Option<String>) -> Result<()> {
        let url = format!("https://api.github.com/repos/{}/{}/contents/{}", 
            self.owner, self.repo, path);
        
        use base64::engine::general_purpose::STANDARD;
        let encoded = STANDARD.encode(content);
        
        let body = serde_json::json!({
            "message": message,
            "content": encoded,
            "sha": sha,
        });

        let resp = self.client.put(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("GitHub API error: {} - {}", resp.status(), error_body));
        }
        Ok(())
    }

    /// Fetch all files in the repository recursively
    pub async fn get_recursive_tree(&self, branch: &str) -> Result<GitTreeResponse> {
        // 1. Get branch refinement to find tree SHA
        let ref_url = format!("https://api.github.com/repos/{}/{}/git/refs/heads/{}", self.owner, self.repo, branch);
        let ref_resp: GitRefResponse = self.client.get(&ref_url).send().await?.json().await?;
        let commit_sha = ref_resp.object.sha;

        // 2. Get the commit tree
        let commit_url = format!("https://api.github.com/repos/{}/{}/git/commits/{}", self.owner, self.repo, commit_sha);
        let commit_resp: GitCommitResponse = self.client.get(&commit_url).send().await?.json().await?;
        let tree_sha = commit_resp.tree.sha;

        // 3. Fetch tree with recursive=1
        let tree_url = format!("https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1", self.owner, self.repo, tree_sha);
        let tree_resp: GitTreeResponse = self.client.get(&tree_url).send().await?.json().await?;
        
        Ok(tree_resp)
    }

    /// True Atomic Batch Commit using Git Data API
    pub async fn atomic_batch_commit(&self, branch: &str, message: &str, updates: HashMap<String, String>) -> Result<String> {
        let ref_url = format!("https://api.github.com/repos/{}/{}/git/refs/heads/{}", self.owner, self.repo, branch);
        let ref_resp: GitRefResponse = self.client.get(&ref_url).send().await?.json().await?;
        let base_commit_sha = ref_resp.object.sha;

        let commit_url = format!("https://api.github.com/repos/{}/{}/git/commits/{}", self.owner, self.repo, base_commit_sha);
        let commit_resp: GitCommitResponse = self.client.get(&commit_url).send().await?.json().await?;
        let base_tree_sha = commit_resp.tree.sha;

        let mut tree_entries = Vec::new();
        for (path, content) in updates {
            let blob_url = format!("https://api.github.com/repos/{}/{}/git/blobs", self.owner, self.repo);
            let blob_body = serde_json::json!({
                "content": content,
                "encoding": "utf-8"
            });
            let blob_resp: GitBlobResponse = self.client.post(&blob_url).json(&blob_body).send().await?.json().await?;
            
            tree_entries.push(GitTreeEntry {
                path,
                mode: "100644".to_string(),
                r#type: "blob".to_string(),
                sha: blob_resp.sha,
            });
        }

        let tree_url = format!("https://api.github.com/repos/{}/{}/git/trees", self.owner, self.repo);
        let tree_body = serde_json::json!({
            "base_tree": base_tree_sha,
            "tree": tree_entries
        });
        let tree_resp_atomic: GitTreeResponseStub = self.client.post(&tree_url).json(&tree_body).send().await?.json().await?;

        let commit_create_url = format!("https://api.github.com/repos/{}/{}/git/commits", self.owner, self.repo);
        let commit_create_body = serde_json::json!({
            "message": message,
            "tree": tree_resp_atomic.sha,
            "parents": [base_commit_sha]
        });
        let commit_create_resp: GitCommitResponse = self.client.post(&commit_create_url).json(&commit_create_body).send().await?.json().await?;

        let ref_update_body = serde_json::json!({
            "sha": commit_create_resp.sha,
            "force": false
        });
        let patch_resp = self.client.patch(&ref_url).json(&ref_update_body).send().await?;
        
        if !patch_resp.status().is_success() {
            return Err(anyhow!("Failed to update ref: {}", patch_resp.status()));
        }

        Ok(commit_create_resp.sha)
    }
}