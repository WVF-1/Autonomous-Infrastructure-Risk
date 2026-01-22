use crate::config::parameters::SimulationConfig;
use crate::models::{agent::Agent, capacity_tier::CapacityTier, event::Event};
use crate::reporting::language::LanguageGenerator;
use crate::simulation::dynamics::SystemDynamics;
use crate::simulation::policy::PolicyEngine;
use crate::utils::rng::SimulationRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReport {
    pub timestamp: usize,
    pub report_text: String,
    pub ground_truth_utilization: f64,
    pub ground_truth_risk: f64,
    pub agent_count: usize,
    pub stability_class: String,
    pub recent_events: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationResults {
    pub reports: Vec<StatusReport>,
    pub events: Vec<Event>,
    pub final_agent_count: usize,
    pub cascading_failures: usize,
}

pub struct SimulationEngine {
    config: SimulationConfig,
    tiers: Vec<CapacityTier>,
    agents: Vec<Agent>,
    events: Vec<Event>,
    dynamics: SystemDynamics,
    policy: PolicyEngine,
    language: LanguageGenerator,
    rng: SimulationRng,
    current_time: usize,
}

impl SimulationEngine {
    pub fn new(config: SimulationConfig) -> Self {
        let mut rng = SimulationRng::new(config.random_seed);

        // Initialize capacity tiers
        let tiers: Vec<CapacityTier> = config
            .capacity_tiers
            .iter()
            .enumerate()
            .map(|(i, tc)| {
                CapacityTier::new(
                    i,
                    tc.name.clone(),
                    tc.max_capacity,
                    tc.congestion_threshold,
                    tc.failure_base_rate,
                )
            })
            .collect();

        // Initialize agents
        let mut agents = Vec::new();
        for i in 0..config.initial_agents {
            let tier_id = i % tiers.len();
            let compliance = rng.gen_bool(config.policy.compliance_rate);
            agents.push(Agent::new(i, tier_id, compliance));
        }

        // Update tier counts
        let mut tiers = tiers;
        for agent in &agents {
            tiers[agent.tier_id].add_agent();
        }

        Self {
            dynamics: SystemDynamics::new(),
            policy: PolicyEngine::new(config.policy.clone()),
            language: LanguageGenerator::new(config.reporting.clone()),
            rng,
            config,
            tiers,
            agents,
            events: Vec::new(),
            current_time: 0,
        }
    }

    pub fn run(&mut self) -> SimulationResults {
        let mut reports = Vec::new();

        for t in 0..self.config.time_steps {
            self.current_time = t;

            // Simulate dynamics
            self.dynamics.update(&mut self.tiers, &mut self.agents, &mut self.events, t, &mut self.rng);

            // Apply policy
            self.policy.apply(&mut self.tiers, &mut self.agents, &mut self.events, t, &mut self.rng);

            // Generate reports
            if t % self.config.reporting.report_interval == 0 {
                let report = self.generate_report(t);
                reports.push(report);
            }

            // Cleanup dead agents
            self.cleanup_agents();
        }

        let cascading_failures = self.events.iter()
            .filter(|e| matches!(e.event_type, crate::models::event::EventType::CascadingFailure))
            .count();

        SimulationResults {
            reports,
            events: self.events.clone(),
            final_agent_count: self.agents.len(),
            cascading_failures,
        }
    }

    fn generate_report(&self, timestamp: usize) -> StatusReport {
        let avg_util = self.tiers.iter().map(|t| t.utilization()).sum::<f64>() / self.tiers.len() as f64;
        let avg_risk = self.tiers.iter().map(|t| t.effective_failure_rate()).sum::<f64>() / self.tiers.len() as f64;
        
        let stability_class = if avg_util < 0.6 {
            "Stable"
        } else if avg_util < 0.85 {
            "Degrading"
        } else {
            "Critical"
        }.to_string();

        let recent_events = self.events.iter()
            .filter(|e| e.timestamp >= timestamp.saturating_sub(self.config.reporting.report_interval))
            .count();

        let report_text = self.language.generate_report(
            &self.tiers,
            avg_util,
            avg_risk,
            recent_events,
            &stability_class,
        );

        StatusReport {
            timestamp,
            report_text,
            ground_truth_utilization: avg_util,
            ground_truth_risk: avg_risk,
            agent_count: self.agents.len(),
            stability_class,
            recent_events,
        }
    }

    fn cleanup_agents(&mut self) {
        let initial_count = self.agents.len();
        self.agents.retain(|a| a.is_alive());
        let removed = initial_count - self.agents.len();
        
        for _ in 0..removed {
            for tier in &mut self.tiers {
                tier.remove_agent();
            }
        }
    }
}