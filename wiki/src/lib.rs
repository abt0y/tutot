use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, html};
use regex::Regex;
use anyhow::{Result, anyhow};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WikiPage {
    pub title: String,
    pub content: String,
    pub path: String,
    pub links: HashSet<String>,
    pub backlinks: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiGraph {
    pub nodes: Vec<WikiNode>,
    pub links: Vec<WikiLink>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiNode {
    pub id: String,
    pub title: String,
    pub group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiLink {
    pub source: String,
    pub target: String,
    pub value: u32,
}

pub struct WikiEngine {
    link_regex: Regex,
}

impl WikiEngine {
    pub fn new() -> Result<Self> {
        let link_regex = Regex::new(r"\[\[(.*?)\]\]")?;
        Ok(WikiEngine { link_regex })
    }

    pub fn parse_markdown(&self, markdown: &str) -> String {
        let parser = Parser::new(markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    pub fn extract_wiki_links(&self, markdown: &str) -> HashSet<String> {
        let mut links = HashSet::new();
        for cap in self.link_regex.captures_iter(markdown) {
            if let Some(link) = cap.get(1) {
                links.insert(link.as_str().to_string());
            }
        }
        links
    }

    /// Build a graph from a collection of paths and their markdown contents
    pub fn build_graph(&self, pages: HashMap<String, String>) -> WikiGraph {
        let mut nodes = Vec::new();
        let mut links = Vec::new();
        let mut page_links_map = HashMap::new();

        for (path, content) in &pages {
            let title = path.split('/').last().unwrap_or(path).replace(".md", "");
            nodes.push(WikiNode {
                id: path.clone(),
                title: title.clone(),
                group: if path.contains("concepts") { "concept".to_string() } else { "standard".to_string() },
            });

            let out_links = self.extract_wiki_links(content);
            page_links_map.insert(path.clone(), out_links);
        }

        // Second pass to create links directed at paths
        for (source_path, target_names) in page_links_map {
            for target_name in target_names {
                // Try to find the path that matches this name
                if let Some(target_path) = pages.keys().find(|p| p.to_lowercase().contains(&target_name.to_lowercase())) {
                    links.push(WikiLink {
                        source: source_path.clone(),
                        target: target_path.clone(),
                        value: 1,
                    });
                }
            }
        }

        WikiGraph { nodes, links }
    }

    pub fn generate_page_stub(&self, title: &str, content: &str) -> String {
        format!(
            "---\ntitle: {}\nlast_updated: {}\ntags: []\ncitations: []\n---\n\n# {}\n\n{}",
            title, chrono::Utc::now().to_rfc3339(), title, content
        )
    }
}
