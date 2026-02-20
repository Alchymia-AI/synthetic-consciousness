//! Attraction module: potential fields and attention computation.
//!
//! This module implements the attention layer of the Geometric Consciousness Model.
//! Entities exert attraction forces on each other based on spatial proximity and
//! configurable kernels (Gaussian or inverse-distance).
//!
//! ## Core Concepts
//!
//! - **Attraction Potential**: Scalar field representing total influence from other entities
//! - **Attention Gradient**: Vector pointing toward regions of high attraction
//! - **Kernel Functions**: Mathematical shapes controlling influence falloff with distance
//!
//! ## Architectural Role
//!
//! The attention layer creates a dynamic field that guides entity motion and awareness.
//! This is a core architectural primitive enabling emergent collective behavior and
//! information flow between entities.
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use serde::{Deserialize, Serialize};

/// Kernel type for attraction potential computation.
/// 
/// Different kernels produce different shapes of influence falloff:
/// - Gaussian: Smooth, bell-shaped falloff (local influence)
/// - InverseDistance: Long-range power-law falloff
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KernelType {
    Gaussian,
    InverseDistance,
}

/// Configuration for the attraction field.
/// 
/// Controls how entities attract each other and how attention is allocated.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttractionConfig {
    pub kernel: KernelType,
    /// Kernel bandwidth parameter sigma.
    pub sigma: f32,
    /// Softmax temperature for attention selection.
    pub lambda: f32,
}

/// Compute Gaussian kernel.
/// 
/// Returns exp(-d²/(2σ²)) for smooth, localized influence.
/// 
/// # Arguments
/// * `distance` - Euclidean distance between entities
/// * `sigma` - Kernel bandwidth (width of Gaussian)
/// 
/// # Returns
/// Kernel value in range [0, 1]
pub fn gaussian_kernel(distance: f32, sigma: f32) -> f32 {
    let sigma2 = sigma * sigma;
    (-distance.powi(2) / (2.0 * sigma2)).exp()
}

/// Compute inverse-distance kernel (with epsilon for numerical stability).
/// 
/// Returns 1/(d + ε) for long-range influence.
/// 
/// # Arguments
/// * `distance` - Euclidean distance between entities
/// * `_sigma` - Unused (kept for interface consistency)
/// 
/// # Returns
/// Kernel value, unbounded but decreasing with distance
pub fn inverse_distance_kernel(distance: f32, _sigma: f32) -> f32 {
    1.0 / (distance + 1e-6)
}

/// Compute attraction kernel based on type.
/// 
/// Dispatches to the appropriate kernel function.
/// 
/// # Arguments
/// * `kernel_type` - Which kernel to use
/// * `distance` - Distance between entities
/// * `sigma` - Kernel parameter
/// 
/// # Returns
/// Computed kernel value
pub fn compute_kernel(kernel_type: &KernelType, distance: f32, sigma: f32) -> f32 {
    match kernel_type {
        KernelType::Gaussian => gaussian_kernel(distance, sigma),
        KernelType::InverseDistance => inverse_distance_kernel(distance, sigma),
    }
}

/// Compute attraction potential for an entity given positions of others.
/// 
/// Sums weighted kernel values across all other entities to produce
/// a scalar potential field value at the given position.
/// 
/// # Arguments
/// * `position` - Position to evaluate potential at
/// * `others` - Positions of other entities
/// * `weights` - Per-entity weights (influence multipliers)
/// * `kernel_config` - Kernel configuration
/// 
/// # Returns
/// Scalar potential value (higher = more attraction)
pub fn attraction_potential(
    position: &[f32],
    others: &[Vec<f32>],
    weights: &[f32],
    kernel_config: &AttractionConfig,
) -> f32 {
    let mut potential = 0.0;
    for (idx, other_pos) in others.iter().enumerate() {
        let distance: f32 = position
            .iter()
            .zip(other_pos.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt();
        let kernel_val = compute_kernel(&kernel_config.kernel, distance, kernel_config.sigma);
        let weight = weights.get(idx).copied().unwrap_or(1.0);
        potential += weight * kernel_val;
    }
    potential
}

/// Compute attention prompts (gradient-like force) toward other entities.
/// 
/// Uses finite differences to approximate the gradient of the attraction
/// potential, producing a vector pointing toward regions of high influence.
/// This gradient guides entity motion and attention allocation.
/// 
/// # Arguments
/// * `position` - Position to compute gradient at
/// * `others` - Positions of other entities
/// * `weights` - Per-entity influence weights
/// * `kernel_config` - Kernel configuration
/// 
/// # Returns
/// Gradient vector (same dimensionality as position)
pub fn attention_gradient(
    position: &[f32],
    others: &[Vec<f32>],
    weights: &[f32],
    kernel_config: &AttractionConfig,
) -> Vec<f32> {
    let mut gradient = vec![0.0; position.len()];
    let h = 1e-5; // finite difference step

    for dim in 0..position.len() {
        let mut pos_plus = position.to_vec();
        pos_plus[dim] += h;
        let phi_plus = attraction_potential(&pos_plus, others, weights, kernel_config);

        let mut pos_minus = position.to_vec();
        pos_minus[dim] -= h;
        let phi_minus = attraction_potential(&pos_minus, others, weights, kernel_config);

        gradient[dim] = -(phi_plus - phi_minus) / (2.0 * h);
    }

    gradient
}

/// Compute softmax attention distribution toward neighbors.
pub fn softmax_attention(scores: &[f32], lambda: f32) -> Vec<f32> {
    if scores.is_empty() {
        return vec![];
    }

    let max_score = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exp_scores: Vec<f32> = scores
        .iter()
        .map(|s| ((s - max_score) * lambda).exp())
        .collect();
    let sum_exp: f32 = exp_scores.iter().sum();

    if sum_exp > 0.0 {
        exp_scores.iter().map(|e| e / sum_exp).collect()
    } else {
        vec![0.0; scores.len()]
    }
}
