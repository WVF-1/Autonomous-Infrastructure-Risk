use crate::config::parameters::PolicyConfig;
use crate::models::{agent::Agent, capacity_tier::CapacityTier, event::Event};
use crate::utils::rng::SimulationRng;

pub struct PolicyEngine {
    config: PolicyConfig,
    next_agent_id: usize,
}

impl PolicyEngine {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            config,
            next_agent_id: 10000, // Start IDs high to avoid conflicts
        }
    }

    pub fn apply(
        &mut self,
        tiers: &mut [CapacityTier],
        agents: &mut Vec<Agent>,
        events: &mut Vec<Event>,
        timestamp: usize,
        rng: &mut SimulationRng,
    ) {
        // Launch new agents
        self.launch_agents(tiers, agents, events, timestamp, rng);

        // Apply mitigation
        self.apply_mitigation(tiers, agents, rng);

        // Deorbit agents in critical tiers
        self.deorbit_agents(tiers, agents, events, timestamp, rng);
    }

    fn launch_agents(
        &mut self,
        tiers: &mut [CapacityTier],
        agents: &mut Vec<Agent>,
        events: &mut Vec<Event>,
        timestamp: usize,
        rng: &mut SimulationRng,
    ) {
        if rng.gen_bool(self.config.launch_rate) {
            let tier_id = rng.gen_range(0..tiers.len());
            let compliance = rng.gen_bool(self.config.compliance_rate);
            
            let agent = Agent::new(self.next_agent_id, tier_id, compliance);
            self.next_agent_id += 1;
            
            tiers[tier_id].add_agent();
            events.push(Event::launch(timestamp, tier_id, agent.id));
            agents.push(agent);
        }
    }

    fn apply_mitigation(
        &self,
        _tiers: &[CapacityTier],
        agents: &mut [Agent],
        rng: &mut SimulationRng,
    ) {
        for agent in agents.iter_mut() {
            if agent.compliance && rng.gen_bool(self.config.mitigation_strength * 0.1) {
                // Mitigation slightly improves health
                agent.health = (agent.health + 0.01).min(1.0);
            }
        }
    }

    fn deorbit_agents(
        &mut self,
        tiers: &mut [CapacityTier],
        agents: &mut Vec<Agent>,
        events: &mut Vec<Event>,
        timestamp: usize,
        rng: &mut SimulationRng,
    ) {
        let mut to_deorbit = Vec::new();

        for tier in tiers.iter() {
            if tier.utilization() > self.config.deorbit_threshold {
                // Force deorbit in critical tiers
                for agent in agents.iter() {
                    if agent.tier_id == tier.id && rng.gen_bool(0.3) {
                        to_deorbit.push(agent.id);
                        break;
                    }
                }
            }
        }

        for agent_id in to_deorbit {
            if let Some(agent) = agents.iter_mut().find(|a| a.id == agent_id) {
                let tier_id = agent.tier_id;
                agent.health = 0.0;
                tiers[tier_id].remove_agent();
                events.push(Event::deorbit(timestamp, tier_id, agent_id));
            }
        }
    }
}