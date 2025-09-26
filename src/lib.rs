#[cfg(feature = "llm")]
pub mod llm;

#[cfg(feature = "tools")]
pub mod tools;

#[cfg(feature = "rag")]
pub mod rag;

#[cfg(feature = "memory")]
pub mod memory;

#[cfg(feature = "chain")]
pub mod chain;

#[cfg(feature = "agent")]
pub mod agent;

#[cfg(feature = "art")]
pub mod art;

#[cfg(feature = "smt")]
pub mod smt;

#[cfg(feature = "compliance")]
pub mod compliance;

// compliance_sdk removed - now self-contained in compliance module

#[cfg(feature = "sandbox")]
pub mod sandbox;

// Registry module (community features + enterprise extensions)
#[cfg(any(feature = "registry", feature = "enterprise"))]
pub mod registry;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "concurrency")]
pub mod concurrency;

#[cfg(feature = "invariants")]
pub mod testing;

pub mod invariant_ppt;

// Universal Transpiler System
#[cfg(feature = "transpiler")]
pub mod transpiler;

// Enterprise features (gated by enterprise feature)
#[cfg(feature = "enterprise")]
pub mod security;

// Core modules always available
pub mod core;
pub mod engine;
pub mod policy;
pub mod runtime;
pub mod safety;
pub mod telemetry;
pub mod validation;
pub mod performance;
pub mod build_dashboard;
pub mod benchmarks;
// Note: Some enterprise modules may be conditionally available
// - security (available with enterprise feature)
// - visual (moved to rustchain-enterprise)
// - registry (available with registry/enterprise features)
