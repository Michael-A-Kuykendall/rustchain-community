//! Mission Verification Workflows
//!
//! High-level verification orchestration using SMT solving

use crate::core::Result;
use crate::engine::Mission;
use serde::{Deserialize, Serialize};
use super::constraints::{ConstraintGenerator, SMTConstraint};
use super::solver::{SMTSolver, SMTResult, Z3Solver};

/// Verification result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub passed: bool,
    pub violations: Vec<String>,
    pub smt_result: Option<SMTResult>,
    pub verification_time_ms: u64,
}

/// Mission verifier trait
pub trait MissionVerifier {
    fn verify_safety(&self, mission: &Mission) -> Result<VerificationResult>;
    fn generate_constraints(&self, mission: &Mission) -> Result<Vec<SMTConstraint>>;
}

/// SMT-based mission verifier
pub struct SMTMissionVerifier {
    generator: ConstraintGenerator,
    solver: Box<dyn SMTSolver>,
}

impl SMTMissionVerifier {
    pub fn new() -> Self {
        Self {
            generator: ConstraintGenerator::new(),
            solver: Box::new(Z3Solver::new(5000)), // 5 second timeout
        }
    }
    
    pub fn with_solver(solver: Box<dyn SMTSolver>) -> Self {
        Self {
            generator: ConstraintGenerator::new(),
            solver,
        }
    }
}

impl MissionVerifier for SMTMissionVerifier {
    fn verify_safety(&self, mission: &Mission) -> Result<VerificationResult> {
        let start_time = std::time::Instant::now();
        
        // Generate constraints for the mission
        let constraints = self.generate_constraints(mission)?;
        
        // Solve constraints
        let smt_result = self.solver.solve(&constraints)?;
        
        let verification_time_ms = start_time.elapsed().as_millis() as u64;
        
        // Analyze result for violations
        let violations = if !smt_result.satisfiable {
            vec!["Mission constraints are unsatisfiable".to_string()]
        } else {
            Vec::new()
        };
        
        Ok(VerificationResult {
            passed: smt_result.satisfiable && violations.is_empty(),
            violations,
            smt_result: Some(smt_result),
            verification_time_ms,
        })
    }
    
    fn generate_constraints(&self, mission: &Mission) -> Result<Vec<SMTConstraint>> {
        Ok(self.generator.generate_constraints(mission))
    }
}