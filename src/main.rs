//! Main binary: entry point for synthetic consciousness simulation.
//!
//! This program runs the Geometric Consciousness Model simulation with optional
//! real-time visualization. It loads configuration from TOML files or uses defaults,
//! executes the simulation, and generates analysis reports.
//!
//! ## Usage
//!
//! ```bash
//! # Run with default 2D configuration
//! cargo run --release
//!
//! # Run with custom configuration
//! cargo run --release -- config.toml
//!
//! # Run with real-time visualization
//! cargo run --release -- config.toml --visualize
//! cargo run --release -- -v
//! ```
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use synthetic_consciousness::config::SimulationConfig;
use synthetic_consciousness::simulation::Simulation;
use synthetic_consciousness::visualization::{VisualizationState, launch_visualization};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check for visualization flag
    let visualize = args.contains(&"--visualize".to_string()) || args.contains(&"-v".to_string());
    
    // Get config file path (skip --visualize flag)
    let config_path = args.iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-'));

    let config = if let Some(path) = config_path {
        // Load configuration from file
        match SimulationConfig::from_toml(path) {
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
    if visualize {
        println!("Visualization: ENABLED");
    }
    println!();

    // Create and run simulation
    match Simulation::new(config.clone()) {
        Ok(sim) => {
            if visualize {
                run_with_visualization(sim, config);
            } else {
                run_without_visualization(sim);
            }
        }
        Err(e) => {
            eprintln!("Error creating simulation: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_with_visualization(mut sim: Simulation, config: SimulationConfig) {
    // Create shared visualization state
    let viz_state = Arc::new(Mutex::new(VisualizationState {
        dimension: config.geometry.dimension,
        ..Default::default()
    }));
    
    let viz_state_clone = Arc::clone(&viz_state);
    
    // Run simulation in separate thread
    let sim_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(500)); // Let GUI initialize first
        
        println!("Starting simulation with real-time visualization...");
        
        // Run simulation with visualization updates
        for step in 0..config.simulation.num_steps {
            sim.step();
            
            // Update visualization every N steps
            if step % 10 == 0 {
                sim.update_visualization(&viz_state_clone);
                thread::sleep(Duration::from_millis(10)); // Slow down for visibility
            }
            
            if step % 100 == 0 {
                println!("Step {}/{}", step, config.simulation.num_steps);
            }
        }
        
        // Final update
        sim.update_visualization(&viz_state_clone);
        
        // Finalize results
        sim.finalize_results();
        
        println!();
        println!("Simulation complete! Visualization window will remain open.");
        println!("Close the window to exit...");
        println!();
        
        sim
    });
    
    // Launch visualization on main thread (required for macOS)
    if let Err(e) = launch_visualization(viz_state) {
        eprintln!("Visualization error: {}", e);
    }
    
    // Wait for simulation to complete
    if let Ok(sim) = sim_thread.join() {
        print_final_results(&sim);
    }
}

fn run_without_visualization(mut sim: Simulation) {
    println!("Starting simulation...");
    sim.run();
    
    // Finalize results and analyze consciousness
    sim.finalize_results();
    
    println!("Simulation complete!");
    println!();
    
    print_final_results(&sim);
}

fn print_final_results(sim: &Simulation) {
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
