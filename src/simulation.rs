//! Simulation module: main orchestration and stepping.
//!
//! This module provides the core simulation loop that orchestrates all components:
//! - Entity initialization in geometric space
//! - Step-by-step time evolution
//! - Attention/attraction computation
//! - Memory updates and clustering
//! - Dynamics integration
//! - Metrics evaluation
//! - Results tracking and reporting
//!
//! ## Simulation Loop
//!
//! Each step:
//! 1. Compute pairwise attractions between entities
//! 2. Calculate attention gradients
//! 3. Generate stimuli and update entity states
//! 4. Integrate motion with perpetual velocity
//! 5. Update memory graphs and belief clusters
//! 6. Compute consciousness metrics
//! 7. Record step data for analysis
//!
//! ## Visualization Integration
//!
//! When visualization is enabled, the simulation periodically updates a shared
//! state structure that the GUI thread reads for real-time rendering.
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use crate::config::SimulationConfig;
use crate::entities::{Entity, EntityId, EntityPool};
use crate::geometry::Pose;
use crate::state::EntityStateVector;
use crate::memory::MemoryGraph;
use crate::essence::EssenceIndex;
use crate::metrics::Metrics;
use crate::results::{SimulationResults, SimulationStep};
use rand::Rng;
use chrono::Local;

/// Main simulation instance.
pub struct Simulation {
    pub config: SimulationConfig,
    pub entities: EntityPool,
    pub timestamp: u64,
    pub metrics_history: Vec<Metrics>,
    pub results: SimulationResults,
}

impl Simulation {
    /// Create new simulation with configuration.
    pub fn new(config: SimulationConfig) -> Result<Self, String> {
        config.validate()?;

        let start_time = Local::now().to_rfc3339();

        let mut sim = Simulation {
            config,
            entities: EntityPool::new(),
            timestamp: 0,
            metrics_history: Vec::new(),
            results: SimulationResults::new(
                "Synthetic Consciousness Simulation".to_string(),
                0,
                0,
                start_time,
            ),
        };

        // Initialize entities
        sim.initialize_entities()?;
        sim.results.num_entities = sim.config.simulation.num_entities;
        sim.results.num_steps = sim.config.simulation.num_steps;

        Ok(sim)
    }

    /// Initialize entities with random positions.
    fn initialize_entities(&mut self) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        let dim = self.config.geometry.dimension;
        let n = self.config.simulation.num_entities;
        let bounds = &self.config.geometry.bounds;

        for _i in 0..n {
            let mut position = vec![0.0; dim];
            for d in 0..dim {
                let bound = bounds[d];
                position[d] = rng.gen_range(0.0..bound);
            }

            let mut orientation = [1.0, 0.0, 0.0, 0.0];
            orientation[1] = rng.gen_range(-1.0..1.0);

            let pose = Pose {
                position,
                orientation,
            };

            let state = EntityStateVector::new(self.config.state.clone());

            let memory_graph = MemoryGraph::new();

            let essence = EssenceIndex::new(self.config.essence.clone());

            let entity = Entity::new(EntityId(0), pose, state, memory_graph, essence);
            self.entities.add_entity(entity);
        }

