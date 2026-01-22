use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: usize,
    pub tier_id: usize,
    pub age: usize,
    pub compliance: bool,
    pub health: f64,
}

impl Agent {
    pub fn new(id: usize, tier_id: usize, compliance: bool) -> Self {
        Self {
            id,
            tier_id,
            age: 0,
            compliance,
            health: 1.0,
        }
    }

    pub fn tick(&mut self) {
        self.age += 1;
        // Health degrades slightly over time
        self.health *= 0.9999;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.01
    }

    pub fn apply_damage(&mut self, damage: f64) {
        self.health -= damage;
        if self.health < 0.0 {
            self.health = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new(1, 0, true);
        assert_eq!(agent.id, 1);
        assert_eq!(agent.tier_id, 0);
        assert!(agent.is_alive());
    }

    #[test]
    fn test_agent_damage() {
        let mut agent = Agent::new(1, 0, true);
        agent.apply_damage(0.5);
        assert_eq!(agent.health, 0.5);
        assert!(agent.is_alive());
        
        agent.apply_damage(0.6);
        assert!(!agent.is_alive());
    }
}