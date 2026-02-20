//! Main binary: entry point for synthetic consciousness simulation.

use synthetic_consciousness::config::SimulationConfig;
use synthetic_consciousness::simulation::Simulation;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = if args.len() > 1 {
        // Load configuration from file
        match SimulationConfig::from_toml(&args[1]) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Error loading config: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Use default 2D configuration
        println!("No config specified, using default 2D configuration");
        SimulationConfig::default_2d()
    };

    println!("===== Synthetic Consciousness Simulation =====");
    println!("Name: {}", config.metadata.name);
    println!("Description: {}", config.metadata.description);
    println!("Dimensionality: {}D", config.geometry.dimension);
    println!("Entities: {}", config.simulation.num_entities);
    println!("Steps: {}", config.simulation.num_steps);
    println!();

    // Create and run simulation
    match Simulation::new(config.clone()) {
        Ok(mut sim) => {
            println!("Starting simulation...");
            sim.run();
            
            // Finalize results and analyze consciousness
            sim.finalize_results();
            
            println!("Simulation complete!");
            println!();

            // Print final metrics
            if let Some(final_metrics) = sim.metrics_history.last() {
                println!("===== Final Metrics =====");
                println!("Attention Entropy: {:.4}", final_metrics.attention_entropy);
                println!("Memory Diversity: {:.4}", final_metrics.memory_diversity);
                println!("Velocity Stability: {:.4}", final_metrics.velocity_stability);
                println!("Identity Coherence: {:.4}", final_metrics.identity_coherence);
                println!("Cluster Stability: {:.4}", final_metrics.cluster_stability);
                println!("Affective Strength: {:.4}", final_metrics.affective_strength);
                println!("Average Essence: {:.4}", final_metrics.average_essence);
                println!();
            }

            // Export metrics
            match sim.export_metrics_csv("metrics.csv") {
                Ok(_) => println!("Metrics exported to metrics.csv"),
                Err(e) => eprintln!("Error exporting metrics: {}", e),
            }
            
            // Generate detailed report
            match sim.generate_report("simulation") {
                Ok(_) => {},
                Err(e) => eprintln!("Error generating report: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error creating simulation: {}", e);
            std::process::exit(1);
        }
    }
}
