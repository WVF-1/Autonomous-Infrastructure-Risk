use crate::config::parameters::ReportingConfig;
use crate::models::capacity_tier::CapacityTier;
use crate::reporting::templates::TemplateBank;

pub struct LanguageGenerator {
    config: ReportingConfig,
    templates: TemplateBank,
}

impl LanguageGenerator {
    pub fn new(config: ReportingConfig) -> Self {
        Self {
            config,
            templates: TemplateBank::new(),
        }
    }

    pub fn generate_report(
        &self,
        tiers: &[CapacityTier],
        avg_utilization: f64,
        avg_risk: f64,
        recent_events: usize,
        stability_class: &str,
    ) -> String {
        let mut parts = Vec::new();

        // Opening statement with risk-dependent hedging
        parts.push(self.generate_opening(stability_class, avg_utilization));

        // Tier-specific observations
        parts.push(self.generate_tier_summary(tiers));

        // Event summary with confidence modulation
        parts.push(self.generate_event_summary(recent_events, avg_risk));

        // Closing with risk-dependent language
        parts.push(self.generate_closing(stability_class, avg_utilization));

        parts.join(" ")
    }

    fn generate_opening(&self, stability_class: &str, utilization: f64) -> String {
        match stability_class {
            "Stable" => self.templates.stable_opening(utilization, self.config.hedging_enabled),
            "Degrading" => self.templates.degrading_opening(utilization, self.config.hedging_enabled),
            "Critical" => self.templates.critical_opening(utilization, self.config.hedging_enabled),
            _ => "Operational status nominal.".to_string(),
        }
    }

    fn generate_tier_summary(&self, tiers: &[CapacityTier]) -> String {
        let congested_count = tiers.iter().filter(|t| t.is_congested()).count();
        
        if congested_count == 0 {
            "All capacity tiers operating within normal parameters.".to_string()
        } else if congested_count == 1 {
            let tier = tiers.iter().find(|t| t.is_congested()).unwrap();
            format!(
                "{} tier experiencing elevated utilization at {:.1}% of capacity.",
                tier.name,
                tier.utilization() * 100.0
            )
        } else {
            format!(
                "Multiple capacity tiers ({}) showing congestion patterns.",
                congested_count
            )
        }
    }

    fn generate_event_summary(&self, recent_events: usize, risk: f64) -> String {
        let hedge = if self.config.hedging_enabled && risk > 0.01 {
            "though elevated risk persists"
        } else {
            "with risk levels stable"
        };

        if recent_events == 0 {
            format!("No significant conjunction events detected, {}.", hedge)
        } else if recent_events < 5 {
            format!(
                "Limited conjunction activity observed ({} events), {}.",
                recent_events, hedge
            )
        } else {
            format!(
                "Increased conjunction activity detected ({} events), {}.",
                recent_events, hedge
            )
        }
    }

    fn generate_closing(&self, stability_class: &str, utilization: f64) -> String {
        if stability_class == "Critical" {
            if self.config.confidence_modulation {
                "Mitigation protocols engaged. Continued monitoring advised.".to_string()
            } else {
                "Mitigation protocols active.".to_string()
            }
        } else if utilization > 0.7 {
            "Debris mitigation protocols remain nominal.".to_string()
        } else {
            "All systems nominal.".to_string()
        }
    }
}