use crate::models::{agent::Agent, capacity_tier::CapacityTier, event::Event};
use crate::utils::rng::SimulationRng;

pub struct SystemDynamics;

impl SystemDynamics {
    pub fn new() -> Self {
        Self
    }

    pub fn update(
        &self,
        tiers: &mut [CapacityTier],
        agents: &mut [Agent],
        events: &mut Vec<Event>,
        timestamp: usize,
        rng: &mut SimulationRng,
    ) {
        // Age all agents
        for agent in agents.iter_mut() {
            agent.tick();
        }

        // Check for collisions in each tier
        for tier in tiers.iter() {
            if tier.is_congested() {
                self.check_collisions(tier, agents, events, timestamp, rng);
            }
        }

        // Check for cascading failures
        self.check_cascading_failures(tiers, agents, events, timestamp, rng);
    }

    fn check_collisions(
        &self,
        tier: &CapacityTier,
        agents: &mut [Agent],
        events: &mut Vec<Event>,
        timestamp: usize,
        rng: &mut SimulationRng,
    ) {
        let failure_rate = tier.effective_failure_rate();
        
        for agent in agents.iter_mut() {
            if agent.tier_id == tier.id && rng.gen_bool(failure_rate) {
                let damage = rng.gen_range(0.1..0.5);
                agent.apply_damage(damage);
                
                events.push(Event::collision(
                    timestamp,
                    tier.id,
                    vec![agent.id],
                    damage,
                ));
            }
        }
    }

    fn check_cascading_failures(
        &self,
        tiers: &[CapacityTier],
        agents: &mut [Agent],
        events: &mut Vec<Event>,
        timestamp: usize,
        rng: &mut SimulationRng,
    ) {
        for tier in tiers.iter() {
            let severity = tier.congestion_severity();
            
            // Cascading failures become likely at high congestion
            if severity > 0.8 && rng.gen_bool(severity * 0.05) {
                let affected: Vec<usize> = agents
                    .iter()
                    .filter(|a| a.tier_id == tier.id)
                    .take(5)
                    .map(|a| a.id)
                    .collect();

                for agent in agents.iter_mut() {
                    if affected.contains(&agent.id) {
                        agent.apply_damage(rng.gen_range(0.3..0.8));
                    }
                }

                events.push(Event::cascading_failure(
                    timestamp,
                    tier.id,
                    affected,
                    severity,
                ));
            }
        }
    }
}