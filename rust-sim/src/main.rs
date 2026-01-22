use autonomous_infrastructure_risk::config::parameters::SimulationConfig;
use autonomous_infrastructure_risk::simulation::engine::SimulationEngine;
use autonomous_infrastructure_risk::utils::export::export_to_csv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Autonomous Infrastructure Risk Simulation");
    println!("================================================\n");

    // Load configuration
    let config = SimulationConfig::default();
    println!("Configuration loaded:");
    println!("  Time steps: {}", config.time_steps);
    println!("  Initial agents: {}", config.initial_agents);
    println!("  Capacity tiers: {}\n", config.capacity_tiers.len());

    // Initialize simulation
    println!("Initializing simulation engine...");
    let mut engine = SimulationEngine::new(config);

    // Run simulation
    println!("Running simulation...\n");
    let results = engine.run();

    println!("\nSimulation complete!");
    println!("  Total reports generated: {}", results.reports.len());
    println!("  Final agent count: {}", results.final_agent_count);
    println!("  Cascading failures: {}", results.cascading_failures);

    // Export results
    let output_path = "simulation_outputs.csv";
    println!("\nExporting to {}...", output_path);
    export_to_csv(&results, output_path)?;

    println!("Export complete!\n");
    println!("Next steps:");
    println!("  1. Open Jupyter Lab: jupyter lab");
    println!("  2. Navigate to notebooks/01_simulation_overview.ipynb");
    println!("  3. Run the analysis pipeline\n");

    Ok(())
}