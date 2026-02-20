//! Attraction module: potential fields and attention computation.

use serde::{Deserialize, Serialize};

/// Kernel type for attraction potential computation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KernelType {
    Gaussian,
    InverseDistance,
}

/// Configuration for the attraction field.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttractionConfig {
    pub kernel: KernelType,
    /// Kernel bandwidth parameter sigma.
    pub sigma: f32,
    /// Softmax temperature for attention selection.
    pub lambda: f32,
}

/// Compute Gaussian kernel.
pub fn gaussian_kernel(distance: f32, sigma: f32) -> f32 {
    let sigma2 = sigma * sigma;
    (-distance.powi(2) / (2.0 * sigma2)).exp()
}

/// Compute inverse-distance kernel (with epsilon for numerical stability).
pub fn inverse_distance_kernel(distance: f32, _sigma: f32) -> f32 {
    1.0 / (distance + 1e-6)
}

/// Compute attraction kernel based on type.
pub fn compute_kernel(kernel_type: &KernelType, distance: f32, sigma: f32) -> f32 {
    match kernel_type {
        KernelType::Gaussian => gaussian_kernel(distance, sigma),
        KernelType::InverseDistance => inverse_distance_kernel(distance, sigma),
    }
}

/// Compute attraction potential for an entity given positions of others.
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
