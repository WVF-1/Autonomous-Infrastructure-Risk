pub struct TemplateBank;

impl TemplateBank {
    pub fn new() -> Self {
        Self
    }

    pub fn stable_opening(&self, utilization: f64, hedge: bool) -> String {
        let base = format!(
            "Operational status nominal. System utilization at {:.1}%.",
            utilization * 100.0
        );
        
        if hedge && utilization > 0.5 {
            format!("{} Capacity reserves adequate.", base)
        } else {
            base
        }
    }

    pub fn degrading_opening(&self, utilization: f64, hedge: bool) -> String {
        let base = format!(
            "System experiencing elevated utilization at {:.1}%.",
            utilization * 100.0
        );
        
        if hedge {
            format!("{} Trend monitoring ongoing.", base)
        } else {
            base
        }
    }

    pub fn critical_opening(&self, utilization: f64, hedge: bool) -> String {
        let base = format!(
            "Critical utilization detected at {:.1}%.",
            utilization * 100.0
        );
        
        if hedge {
            format!("{} Immediate attention required, though some uncertainty remains in projections.", base)
        } else {
            format!("{} Immediate attention required.", base)
        }
    }
}