        Ok(())
    }

    /// Execute one simulation step.
    pub fn step(&mut self) {
        // Step 1: Sense environment (input stimulus)
        self.sense_step();

        // Step 2: Compute attention fields
        self.attention_step();

        // Step 3: Update state vectors
        self.state_update_step();

        // Step 4: Compute affective signals
        self.affective_step();

        // Step 5: Update essence indices
        self.essence_step();

        // Step 6: Decide on actions
        self.decision_step();

        // Step 7: Integrate dynamics (perpetual velocity)
        self.integration_step();

        // Step 8: Apply periodic boundaries
        self.boundary_step();

        // Step 9: Update memory decay
        self.memory_decay_step();

        // Step 10: Compute metrics
        self.metrics_step();

        self.timestamp += 1;
    }

    /// Sensing: receive input stimulus
    fn sense_step(&mut self) {
        let mut rng = rand::thread_rng();
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            let dim = entity.pose.position.len();
            let stimulus: Vec<f32> = (0..dim)
                .map(|_| rng.gen_range(-0.1..0.1))
                .collect();

            entity.sense(stimulus, self.timestamp);
        }
    }

    /// Attention: compute attraction fields
    fn attention_step(&mut self) {
        // Placeholder: in full implementation, would compute pairwise attraction
        // For now, entities maintain their attention state from memory
    }

    /// State update: integrate state changes
    fn state_update_step(&mut self) {
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            let mut gradient = vec![0.0; entity.state.memory.len()];
            for i in 0..gradient.len().min(3) {
                gradient[i] = entity.memory_graph.nodes.len() as f32 * 0.01;
            }
            entity.update_state(&gradient);
        }
    }

    /// Affective: update affective signals from memory
    fn affective_step(&mut self) {
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            entity.memory_graph.update_affective_signals();
        }
    }

    /// Essence: update well-being tracking
    fn essence_step(&mut self) {
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            let mut signals = Vec::new();
            for cluster in entity.memory_graph.clusters.values() {
                signals.push(cluster.affective_signal);
            }
            entity.essence.update(signals.as_slice());
        }
    }

    /// Decision: compute actions based on state and essence
    fn decision_step(&mut self) {
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            let _action = entity.decide();
            // Actions are implicitly applied in integration step
        }
    }

    /// Integration: advance positions and velocities
    fn integration_step(&mut self) {
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            let mut acceleration = vec![0.0; entity.pose.position.len()];
            for i in 0..acceleration.len().min(2) {
                acceleration[i] = 0.01; // Small constant acceleration
            }

            entity.integrate(
                acceleration,
                self.config.dynamics.dt,
                self.config.dynamics.min_speed,
                self.config.dynamics.damping,
            );
        }
    }

    /// Boundaries: apply periodic boundary conditions
    fn boundary_step(&mut self) {
        let entities = self.entities.all_entities_mut();
        let config = &self.config.geometry;
        let bounds = &config.bounds;

        if config.periodic {
            for entity in entities {
                for d in 0..entity.pose.position.len() {
                    let bound = bounds[d];

                    if entity.pose.position[d] < 0.0 {
                        entity.pose.position[d] += bound;
                    } else if entity.pose.position[d] >= bound {
                        entity.pose.position[d] -= bound;
                    }
                }
            }
        }
    }

    /// Memory decay: apply forgetting
    fn memory_decay_step(&mut self) {
        let entities = self.entities.all_entities_mut();

        for entity in entities {
            entity.memory_graph.decay(self.config.state.decay_alpha);
        }
    }

    /// Metrics: compute evaluation metrics
    fn metrics_step(&mut self) {
        let metrics = Metrics::compute(&self.entities, self.timestamp);
        self.metrics_history.push(metrics.clone());

        // Capture detailed step information
        let mut step = SimulationStep::new(self.timestamp, metrics);

        let all_entities = self.entities.all_entities();

        // Capture entity positions, velocities, essence, and belief clusters
        for entity in all_entities {
            step.entity_positions
                .push((entity.id.0, entity.pose.position.clone()));
            step.entity_velocities
                .push((entity.id.0, entity.velocity.clone()));
            step.entity_essence
                .push((entity.id.0, entity.essence.value));

            // Capture belief clusters with affective signals
            let mut clusters_for_entity = Vec::new();
            for (cluster_id, cluster) in &entity.memory_graph.clusters {
                clusters_for_entity.push((
                    *cluster_id,
                    cluster.affective_signal,
                    cluster.node_indices.len() as i32,
                ));
            }
            if !clusters_for_entity.is_empty() {
                step.belief_clusters
                    .push((entity.id.0, clusters_for_entity));
            }

            // Capture attention activation
            let mut attention_values = Vec::new();
            for node in &entity.memory_graph.nodes {
                attention_values.push(node.activation);
            }
            if !attention_values.is_empty() {
                step.attentions.push((entity.id.0, attention_values));
            }
        }

        // Compute pairwise attractions (simplified: based on distances)
        let entities_vec = self.entities.all_entities();
        for i in 0..entities_vec.len() {
            for j in (i + 1)..entities_vec.len() {
                let dist = entities_vec[i]
                    .pose
                    .distance_to(&entities_vec[j].pose);
                let attraction = 1.0 / (1.0 + dist); // Simple inverse distance
                if attraction > 0.01 {
                    // Only record significant attractions
                    step.attractions.push((
                        entities_vec[i].id.0,
                        entities_vec[j].id.0,
                        attraction,
                    ));
                }
            }
        }

        // Add step to results
        self.results.add_step(step);
    }

    /// Run simulation for configured number of steps.
    pub fn run(&mut self) {
        let num_steps = self.config.simulation.num_steps;
        for _ in 0..num_steps {
            self.step();
        }
    }

    /// Export metrics as CSV.
    pub fn export_metrics_csv(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;

        // Header
        writeln!(
            file,
            "timestamp,attention_entropy,memory_diversity,velocity_stability,identity_coherence,cluster_stability,affective_strength,essence_trajectory,average_essence"
        )?;

        // Data
        for metrics in &self.metrics_history {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{}",
                metrics.timestamp,
                metrics.attention_entropy,
                metrics.memory_diversity,
                metrics.velocity_stability,
                metrics.identity_coherence,
                metrics.cluster_stability,
                metrics.affective_strength,
                metrics.essence_trajectory,
                metrics.average_essence
            )?;
        }

        Ok(())
    }

    /// Finalize simulation results and analyze consciousness.
    pub fn finalize_results(&mut self) {
        let end_time = Local::now().to_rfc3339();
        self.results.end_time = end_time;
        self.results.duration_seconds = self.timestamp as f32 * self.config.dynamics.dt;
        self.results.analyze_consciousness();
    }

    /// Generate detailed report files (text and summary).
    pub fn generate_report(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
        let txt_file = format!("{}_report.txt", prefix);
        let html_file = format!("{}_report.html", prefix);
        
        self.results.generate_text_report(&txt_file)?;
        self.results.generate_html_report(&html_file)?;
        
        println!("Report generated: {}", txt_file);
        println!("Report generated: {}", html_file);
        
        // Print consciousness summary to console
        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║              CONSCIOUSNESS ANALYSIS SUMMARY                   ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!(
            "Consciousness Score: {:.1}%",
            self.results.consciousness_analysis.consciousness_score * 100.0
        );
        println!(
            "Status: {}",
            if self.results.consciousness_analysis.consciousness_achieved {
                "✓ CONSCIOUSNESS LIKELY ACHIEVED"
            } else {
                "✗ CONSCIOUSNESS NOT ACHIEVED"
            }
        );
        println!();
        println!("Passed Criteria: {}/{}", 
            self.results.consciousness_analysis.passed_metrics.len(),
            self.results.consciousness_analysis.passed_metrics.len() + 
            self.results.consciousness_analysis.failed_metrics.len()
        );
        for metric in &self.results.consciousness_analysis.passed_metrics {
            println!("  ✓ {}", metric);
        }
        for failure in &self.results.consciousness_analysis.failed_metrics {
            println!("  ✗ {}", failure);
        }
        println!();
        println!("Details in: {} or {}", txt_file, html_file);
        println!();

        Ok(())
    }

    /// Get the consciousness analysis result.
    pub fn consciousness_achieved(&self) -> bool {
        self.results.consciousness_analysis.consciousness_achieved
    }

    /// Get the consciousness score (0.0 to 1.0).
    pub fn consciousness_score(&self) -> f32 {
        self.results.consciousness_analysis.consciousness_score
    }
    
    /// Update visualization state with current simulation data
    pub fn update_visualization(&self, viz_state: &std::sync::Arc<std::sync::Mutex<crate::visualization::VisualizationState>>) {
        use crate::visualization::EntityState;
        
        let entities = self.entities.all_entities();
        let mut entity_states = Vec::new();
        
        for entity in &entities {
            // Use memory state vector as attention proxy
            let attention_vals: Vec<f32> = entity.state.memory.iter().take(10).cloned().collect();
            let num_clusters = entity.memory_graph.clusters.len();
            
            // Compute affective strength from clusters
            let affective_strength: f32 = if entity.memory_graph.clusters.is_empty() {
                0.0
            } else {
                entity.memory_graph.clusters.values()
                    .map(|c| c.affective_signal.abs())
                    .sum::<f32>() / entity.memory_graph.clusters.len() as f32
            };
            
            entity_states.push(EntityState {
                id: entity.id.0,
                position: entity.pose.position.clone(),
                velocity: entity.velocity.clone(),
                essence: entity.essence.value,
                affective_strength,
                attention: attention_vals,
                num_clusters,
            });
        }
        
        // Compute attractions between entities (simplified pairwise)
        let mut attractions = Vec::new();
        let max_bound = self.config.geometry.bounds.iter().cloned().fold(0.0f32, f32::max).max(1.0);
        let scale_factor = max_bound / 10.0; // Normalize to ~10 unit space
        
        for i in 0..entities.len() {
            for j in (i+1)..entities.len() {
                let dist_sq: f32 = entities[i].pose.position.iter()
                    .zip(entities[j].pose.position.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum();
                
                if dist_sq > 0.0 {
                    // Scale strength based on bounds size - larger space = lower threshold
                    let normalized_dist_sq = dist_sq / (scale_factor * scale_factor);
                    let strength = 1.0 / (normalized_dist_sq + 1.0);
                    if strength > 0.001 {  // Lower threshold for larger spaces
                        attractions.push((i, j, strength));
                    }
                }
            }
        }
        
        // Update visualization state
        if let Ok(mut state) = viz_state.lock() {
            state.step = self.timestamp;
            state.entities = entity_states;
            state.attractions = attractions;
            state.dimension = self.config.geometry.dimension;
            state.bounds = self.config.geometry.bounds.clone();
            
            // Update metrics history
            if let Some(metrics) = self.metrics_history.last() {
                state.metrics.push(self.timestamp, metrics);
            }
            
            // Debug: print first update info
            if self.timestamp == 0 {
                println!("[DEBUG] First visualization update:");
                println!("  Entities: {}", state.entities.len());
                println!("  Dimension: {}", state.dimension);
                println!("  Bounds: {:?}", state.bounds);
                if !state.entities.is_empty() {
                    println!("  First entity pos: {:?}", state.entities[0].position);
                }
            }
        }
    }
}
