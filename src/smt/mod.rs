//! SMT Verification Gates using ContextLite Z3 Integration
//! 
//! Formal verification for mission safety and correctness

pub mod constraints;
pub mod solver;
pub mod verification;
pub mod contextlite_bridge;
pub mod standards_compliance;
pub mod compliance_engine;
pub mod customer_tools;
pub mod oscal_converter;

pub use constraints::*;
pub use solver::*;
pub use verification::*;
pub use contextlite_bridge::*;
pub use standards_compliance::*;
pub use compliance_engine::*;
pub use customer_tools::*;
pub use oscal_converter::*;

/// SMT verification configuration
#[derive(Debug, Clone)]
pub struct SMTConfig {
    pub enabled: bool,
    pub solver_timeout_ms: u32,
    pub cache_enabled: bool,
    pub cache_size: usize,
    pub contextlite_endpoint: String,
}

impl Default for SMTConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            solver_timeout_ms: 5000,
            cache_enabled: true,
            cache_size: 1000,
            contextlite_endpoint: "http://localhost:8080".to_string(),
        }
    }
}
