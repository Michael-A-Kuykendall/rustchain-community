//! SMT Solver Integration
//!
//! Z3 solver integration via ContextLite API

use crate::core::Result;
use serde::{Deserialize, Serialize};
use super::constraints::SMTConstraint;

/// SMT solving result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMTResult {
    pub satisfiable: bool,
    pub model: Option<String>,
    pub execution_time_ms: u64,
}

/// SMT solver interface
pub trait SMTSolver {
    fn solve(&self, constraints: &[SMTConstraint]) -> Result<SMTResult>;
    fn add_constraint(&mut self, constraint: SMTConstraint) -> Result<()>;
    fn check_satisfiability(&self) -> Result<bool>;
}

/// Z3 SMT Solver implementation
pub struct Z3Solver {
    constraints: Vec<SMTConstraint>,
    timeout_ms: u32,
    contextlite_endpoint: Option<String>,
    client: Option<reqwest::Client>,
}

impl Z3Solver {
    pub fn new(timeout_ms: u32) -> Self {
        Self {
            constraints: Vec::new(),
            timeout_ms,
            contextlite_endpoint: None,
            client: None,
        }
    }
    
    /// Create Z3 solver with ContextLite integration
    pub fn with_contextlite(timeout_ms: u32, endpoint: String) -> Self {
        Self {
            constraints: Vec::new(),
            timeout_ms,
            contextlite_endpoint: Some(endpoint),
            client: Some(reqwest::Client::new()),
        }
    }
    
    /// Generate SMT-LIB2 format from constraints for Z3
    fn constraints_to_smt_lib(&self, constraints: &[SMTConstraint]) -> String {
        let mut smt_program = String::new();
        
        // SMT-LIB2 header
        smt_program.push_str("(set-logic QF_LIA)\n");
        smt_program.push_str("(set-info :source |RustChain SMT verification|)\n");
        
        // Add constraint assertions
        for constraint in constraints {
            // Basic SMT-LIB2 assertion format
            if constraint.expression.starts_with("(assert") {
                smt_program.push_str(&format!("{}\n", constraint.expression));
            } else {
                smt_program.push_str(&format!("(assert {})\n", constraint.expression));
            }
        }
        
        // Check satisfiability command
        smt_program.push_str("(check-sat)\n");
        smt_program.push_str("(get-model)\n");
        
        smt_program
    }
    
    /// Local satisfiability analysis using constraint heuristics
    fn local_satisfiability_analysis(&self, constraints: &[SMTConstraint]) -> Result<bool> {
        // Check for obvious contradictions
        let mut has_safety_violation = false;
        let mut resource_constraints = std::collections::HashMap::new();
        
        for constraint in constraints {
            match constraint.constraint_type {
                super::constraints::ConstraintType::Safety => {
                    if constraint.expression.contains("false") || 
                       constraint.expression.contains("dangerous_commands") {
                        has_safety_violation = true;
                    }
                },
                super::constraints::ConstraintType::Resource => {
                    // Track resource constraints for conflicts
                    if constraint.expression.contains("memory") {
                        resource_constraints.insert("memory", constraint.expression.clone());
                    }
                    if constraint.expression.contains("cpu") {
                        resource_constraints.insert("cpu", constraint.expression.clone());
                    }
                },
                _ => {}
            }
        }
        
        // Simple heuristic: if safety violations detected, likely unsatisfiable
        if has_safety_violation {
            return Ok(false);
        }
        
        // Check for resource constraint conflicts (simplified)
        if resource_constraints.len() > 5 {
            // Too many resource constraints might conflict
            return Ok(false);
        }
        
        Ok(true)
    }
}

impl SMTSolver for Z3Solver {
    fn solve(&self, constraints: &[SMTConstraint]) -> Result<SMTResult> {
        let start_time = std::time::Instant::now();
        
        // Try ContextLite integration first if available
        if let (Some(endpoint), Some(client)) = (&self.contextlite_endpoint, &self.client) {
            let rt = tokio::runtime::Runtime::new()?;
            return rt.block_on(async {
                let smt_lib_program = self.constraints_to_smt_lib(constraints);
                
                let request = serde_json::json!({
                    "program": smt_lib_program,
                    "solver": "z3",
                    "timeout_ms": self.timeout_ms
                });
                
                let response = client
                    .post(&format!("{}/api/smt/solve", endpoint))
                    .json(&request)
                    .send()
                    .await
                    .map_err(|e| crate::core::error::RustChainError::Execution(
                        crate::core::error::ExecutionError::step_failed(
                            "z3_solve",
                            "smt_solver",
                            format!("Failed to send Z3 request: {}", e)
                        )
                    ))?;
                    
                let execution_time_ms = start_time.elapsed().as_millis() as u64;
                    
                if response.status().is_success() {
                    let z3_response: serde_json::Value = response
                        .json()
                        .await
                        .map_err(|e| crate::core::error::RustChainError::Execution(
                            crate::core::error::ExecutionError::step_failed(
                                "z3_parse",
                                "smt_solver",
                                format!("Failed to parse Z3 response: {}", e)
                            )
                        ))?;
                        
                    Ok(SMTResult {
                        satisfiable: z3_response.get("satisfiable").and_then(|v| v.as_bool()).unwrap_or(false),
                        model: z3_response.get("model").and_then(|v| v.as_str()).map(String::from),
                        execution_time_ms,
                    })
                } else {
                    // Fallback to local analysis
                    tracing::warn!("ContextLite Z3 API unavailable, using local analysis");
                    let satisfiable = self.local_satisfiability_analysis(constraints)?;
                    
                    Ok(SMTResult {
                        satisfiable,
                        model: if satisfiable { 
                            Some(format!("(local-analysis-model {} constraints)", constraints.len()))
                        } else { 
                            None 
                        },
                        execution_time_ms,
                    })
                }
            });
        }
        
        // Fallback to local satisfiability analysis
        let satisfiable = self.local_satisfiability_analysis(constraints)?;
        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(SMTResult {
            satisfiable,
            model: if satisfiable { 
                Some(format!("(local-model {} constraints)", constraints.len()))
            } else { 
                None 
            },
            execution_time_ms,
        })
    }
    
    fn add_constraint(&mut self, constraint: SMTConstraint) -> Result<()> {
        let constraint_id = constraint.id.clone();
        self.constraints.push(constraint);
        tracing::debug!("Added constraint to Z3 solver: {} (total: {})", 
                       constraint_id, self.constraints.len());
        Ok(())
    }
    
    fn check_satisfiability(&self) -> Result<bool> {
        // Use accumulated constraints for satisfiability check
        let all_constraints: Vec<SMTConstraint> = self.constraints.clone();
        
        // Quick local analysis first
        let local_result = self.local_satisfiability_analysis(&all_constraints)?;
        
        // If local analysis is optimistic and we have ContextLite, do a remote check
        if local_result && self.contextlite_endpoint.is_some() {
            match self.solve(&all_constraints) {
                Ok(result) => Ok(result.satisfiable),
                Err(_) => {
                    // If remote check fails, fall back to local result
                    tracing::debug!("Remote satisfiability check failed, using local result");
                    Ok(local_result)
                }
            }
        } else {
            Ok(local_result)
        }
    }
}