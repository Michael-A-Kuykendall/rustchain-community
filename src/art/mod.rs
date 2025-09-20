//! Agent Reinforcement Training (ART) System
//! 
//! Self-improving AI agents through reinforcement learning
//! 
//! This module implements a complete ART system inspired by OpenPipe's approach,
//! retrofitted for RustChain with personal model integration.

#[cfg(feature = "art")]
pub mod trajectory;

#[cfg(feature = "art")]
pub mod performance;

#[cfg(feature = "art")]
pub mod training;

#[cfg(feature = "art")]
pub mod session;

#[cfg(feature = "art")]
pub mod ruler;

#[cfg(feature = "art")]
pub use trajectory::*;

#[cfg(feature = "art")]
pub use performance::*;

#[cfg(feature = "art")]
pub use training::*;

#[cfg(feature = "art")]
pub use session::*;

#[cfg(feature = "art")]
pub use ruler::*;

/// ART system configuration
#[derive(Debug, Clone)]
pub struct ARTConfig {
    pub enabled: bool,
    pub trajectory_storage_path: String,
    pub training_data_path: String,
    pub model_registry_path: String,
    pub performance_threshold: f64,
}

impl Default for ARTConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            trajectory_storage_path: "./art_data/trajectories".to_string(),
            training_data_path: "./art_data/training".to_string(),
            model_registry_path: "./art_data/models".to_string(),
            performance_threshold: 0.7,
        }
    }
}
