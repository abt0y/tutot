use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeachingStrategy {
    Socratic,
    DirectInstruction,
    InquiryBased,
    ActiveRecall,
}

impl TeachingStrategy {
    pub fn description(&self) -> &'static str {
        match self {
            TeachingStrategy::Socratic => "Ask guiding questions to help the user discover answers.",
            TeachingStrategy::DirectInstruction => "Provide clear, step-by-step explanations of concepts.",
            TeachingStrategy::InquiryBased => "Present a task or question for the user to research and solve.",
            TeachingStrategy::ActiveRecall => "Prompt the user to retrieve and summarize information from memory.",
        }
    }
}

pub struct PedagogyEngine;

impl PedagogyEngine {
    pub fn select_strategy(input: &str) -> TeachingStrategy {
        // Simple logic for now, in a real system this would be more adaptive.
        if input.to_lowercase().contains("teach me") {
            TeachingStrategy::DirectInstruction
        } else if input.to_lowercase().contains("question") || input.to_lowercase().contains("quiz") {
            TeachingStrategy::ActiveRecall
        } else {
            TeachingStrategy::Socratic
        }
    }
}
