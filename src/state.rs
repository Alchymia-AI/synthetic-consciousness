//! State module: entity state vector and context management.
//!
//! This module defines the internal state representation for entities,
//! consisting of three components:
//! - **Memory**: Long-term representation encoding past experiences
//! - **Context**: Short-term working memory of current situation
//! - **Traits**: Persistent personality/behavioral characteristics
//!
//! ## State Dynamics
//!
//! States evolve through:
//! - Decay toward baseline (forgetting)
//! - Attention-driven updates (learning)
//! - Memory consolidation (integration)
//!
//! ## Architectural Role
//!
//! The state vector is a core primitive providing temporal continuity
//! and identity coherence. It enables entities to maintain a sense of self
//! across time, a critical requirement for consciousness.
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use serde::{Deserialize, Serialize};

/// Configuration for state dimensionality and decay.
/// 
/// Controls the size and update dynamics of entity state vectors.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateConfig {
    pub memory_dim: usize,
    pub context_dim: usize,
    pub decay_alpha: f32,
    pub beta_attention: f32,
    pub gamma_memory: f32,
}

impl StateConfig {
    pub fn default() -> Self {
        StateConfig {
            memory_dim: 128,
            context_dim: 64,
            decay_alpha: 0.995,
            beta_attention: 0.5,
            gamma_memory: 0.3,
        }
    }
}

/// Internal state of an entity.
/// 
/// Maintains three distinct components:
/// - Memory: Long-term representation of experiences
/// - Context: Short-term working memory
/// - Traits: Persistent personality characteristics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityStateVector {
    /// Long-term memory representation.
    pub memory: Vec<f32>,
    /// Current context vector.
    pub context: Vec<f32>,
    /// Persistent traits.
    pub traits: Vec<f32>,
    /// Configuration.
    pub config: StateConfig,
}

impl EntityStateVector {
    /// Create a new state vector with given configuration.
    /// 
    /// Initializes all components to zero.
    /// 
    /// # Arguments
    /// * `config` - State configuration specifying dimensions
    /// 
    /// # Returns
    /// New EntityStateVector
    pub fn new(config: StateConfig) -> Self {
        EntityStateVector {
            memory: vec![0.0; config.memory_dim],
            context: vec![0.0; config.context_dim],
            traits: vec![0.0; 10], // Fixed trait vector
            config,
        }
    }

    /// Update state based on attention and memory input.
    /// 
    /// Implements the state update equation:
    /// s(t+dt) = α·s(t) + β·g(F) + γ·m
    /// 
    /// Where:
    /// - α = decay toward baseline (forgetting)
    /// - β = attention influence weight
    /// - γ = memory consolidation weight
    /// - F = attention force vector
    /// - m = memory input vector
    /// 
    /// # Arguments
    /// * `attention_force` - Attention gradient from attraction layer
    /// * `memory_input` - Memory signals from belief clusters
    pub fn update(
        &mut self,
        attention_force: &[f32],
        memory_input: &[f32],
    ) {
        // Update memory: s_i(t+dt) = alpha * s_i(t) + beta * g(F_i) + gamma * m_i
        let alpha = self.config.decay_alpha;
        let beta = self.config.beta_attention;
        let gamma = self.config.gamma_memory;

        for i in 0..self.memory.len() {
            let att_contrib = if i < attention_force.len() {
                beta * attention_force[i]
            } else {
                0.0
            };
            let mem_contrib = if i < memory_input.len() {
                gamma * memory_input[i]
            } else {
                0.0
            };

            self.memory[i] = alpha * self.memory[i] + att_contrib + mem_contrib;
        }

        // Optionally update context (simplified: use portion of memory)
        for i in 0..self.context.len() {
            if i < self.memory.len() {
                self.context[i] = 0.5 * self.context[i] + 0.5 * self.memory[i];
            }
        }
    }

    /// Compute state vector norm for identity coherence.
    pub fn norm(&self) -> f32 {
        self.memory
            .iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt()
    }

    /// Compute dot product with another state for coherence.
    pub fn dot(&self, other: &EntityStateVector) -> f32 {
        self.memory
            .iter()
            .zip(other.memory.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl Default for EntityStateVector {
    fn default() -> Self {
        EntityStateVector::new(StateConfig::default())
    }
}
