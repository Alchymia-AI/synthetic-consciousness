//! Simulation module: main orchestration and stepping.

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
}
