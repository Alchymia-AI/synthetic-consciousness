//! Essence module: well-being tracking and influence computation.
//!
//! This module implements the Essence Index, a scalar value (0-10) representing
//! an entity's affective state or "well-being":
//! - 0 = Dread, suffering, extreme negativity
//! - 5 = Neutral, balanced baseline
//! - 10 = Joyous, optimal well-being
//!
//! ## Essence Dynamics
//!
//! Essence evolves based on:
//! - Decay toward baseline (homeostatic regulation)
//! - Affective signals from belief clusters (experience integration)
//! - Experience scaling (sensitivity to events)
//!
//! ## Architectural Role
//!
//! Essence provides a simple but powerful metric of subjective experience.
//! It influences entity behavior and serves as a key indicator of consciousness
//! quality in the evaluation metrics (Average Essence).
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use serde::{Deserialize, Serialize};

/// Configuration for Essence Index behavior.
/// 
/// Controls baseline well-being, decay rate, and sensitivity to experiences.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EssenceConfig {
    /// Baseline value (typically 5.0, midpoint).
    pub baseline: f32,
    /// Decay rate toward baseline per step.
    pub decay: f32,
    /// Scaling factor for experience delta.
    pub experience_scale: f32,
}

impl EssenceConfig {
    pub fn default() -> Self {
        EssenceConfig {
            baseline: 5.0,
            decay: 0.001,
            experience_scale: 1.0,
        }
    }
}

/// Essence Index tracking well-being (0 = dread, 10 = joyous).
/// 
/// Represents the affective or hedonic tone of an entity's experience.
/// Influences behavior through the influence_factor() method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EssenceIndex {
    /// Current value (0.0 to 10.0).
    pub value: f32,
    /// Configuration parameters.
    pub config: EssenceConfig,
}

impl EssenceIndex {
    /// Create a new Essence Index.
    /// 
    /// Initializes at the configured baseline value.
    /// 
    /// # Arguments
    /// * `config` - Essence configuration
    /// 
    /// # Returns
    /// New EssenceIndex at baseline
    pub fn new(config: EssenceConfig) -> Self {
        EssenceIndex {
            value: config.baseline,
            config,
        }
    }

    /// Update Essence based on aggregated affective signals.
    /// 
    /// Applies a weighted combination of:
    /// - Decay toward baseline (homeostasis)
    /// - Affective signal influence (experience integration)
    /// 
    /// The value is clamped to [0, 10] to maintain valid range.
    /// 
    /// # Arguments
    /// * `affective_signals` - Array of affective values from belief clusters
    pub fn update(&mut self, affective_signals: &[f32]) {
        let avg_signal = if affective_signals.is_empty() {
            0.0
        } else {
            affective_signals.iter().sum::<f32>() / affective_signals.len() as f32
        };

        // Clamp signal to [-5, +5] and rescale to affect index change
        let bounded_signal = avg_signal.max(-5.0).min(5.0);
        let delta = bounded_signal * self.config.experience_scale;

        // Update with decay toward baseline
        self.value = self.value
            + (self.config.baseline - self.value) * self.config.decay
            + delta;

        // Clamp to [0, 10]
        self.value = self.value.max(0.0).min(10.0);
    }

    /// Compute influence factor (extremity modulates response decisiveness).
    pub fn influence_factor(&self) -> f32 {
        2.0 * (self.value - self.config.baseline).abs()
    }

    /// Get extremity (distance from baseline).
    pub fn extremity(&self) -> f32 {
        (self.value - self.config.baseline).abs()
    }
}

impl Default for EssenceIndex {
    fn default() -> Self {
        EssenceIndex::new(EssenceConfig::default())
    }
}
