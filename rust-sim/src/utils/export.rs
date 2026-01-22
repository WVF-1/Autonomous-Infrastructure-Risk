use crate::simulation::engine::SimulationResults;
use csv::Writer;
use std::error::Error;

pub fn export_to_csv(results: &SimulationResults, path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    // Write header
    wtr.write_record(&[
        "timestamp",
        "report_text",
        "ground_truth_utilization",
        "ground_truth_risk",
        "agent_count",
        "stability_class",
        "recent_events",
    ])?;

    // Write data
    for report in &results.reports {
        wtr.write_record(&[
            report.timestamp.to_string(),
            report.report_text.clone(),
            format!("{:.6}", report.ground_truth_utilization),
            format!("{:.6}", report.ground_truth_risk),
            report.agent_count.to_string(),
            report.stability_class.clone(),
            report.recent_events.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}