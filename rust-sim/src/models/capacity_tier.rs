use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityTier {
    pub id: usize,
    pub name: String,
    pub max_capacity: usize,
    pub current_count: usize,
    pub congestion_threshold: f64,
    pub failure_base_rate: f64,
}

impl CapacityTier {
    pub fn new(
        id: usize,
        name: String,
        max_capacity: usize,
        congestion_threshold: f64,
        failure_base_rate: f64,
    ) -> Self {
        Self {
            id,
            name,
            max_capacity,
            current_count: 0,
            congestion_threshold,
            failure_base_rate,
        }
    }

    pub fn utilization(&self) -> f64 {
        self.current_count as f64 / self.max_capacity as f64
    }

    pub fn is_congested(&self) -> bool {
        self.utilization() > self.congestion_threshold
    }

    pub fn congestion_severity(&self) -> f64 {
        if self.utilization() <= self.congestion_threshold {
            0.0
        } else {
            (self.utilization() - self.congestion_threshold)
                / (1.0 - self.congestion_threshold)
        }
    }

    pub fn effective_failure_rate(&self) -> f64 {
        let base = self.failure_base_rate;
        let utilization = self.utilization();
        
        // Exponential increase in failure rate as capacity is approached
        if utilization < self.congestion_threshold {
            base
        } else {
            let excess = utilization - self.congestion_threshold;
            base * (1.0 + 10.0 * excess.powi(2))
        }
    }

    pub fn add_agent(&mut self) {
        self.current_count += 1;
    }

    pub fn remove_agent(&mut self) {
        if self.current_count > 0 {
            self.current_count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilization() {
        let mut tier = CapacityTier::new(0, "Test".to_string(), 100, 0.7, 0.001);
        assert_eq!(tier.utilization(), 0.0);
        
        tier.current_count = 50;
        assert_eq!(tier.utilization(), 0.5);
    }

    #[test]
    fn test_congestion() {
        let mut tier = CapacityTier::new(0, "Test".to_string(), 100, 0.7, 0.001);
        tier.current_count = 60;
        assert!(!tier.is_congested());
        
        tier.current_count = 80;
        assert!(tier.is_congested());
    }
}