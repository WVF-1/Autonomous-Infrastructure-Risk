use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub time_steps: usize,
    pub initial_agents: usize,
    pub capacity_tiers: Vec<TierConfig>,
    pub policy: PolicyConfig,
    pub reporting: ReportingConfig,
    pub random_seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    pub name: String,
    pub max_capacity: usize,
    pub congestion_threshold: f64,
    pub failure_base_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub launch_rate: f64,
    pub mitigation_strength: f64,
    pub compliance_rate: f64,
    pub deorbit_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    pub report_interval: usize,
    pub hedging_enabled: bool,
    pub confidence_modulation: bool,
    pub ambiguity_level: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            time_steps: 1000,
            initial_agents: 500,
            capacity_tiers: vec![
                TierConfig {
                    name: "LEO-Low".to_string(),
                    max_capacity: 200,
                    congestion_threshold: 0.7,
                    failure_base_rate: 0.001,
                },
                TierConfig {
                    name: "LEO-Mid".to_string(),
                    max_capacity: 300,
                    congestion_threshold: 0.75,
                    failure_base_rate: 0.0008,
                },
                TierConfig {
                    name: "LEO-High".to_string(),
                    max_capacity: 250,
                    congestion_threshold: 0.8,
                    failure_base_rate: 0.0006,
                },
            ],
            policy: PolicyConfig {
                launch_rate: 0.02,
                mitigation_strength: 0.3,
                compliance_rate: 0.85,
                deorbit_threshold: 0.9,
            },
            reporting: ReportingConfig {
                report_interval: 10,
                hedging_enabled: true,
                confidence_modulation: true,
                ambiguity_level: 0.5,
            },
            random_seed: Some(42),
        }
    }
}

impl SimulationConfig {
    pub fn with_policy(mut self, policy: PolicyConfig) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_time_steps(mut self, steps: usize) -> Self {
        self.time_steps = steps;
        self
    }
}