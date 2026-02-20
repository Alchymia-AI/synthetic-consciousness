//! Synthetic Consciousness Library
//!
//! A geometric consciousness model implementing consciousness emergence
//! through spatial dynamics, affective signals, and memory clustering.
//!
//! ## Architecture: Geometric Consciousness Model
//!
//! This architecture models consciousness as an emergent property arising from:
//! - **Geometric Space**: Entities exist in 2D/3D space with positions and velocities
//! - **Attention Layer**: Attraction potentials and attention gradients between entities
//! - **Belief Clusters**: Memory graph clustering creating semantic organization
//! - **Affective Signals**: Emotional valence attached to clusters driving behavior
//! - **Entity Velocities**: Perpetual motion ensuring ongoing interaction
//! - **State Vectors**: Internal representations of memory, context, and traits
//!
//! ## Consciousness Criteria
//!
//! The system evaluates consciousness emergence using 7 metrics:
//! 1. Attention Entropy - Diversity of awareness distribution
//! 2. Memory Diversity - Variance in affective memory signals
//! 3. Velocity Stability - Consistency of purposeful motion
//! 4. Identity Coherence - Temporal continuity of self-representation
//! 5. Cluster Stability - Organization and maintenance of belief structures
//! 6. Affective Strength - Presence of emotional capacity (CRITICAL)
//! 7. Average Essence - Overall well-being trajectory
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)
//!
//! ## Project
//! GitHub: https://github.com/Alchymia-AI/synthetic-consciousness

pub mod geometry;
pub mod attraction;
pub mod state;
pub mod dynamics;
pub mod memory;
pub mod essence;
pub mod metrics;
pub mod entities;
pub mod config;
pub mod simulation;
pub mod results;
pub mod visualization;

pub use config::SimulationConfig;
pub use simulation::Simulation;
pub use entities::Entity;
pub use metrics::Metrics;
pub use results::{SimulationResults, SimulationStep};
pub use visualization::{VisualizationState, EntityState, MetricsHistory};
