//! SMT Constraint Generation and Management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::engine::Mission;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMTConstraint {
    pub id: String,
    pub constraint_type: ConstraintType,
    pub expression: String,
    pub description: String,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstraintType {
    Safety,
    Performance,
    Resource,
    Temporal,
    DataFlow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstraintSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct ConstraintGenerator {
    pub safety_rules: Vec<String>,
    pub performance_bounds: HashMap<String, f64>,
    pub resource_limits: HashMap<String, u64>,
}

impl ConstraintGenerator {
    pub fn new() -> Self {
        Self {
            safety_rules: vec![
                "no_system_modification".to_string(),
                "no_network_access_without_permission".to_string(),
                "no_file_deletion_without_confirmation".to_string(),
            ],
            performance_bounds: HashMap::new(),
            resource_limits: HashMap::new(),
        }
    }
    
    pub fn generate_constraints(&self, mission: &Mission) -> Vec<SMTConstraint> {
        let mut constraints = Vec::new();
        
        // Generate safety constraints
        for (i, rule) in self.safety_rules.iter().enumerate() {
            constraints.push(SMTConstraint {
                id: format!("safety_{}", i),
                constraint_type: ConstraintType::Safety,
                expression: format!("(assert {})", rule),
                description: format!("Safety rule: {}", rule),
                severity: ConstraintSeverity::Critical,
            });
        }
        
        // Generate mission-specific constraints
        for step in &mission.steps {
            match step.step_type {
                crate::engine::StepType::Command => {
                    constraints.push(SMTConstraint {
                        id: format!("cmd_safety_{}", step.id),
                        constraint_type: ConstraintType::Safety,
                        expression: "(assert (not (contains dangerous_commands step_command)))".to_string(),
                        description: "Command must not be dangerous".to_string(),
                        severity: ConstraintSeverity::High,
                    });
                }
                crate::engine::StepType::CreateFile | 
                crate::engine::StepType::EditFile | 
                crate::engine::StepType::DeleteFile => {
                    constraints.push(SMTConstraint {
                        id: format!("file_safety_{}", step.id),
                        constraint_type: ConstraintType::Safety,
                        expression: "(assert (path_is_safe file_path))".to_string(),
                        description: "File operation must target safe paths".to_string(),
                        severity: ConstraintSeverity::High,
                    });
                }
                _ => {}
            }
        }
        
        constraints
    }
}
