use crate::planner::PlannerAgent;
use crate::tutor::TutorAgent;
use crate::critic::CriticAgent;
use crate::synthesizer::SynthesizerAgent;

pub fn get_agent(agent_name: &str) -> Option<Box<dyn Agent>> {
    match agent_name {
        "Planner" => Some(Box::new(PlannerAgent::new())),
        "Tutor" => Some(Box::new(TutorAgent::new())),
        "Critic" => Some(Box::new(CriticAgent::new())),
        "Synthesizer" => Some(Box::new(SynthesizerAgent::new())),
        _ => None,
    }
}