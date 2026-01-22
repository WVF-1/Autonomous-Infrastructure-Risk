use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Launch,
    Collision,
    Deorbit,
    CascadingFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: usize,
    pub event_type: EventType,
    pub tier_id: usize,
    pub agent_ids: Vec<usize>,
    pub severity: f64,
}

impl Event {
    pub fn launch(timestamp: usize, tier_id: usize, agent_id: usize) -> Self {
        Self {
            timestamp,
            event_type: EventType::Launch,
            tier_id,
            agent_ids: vec![agent_id],
            severity: 0.0,
        }
    }

    pub fn collision(
        timestamp: usize,
        tier_id: usize,
        agent_ids: Vec<usize>,
        severity: f64,
    ) -> Self {
        Self {
            timestamp,
            event_type: EventType::Collision,
            tier_id,
            agent_ids,
            severity,
        }
    }

    pub fn deorbit(timestamp: usize, tier_id: usize, agent_id: usize) -> Self {
        Self {
            timestamp,
            event_type: EventType::Deorbit,
            tier_id,
            agent_ids: vec![agent_id],
            severity: 0.0,
        }
    }

    pub fn cascading_failure(
        timestamp: usize,
        tier_id: usize,
        agent_ids: Vec<usize>,
        severity: f64,
    ) -> Self {
        Self {
            timestamp,
            event_type: EventType::CascadingFailure,
            tier_id,
            agent_ids,
            severity,
        }
    }
}