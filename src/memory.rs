//! Memory module: memory graph, nodes, and belief cluster management.
//!
//! This module implements the memory system as a graph structure where:
//! - **Nodes**: Individual memory events with activation levels
//! - **Edges**: Associative links between related memories
//! - **Clusters**: Semantic groupings of similar experiences (belief structures)
//!
//! ## Belief Clusters
//!
//! Clusters are formed through automatic clustering of similar event vectors.
//! Each cluster maintains:
//! - Member node indices
//! - Affective signal (emotional valence)
//! - Weight (importance/frequency of activation)
//!
//! ## Architectural Role
//!
//! The memory graph is a core primitive enabling:
//! - Semantic organization of experiences
//! - Affective signal generation from clustered beliefs
//! - Temporal identity through persistent memory structures
//! - Context-dependent behavior based on past experiences
//!
//! Belief clusters are essential for consciousness emergence as they provide
//! the organizational structure for coherent self-representation.
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single memory node representing an event in an entity's history.
/// 
/// Memory nodes encode experiences and decay over time.
/// They are clustered into belief structures based on semantic similarity.
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
    /// 
    /// Initialized with full activation (1.0) and no cluster assignment.
    /// 
    /// # Arguments
    /// * `event` - Event vector encoding the experience
    /// * `timestamp` - Creation time
    /// 
    /// # Returns
    /// New MemoryNode
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
/// 
/// Clusters represent coherent concepts or beliefs formed from
/// repeated exposure to similar experiences. Each cluster has an
/// affective signal representing its emotional valence.
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
/// 
/// Maintains:
/// - Nodes: Individual memory events
/// - Edges: Associative links between memories
/// - Clusters: Semantic belief structures
/// 
/// Provides methods for adding, clustering, and maintaining memories.
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
    /// Create a new empty memory graph.
    /// 
    /// # Returns
    /// Empty MemoryGraph ready to receive nodes
    pub fn new() -> Self {
        MemoryGraph {
            nodes: vec![],
            edges: vec![],
            clusters: HashMap::new(),
            next_cluster_id: 0,
        }
    }

    /// Add a memory node.
    /// 
    /// # Arguments
    /// * `node` - Memory node to add
    /// 
    /// # Returns
    /// Index of the added node
    pub fn add_node(&mut self, node: MemoryNode) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    /// Add an edge between two nodes.
    /// 
    /// Creates an associative link between memory nodes.
    /// Both indices must be valid.
    /// 
    /// # Arguments
    /// * `src` - Source node index
    /// * `dst` - Destination node index
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        if src < self.nodes.len() && dst < self.nodes.len() {
            self.edges.push((src, dst));
        }
    }

    /// Decay all node activations.
    /// 
    /// Implements forgetting by reducing activation levels.
    /// 
    /// # Arguments
    /// * `factor` - Decay factor (e.g., 0.99 for 1% decay per step)
    pub fn decay(&mut self, factor: f32) {
        for node in &mut self.nodes {
            node.activation *= factor;
        }
    }

    /// Compute cosine similarity between two event vectors.
    /// 
    /// Returns normalized dot product, measuring semantic similarity.
    /// 
    /// # Arguments
    /// * `vec1` - First vector
    /// * `vec2` - Second vector
    /// 
    /// # Returns
    /// Similarity in range [-1, 1], or 0 if either vector is empty
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
    /// 
    /// Assigns the event (represented by node_idx) to the best-matching
    /// cluster if similarity exceeds threshold tau. If no suitable cluster
    /// exists, creates a new one.
    /// 
    /// # Arguments
    /// * `event` - Event vector
    /// * `node_idx` - Index of the memory node
    /// * `tau` - Similarity threshold for cluster membership
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
