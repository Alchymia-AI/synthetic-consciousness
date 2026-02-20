//! Dynamics module: motion integration and perpetual velocity.

use serde::{Deserialize, Serialize};

/// Configuration for dynamics integration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicsConfig {
    /// Time step.
    pub dt: f32,
    /// Minimum speed to maintain.
    pub min_speed: f32,
    /// Velocity damping per step.
    pub damping: f32,
}

impl DynamicsConfig {
    pub fn default() -> Self {
        DynamicsConfig {
            dt: 0.01,
            min_speed: 0.05,
            damping: 0.99,
        }
    }
}

/// Integrate motion with perpetual velocity enforcement.
pub fn integrate_motion(
    position: &mut [f32],
    velocity: &mut [f32],
    acceleration: &[f32],
    config: &DynamicsConfig,
) {
    let dt = config.dt;
    let min_speed = config.min_speed;
    let damping = config.damping;

    // Apply acceleration and damping
    for i in 0..velocity.len() {
        let acc = acceleration.get(i).copied().unwrap_or(0.0);
        velocity[i] = (velocity[i] + dt * acc) * damping;
    }

    // Enforce perpetual velocity (minimum speed injection)
    let speed_sq: f32 = velocity.iter().map(|v| v * v).sum();
    let speed = speed_sq.sqrt();

    if speed < min_speed && speed > 1e-6 {
        // Scale up velocity to meet minimum
        let scale = min_speed / speed;
        for v in velocity.iter_mut() {
            *v *= scale;
        }
    } else if speed <= 1e-6 {
        // Inject minimum velocity in random direction (simplified: x-direction)
        velocity[0] = min_speed;
    }

    // Update position
    for i in 0..position.len() {
        position[i] += dt * velocity[i];
    }
}

/// Compute acceleration from attraction gradient.
pub fn compute_acceleration_from_gradient(gradient: &[f32]) -> Vec<f32> {
    // Simplified: acceleration proportional to gradient
    gradient.iter().map(|g| g * 0.1).collect()
}

/// Compute baseline drives (self-preservation and curiosity).
pub fn compute_baseline_drives(
    min_distance_to_others: f32,
    attention_magnitude: f32,
) -> (f32, f32) {
    let preservation = if min_distance_to_others > 1e-6 {
        1.0 / min_distance_to_others
    } else {
        1.0
    };
    let curiosity = attention_magnitude;

    (preservation, curiosity)
}
