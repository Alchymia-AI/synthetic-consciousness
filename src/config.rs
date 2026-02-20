//! Configuration module: loading and parsing simulation parameters.

use serde::{Deserialize, Serialize};
use crate::geometry::GeometryConfig;
use crate::attraction::AttractionConfig;
use crate::state::StateConfig;
use crate::dynamics::DynamicsConfig;
use crate::essence::EssenceConfig;
use std::fs;

/// Complete simulation configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub metadata: MetadataConfig,
    pub geometry: GeometryConfig,
    pub attraction: AttractionConfig,
    pub state: StateConfig,
    pub dynamics: DynamicsConfig,
    pub essence: EssenceConfig,
    pub simulation: SimulationParams,
}

/// Metadata about the simulation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataConfig {
    pub name: String,
    pub description: String,
    pub version: String,
}

/// Simulation runtime parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationParams {
    pub num_entities: u32,
    pub num_steps: u32,
    pub dt: f32,
    pub seed: u64,
}

impl SimulationConfig {
    /// Load configuration from TOML file.
    pub fn from_toml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to TOML file.
    pub fn to_toml(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    /// Default configuration for 2D plane.
    pub fn default_2d() -> Self {
        SimulationConfig {
            metadata: MetadataConfig {
                name: "Default 2D Synthetic Consciousness".to_string(),
                description: "Default 2D simulation with 10 entities".to_string(),
                version: "1.0.0".to_string(),
            },
            geometry: GeometryConfig {
                dimension: 2,
                bounds: vec![10.0, 10.0],
                periodic: true,
            },
            attraction: AttractionConfig {
                kernel: crate::attraction::KernelType::Gaussian,
                sigma: 1.0,
                lambda: 0.5,
            },
            state: StateConfig {
                memory_dim: 100,
                context_dim: 20,
                decay_alpha: 0.95,
                beta_attention: 0.5,
                gamma_memory: 0.3,
            },
            dynamics: DynamicsConfig {
                dt: 0.01,
                min_speed: 0.05,
                damping: 0.99,
            },
            essence: EssenceConfig {
                baseline: 5.0,
                decay: 0.1,
                experience_scale: 1.0,
            },
            simulation: SimulationParams {
                num_entities: 10,
                num_steps: 1000,
                dt: 0.01,
                seed: 42,
            },
        }
    }

    /// Default configuration for 3D space.
    pub fn default_3d() -> Self {
        let mut config = Self::default_2d();
        config.geometry.dimension = 3;
        config.geometry.bounds = vec![10.0, 10.0, 10.0];
        config.metadata.name = "Default 3D Synthetic Consciousness".to_string();
        config.metadata.description = "Default 3D simulation with 10 entities".to_string();
        config
    }

    /// Validate configuration parameters.
    pub fn validate(&self) -> Result<(), String> {
        if !self.geometry.is_valid() {
            return Err("Geometry configuration invalid".to_string());
        }

        if self.state.memory_dim == 0 || self.state.context_dim == 0 {
            return Err("State dimensions must be positive".to_string());
        }

        if self.dynamics.dt <= 0.0 || self.dynamics.min_speed < 0.0 {
            return Err("Dynamics parameters must be valid".to_string());
        }

        Ok(())
    }
}
