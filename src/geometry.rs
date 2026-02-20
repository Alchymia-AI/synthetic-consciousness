//! Geometry module: spatial primitives, topology, and position/orientation management.

use serde::{Deserialize, Serialize};

/// Represents the pose (position and orientation) of an entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pose {
    /// Position in d-dimensional space (d=2 or d=3)
    pub position: Vec<f32>,
    /// Orientation as quaternion [w, x, y, z]
    pub orientation: [f32; 4],
}

impl Pose {
    /// Create a new pose in d-dimensional space.
    pub fn new(dimension: usize) -> Self {
        Pose {
            position: vec![0.0; dimension],
            orientation: [1.0, 0.0, 0.0, 0.0],
        }
    }

    /// Set position with a vector.
    pub fn with_position(mut self, pos: Vec<f32>) -> Self {
        self.position = pos;
        self
    }

    /// Set orientation with a quaternion.
    pub fn with_orientation(mut self, quat: [f32; 4]) -> Self {
        self.orientation = quat;
        self
    }

    /// Compute Euclidean distance to another pose.
    pub fn distance_to(&self, other: &Pose) -> f32 {
        self.position
            .iter()
            .zip(other.position.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

/// Geometry configuration specifying the dimensionality and bounds of the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeometryConfig {
    /// Dimension of the world (2 for 2D plane, 3 for 3D space).
    pub dimension: usize,
    /// Bounds of the world (one value per dimension).
    pub bounds: Vec<f32>,
    /// Optional periodic boundary conditions (wrapping).
    pub periodic: bool,
}

impl GeometryConfig {
    /// Create a default 3D geometry configuration.
    pub fn default_3d() -> Self {
        GeometryConfig {
            dimension: 3,
            bounds: vec![100.0, 100.0, 100.0],
            periodic: false,
        }
    }

    /// Create a default 2D geometry configuration.
    pub fn default_2d() -> Self {
        GeometryConfig {
            dimension: 2,
            bounds: vec![100.0, 100.0],
            periodic: false,
        }
    }

    /// Validate bounds match dimension.
    pub fn is_valid(&self) -> bool {
        self.bounds.len() == self.dimension && self.dimension >= 2 && self.dimension <= 3
    }
}

/// Apply periodic boundary conditions if enabled.
pub fn apply_periodic_bounds(position: &mut [f32], bounds: &[f32], periodic: bool) {
    if !periodic {
        return;
    }
    for (pos, bound) in position.iter_mut().zip(bounds.iter()) {
        if *pos < 0.0 {
            *pos = (pos.abs() % bound).abs();
            *pos = bound - *pos;
        } else if *pos > *bound {
            *pos = *pos % bound;
        }
    }
}
