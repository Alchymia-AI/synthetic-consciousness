//! Memory module: memory graph, nodes, and belief cluster management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single memory node representing an event in an entity's history.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryNode {
    /// Event vector encoding the experience.
    pub event: Vec<f32>,
    /// Activation level (decays over time, ranges from 0 to 1).
    pub activation: f32,
    /// Timestamp when the node was created.
    pub timestamp: u64,
    /// Assigned belief cluster ID.
    pub cluster_id: Option<u32>,
}

impl MemoryNode {
    /// Create a new memory node.
    pub fn new(event: Vec<f32>, timestamp: u64) -> Self {
        MemoryNode {
            event,
            activation: 1.0,
            timestamp,
            cluster_id: None,
        }
    }
}

/// A belief cluster grouping semantically similar memories.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeliefCluster {
    pub id: u32,
    /// Node indices belonging to this cluster.
    pub node_indices: Vec<usize>,
    /// Affective signal strength (-5 to +5).
    pub affective_signal: f32,
    /// Cluster weight (higher for frequently activated clusters).
    pub weight: f32,
}

impl BeliefCluster {
    pub fn new(id: u32) -> Self {
        BeliefCluster {
            id,
            node_indices: vec![],
            affective_signal: 0.0,
            weight: 1.0,
        }
    }
}

/// The memory graph storing all nodes and cluster information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryGraph {
    pub nodes: Vec<MemoryNode>,
    /// Edges as (source_idx, target_idx) pairs.
    pub edges: Vec<(usize, usize)>,
    /// Belief clusters.
    pub clusters: HashMap<u32, BeliefCluster>,
    /// Next cluster ID to assign.
    next_cluster_id: u32,
}

impl MemoryGraph {
    pub fn new() -> Self {
        MemoryGraph {
            nodes: vec![],
            edges: vec![],
            clusters: HashMap::new(),
            next_cluster_id: 0,
        }
    }

    /// Add a memory node.
    pub fn add_node(&mut self, node: MemoryNode) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    /// Add an edge between two nodes.
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        if src < self.nodes.len() && dst < self.nodes.len() {
            self.edges.push((src, dst));
        }
    }

    /// Decay all node activations.
    pub fn decay(&mut self, factor: f32) {
        for node in &mut self.nodes {
            node.activation *= factor;
        }
    }

    /// Compute cosine similarity between two event vectors.
    pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.is_empty() || vec2.is_empty() {
            return 0.0;
        }
        let dot: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1 * norm2)
        } else {
            0.0
        }
    }

    /// Find or create a belief cluster for a new event.
    pub fn cluster_event(&mut self, event: &[f32], node_idx: usize, tau: f32) {
        let mut best_cluster_id = None;
        let mut best_similarity = tau;

        // Find best matching cluster
        for (cluster_id, cluster) in &self.clusters {
            if cluster.node_indices.is_empty() {
                continue;
            }
            let mut total_similarity = 0.0;
            for &node_idx_in_cluster in &cluster.node_indices {
                if let Some(node) = self.nodes.get(node_idx_in_cluster) {
                    total_similarity +=
                        Self::cosine_similarity(event, &node.event) / cluster.node_indices.len() as f32;
                }
            }
            if total_similarity > best_similarity {
                best_similarity = total_similarity;
                best_cluster_id = Some(*cluster_id);
            }
        }

        // Assign to existing cluster or create new one
        let cluster_id = if let Some(cid) = best_cluster_id {
            cid
        } else {
            let cid = self.next_cluster_id;
            self.next_cluster_id += 1;
            self.clusters.insert(cid, BeliefCluster::new(cid));
            cid
        };

        if let Some(cluster) = self.clusters.get_mut(&cluster_id) {
            cluster.node_indices.push(node_idx);
            self.nodes[node_idx].cluster_id = Some(cluster_id);
        }
    }

    /// Update affective signals for all clusters
    pub fn update_affective_signals(&mut self) {
        for cluster in self.clusters.values_mut() {
            let mut signal = 0.0;
            let mut count = 0;

            for &node_idx in &cluster.node_indices {
                if let Some(node) = self.nodes.get(node_idx) {
                    if node.activation > 0.01 {
                        // Compute valence from event (simplified: assuming raw event first element encodes valence)
                        let valence = if node.event.is_empty() {
                            0.0
                        } else {
                            // Map event[0] to [-1, 0, +1] valence
                            if node.event[0] > 0.5 {
                                1.0
                            } else if node.event[0] < -0.5 {
                                -1.0
                            } else {
                                0.0
                            }
                        };
                        signal += node.activation * valence;
                        count += 1;
                    }
                }
            }

            cluster.affective_signal = if count > 0 { signal / count as f32 } else { 0.0 };
        }
    }
}

impl Default for MemoryGraph {
    fn default() -> Self {
        Self::new()
    }
}
