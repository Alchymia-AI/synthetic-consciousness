//! Metrics module: evaluation metrics for consciousness quality.

use crate::entities::EntityPool;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Comprehensive metrics for consciousness evaluation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub timestamp: u64,
    pub attention_entropy: f32,
    pub memory_diversity: f32,
    pub velocity_stability: f32,
    pub identity_coherence: f32,
    pub cluster_stability: f32,
    pub affective_strength: f32,
    pub essence_trajectory: f32,
    pub average_essence: f32,
}

impl Metrics {
    /// Compute all metrics from entity pool.
    pub fn compute(entities: &EntityPool, timestamp: u64) -> Self {
        let attention_entropy = Self::compute_attention_entropy(entities);
        let memory_diversity = Self::compute_memory_diversity(entities);
        let velocity_stability = Self::compute_velocity_stability(entities);
        let identity_coherence = Self::compute_identity_coherence(entities);
        let cluster_stability = Self::compute_cluster_stability(entities);
        let affective_strength = Self::compute_affective_strength(entities);
        let essence_trajectory = Self::compute_essence_trajectory(entities);
        let average_essence = Self::compute_average_essence(entities);

        Metrics {
            timestamp,
            attention_entropy,
            memory_diversity,
            velocity_stability,
            identity_coherence,
            cluster_stability,
            affective_strength,
            essence_trajectory,
            average_essence,
        }
    }

    /// Compute attention entropy: Shannon entropy over active memory nodes.
    fn compute_attention_entropy(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 0.0;
        }

        let mut total_entropy = 0.0;

        for entity in &all_entities {
            let activations: Vec<f32> = entity
                .memory_graph
                .nodes
                .iter()
                .map(|n| n.activation)
                .collect();

            if activations.is_empty() {
                continue;
            }

            // Normalize activations to probabilities
            let sum: f32 = activations.iter().sum();
            if sum > 1e-6 {
                let mut entropy = 0.0;
                for a in activations {
                    let p = a / sum;
                    if p > 1e-6 {
                        entropy -= p * p.ln();
                    }
                }
                total_entropy += entropy;
            }
        }

        if all_entities.is_empty() {
            0.0
        } else {
            total_entropy / all_entities.len() as f32
        }
    }

    /// Compute memory diversity: variance in belief cluster affective signals.
    fn compute_memory_diversity(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 0.0;
        }

        let mut total_diversity = 0.0;

        for entity in &all_entities {
            let affective_signals: Vec<f32> = entity
                .memory_graph
                .clusters
                .values()
                .map(|c| c.affective_signal)
                .collect();

            if affective_signals.len() < 2 {
                continue;
            }

            let mean: f32 = affective_signals.iter().sum::<f32>() / affective_signals.len() as f32;
            let variance: f32 = affective_signals
                .iter()
                .map(|s| (s - mean).powi(2))
                .sum::<f32>()
                / affective_signals.len() as f32;

            total_diversity += variance.sqrt();
        }

        if all_entities.is_empty() {
            0.0
        } else {
            total_diversity / all_entities.len() as f32
        }
    }

    /// Compute velocity stability: inverse of velocity variance across entities.
    fn compute_velocity_stability(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 1.0;
        }

        let mut speeds = Vec::new();
        for entity in all_entities {
            let speed_sq: f32 = entity.velocity.iter().map(|v| v * v).sum();
            speeds.push(speed_sq.sqrt());
        }

        if speeds.is_empty() {
            return 1.0;
        }

        let mean: f32 = speeds.iter().sum::<f32>() / speeds.len() as f32;
        let variance: f32 = speeds
            .iter()
            .map(|s| (s - mean).powi(2))
            .sum::<f32>()
            / speeds.len() as f32;

        let std_dev = variance.sqrt();

        if mean > 1e-6 {
            1.0 / (1.0 + std_dev / mean)
        } else {
            1.0
        }
    }

    /// Compute identity coherence: state vector norm consistency.
    fn compute_identity_coherence(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 0.0;
        }

        let mut norms = Vec::new();
        for entity in all_entities {
            norms.push(entity.state.norm());
        }

        if norms.is_empty() {
            return 0.0;
        }

        let mean: f32 = norms.iter().sum::<f32>() / norms.len() as f32;
        let variance: f32 = norms
            .iter()
            .map(|n| (n - mean).powi(2))
            .sum::<f32>()
            / norms.len() as f32;

        if mean > 1e-6 {
            1.0 / (1.0 + variance.sqrt() / mean)
        } else {
            0.0
        }
    }

    /// Compute cluster stability: number of stable belief clusters.
    fn compute_cluster_stability(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 0.0;
        }

        let mut cluster_counts = Vec::new();
        for entity in all_entities {
            cluster_counts.push(entity.memory_graph.clusters.len() as f32);
        }

        let mean: f32 = cluster_counts.iter().sum::<f32>() / cluster_counts.len() as f32;
        mean / 10.0 // Normalize to [0, 1] assuming max ~10 clusters
    }

    /// Compute affective strength: average magnitude of affective signals.
    fn compute_affective_strength(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 0.0;
        }

        let mut total_strength = 0.0;
        let mut count = 0;

        for entity in all_entities {
            for cluster in entity.memory_graph.clusters.values() {
                total_strength += cluster.affective_signal.abs();
                count += 1;
            }
        }

        if count == 0 {
            0.0
        } else {
            total_strength / count as f32
        }
    }

    /// Compute essence trajectory: stability of essence indices over time.
    fn compute_essence_trajectory(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 5.0; // Baseline
        }

        let mut essences = Vec::new();
        for entity in all_entities {
            essences.push(entity.essence.value);
        }

        let mean: f32 = essences.iter().sum::<f32>() / essences.len() as f32;
        mean
    }

    /// Compute average essence: mean well-being across entities.
    fn compute_average_essence(entities: &EntityPool) -> f32 {
        let all_entities = entities.all_entities();
        if all_entities.is_empty() {
            return 5.0;
        }

        let sum: f32 = all_entities.iter().map(|e| e.essence.value).sum();
        sum / all_entities.len() as f32
    }

    /// Return metrics as a HashMap for easy serialization.
    pub fn to_map(&self) -> HashMap<String, f32> {
        let mut map = HashMap::new();
        map.insert("timestamp".to_string(), self.timestamp as f32);
        map.insert("attention_entropy".to_string(), self.attention_entropy);
        map.insert("memory_diversity".to_string(), self.memory_diversity);
        map.insert("velocity_stability".to_string(), self.velocity_stability);
        map.insert("identity_coherence".to_string(), self.identity_coherence);
        map.insert("cluster_stability".to_string(), self.cluster_stability);
        map.insert("affective_strength".to_string(), self.affective_strength);
        map.insert("essence_trajectory".to_string(), self.essence_trajectory);
        map.insert("average_essence".to_string(), self.average_essence);
        map
    }
}
