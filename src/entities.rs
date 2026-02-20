//! Entities module: embodied agents combining all components.

use serde::{Deserialize, Serialize};
use crate::geometry::Pose;
use crate::state::EntityStateVector;
use crate::memory::MemoryGraph;
use crate::essence::EssenceIndex;
use std::collections::HashMap;

/// Unique identifier for an entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u32);

/// Embodied agent combining geometry, state, memory, and essence.
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: EntityId,
    pub pose: Pose,
    pub velocity: Vec<f32>,
    pub state: EntityStateVector,
    pub memory_graph: MemoryGraph,
    pub essence: EssenceIndex,
    pub baseline_drives: (f32, f32), // (self-preservation, curiosity)
}

impl Entity {
    /// Create a new entity.
    pub fn new(
        id: EntityId,
        pose: Pose,
        state: EntityStateVector,
        memory_graph: MemoryGraph,
        essence: EssenceIndex,
    ) -> Self {
        let dim = pose.position.len();
        Entity {
            id,
            pose,
            velocity: vec![0.0; dim],
            state,
            memory_graph,
            essence,
            baseline_drives: (0.5, 0.5),
        }
    }

    /// Sense the local environment (placeholder).
    pub fn sense(&mut self, stimulus: Vec<f32>, _timestamp: u64) {
        // Record stimulus as memory node
        let node = crate::memory::MemoryNode::new(stimulus.clone(), _timestamp);
        let idx = self.memory_graph.add_node(node);
        self.memory_graph.cluster_event(&stimulus, idx, 0.7);
    }

    /// Update internal state based on affective signals.
    pub fn update_state(&mut self, attention_gradient: &[f32]) {
        // Update state vector with gradient influence
        let len = self.state.memory.len();
        for i in 0..len {
            let grad = attention_gradient.get(i).copied().unwrap_or(0.0);
            self.state.memory[i] = self.state.memory[i] * 0.95 + grad * 0.05;
        }

        // Update essence based on memory affective signals
        let mut total_affective = 0.0;
        for cluster in self.memory_graph.clusters.values() {
            total_affective += cluster.affective_signal;
        }
        let avg_affective = if self.memory_graph.clusters.is_empty() {
            0.0
        } else {
            total_affective / self.memory_graph.clusters.len() as f32
        };

        self.essence.update(&[avg_affective]);
    }

    /// Decide on action based on state and essence.
    pub fn decide(&self) -> Vec<f32> {
        let preservation = self.baseline_drives.0;
        let curiosity = self.baseline_drives.1;
        let essence_influence = self.essence.influence_factor();

        // Simplified decision: stochastic combination of drives
        let mut action = self.state.memory.clone();
        for a in &mut action {
            *a = *a * (preservation + curiosity) * essence_influence;
        }
        action
    }

    /// Apply action as acceleration.
    pub fn act(&mut self, acceleration: Vec<f32>) {
        // Apply acceleration via velocity update
        for i in 0..self.velocity.len() {
            if i < acceleration.len() {
                self.velocity[i] += acceleration[i];
            }
        }
    }

    /// Integration step with perpetual velocity.
    pub fn integrate(
        &mut self,
        acceleration: Vec<f32>,
        dt: f32,
        min_speed: f32,
        damping: f32,
    ) {
        // Apply acceleration and damping
        for i in 0..self.velocity.len() {
            let acc = acceleration.get(i).copied().unwrap_or(0.0);
            self.velocity[i] = (self.velocity[i] + dt * acc) * damping;
        }

        // Enforce perpetual velocity
        let speed_sq: f32 = self.velocity.iter().map(|v| v * v).sum();
        let speed = speed_sq.sqrt();

        if speed < min_speed && speed > 1e-6 {
            let scale = min_speed / speed;
            for v in &mut self.velocity {
                *v *= scale;
            }
        } else if speed <= 1e-6 {
            self.velocity[0] = min_speed;
        }

        // Update position
        for i in 0..self.pose.position.len() {
            if i < self.velocity.len() {
                self.pose.position[i] += dt * self.velocity[i];
            }
        }
    }
}

/// Collection of entities in simulation.
pub struct EntityPool {
    entities: HashMap<EntityId, Entity>,
    next_id: u32,
}

impl EntityPool {
    pub fn new() -> Self {
        EntityPool {
            entities: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> EntityId {
        let id = EntityId(self.next_id);
        entity.id = id;
        self.entities.insert(id, entity);
        self.next_id += 1;
        id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn all_entities(&self) -> Vec<&Entity> {
        self.entities.values().collect()
    }

    pub fn all_entities_mut(&mut self) -> Vec<&mut Entity> {
        self.entities.values_mut().collect()
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }
}
