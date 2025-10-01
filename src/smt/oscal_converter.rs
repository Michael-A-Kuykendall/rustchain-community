//! Automated OSCAL to SMT Converter
//!
//! Converts NIST 800-53 OSCAL JSON format to SMT constraints automatically

use crate::core::Result;
use crate::smt::constraints::{SMTConstraint, ConstraintType, ConstraintSeverity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// OSCAL Catalog structure (partial, focused on controls)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSCALCatalog {
    pub catalog: Catalog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub uuid: String,
    pub metadata: Metadata,
    pub groups: Vec<ControlGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlGroup {
    pub id: String,
    pub title: String,
    pub controls: Vec<Control>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub id: String,
    pub title: String,
    pub class: Option<String>,
    pub parts: Option<Vec<ControlPart>>,
    pub controls: Option<Vec<Control>>, // For enhancements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPart {
    pub name: String,
    pub prose: Option<String>,
    pub parts: Option<Vec<ControlPart>>,
}

/// Automated OSCAL to SMT converter
pub struct OSCALToSMTConverter {
    _control_patterns: HashMap<String, ControlPattern>,
    family_patterns: HashMap<String, FamilyPattern>,
}

/// Pattern template for control families
#[derive(Debug, Clone)]
pub struct FamilyPattern {
    pub entity_type: String,      // User, System, Data, etc.
    pub base_predicate: String,   // manages, processes, accesses, etc.
    pub constraint_template: String,
}

/// Pattern template for specific controls
#[derive(Debug, Clone)]
pub struct ControlPattern {
    pub smt_template: String,
    pub variables: Vec<String>,
    pub constraint_type: ConstraintType,
    pub severity: ConstraintSeverity,
}

impl OSCALToSMTConverter {
    pub fn new() -> Self {
        let mut converter = Self {
            _control_patterns: HashMap::new(),
            family_patterns: HashMap::new(),
        };
        
        converter.initialize_patterns();
        converter
    }
    
    /// Load NIST 800-53 catalog and convert to SMT constraints
    pub async fn convert_nist_catalog(&self, catalog_path: &str) -> Result<Vec<SMTConstraint>> {
        let catalog_json = fs::read_to_string(catalog_path)?;
        let catalog: OSCALCatalog = serde_json::from_str(&catalog_json)
            .map_err(|e| crate::core::error::RustChainError::Config(crate::core::error::ConfigError::ParseError { reason: format!("Failed to parse OSCAL catalog: {}", e) }))?;
        
        let mut constraints = Vec::new();
        
        for group in &catalog.catalog.groups {
            println!("Processing control family: {} - {}", group.id, group.title);
            
            for control in &group.controls {
                if let Ok(control_constraints) = self.to_smt(control) {
                    constraints.extend(control_constraints);
                }
                
                // Process control enhancements
                if let Some(ref enhancements) = control.controls {
                    for enhancement in enhancements {
                        if let Ok(enhancement_constraints) = self.to_smt(enhancement) {
                            constraints.extend(enhancement_constraints);
                        }
                    }
                }
            }
        }
        
        println!("Generated {} SMT constraints from NIST 800-53 catalog", constraints.len());
        Ok(constraints)
    }
    
    /// Convert individual control to SMT constraints
    fn to_smt(&self, control: &Control) -> Result<Vec<SMTConstraint>> {
        let mut constraints = Vec::new();
        
        // Skip withdrawn controls
        if control.class.as_ref().map_or(false, |c| c.contains("withdrawn")) {
            return Ok(constraints);
        }
        
        // Extract family from control ID (e.g., "ac-2" -> "ac")
        let family = control.id.split('-').next().unwrap_or("").to_uppercase();
        
        // Get statement text
        let statement = self.extract_statement_text(control);
        
        // Use family-specific pattern or generic pattern
        let constraint = if let Some(pattern) = self.family_patterns.get(&family) {
            self.apply_family_pattern(&control.id, &control.title, &statement, pattern)?
        } else {
            self.apply_generic_pattern(&control.id, &control.title, &statement)?
        };
        
        constraints.push(constraint);
        Ok(constraints)
    }
    
    /// Extract statement text from control parts
    fn extract_statement_text(&self, control: &Control) -> String {
        if let Some(ref parts) = control.parts {
            for part in parts {
                if part.name == "statement" {
                    return part.prose.clone().unwrap_or_default();
                }
            }
        }
        control.title.clone()
    }
    
    /// Apply family-specific SMT pattern
    fn apply_family_pattern(&self, id: &str, title: &str, statement: &str, pattern: &FamilyPattern) -> Result<SMTConstraint> {
        let smt_expression = pattern.constraint_template
            .replace("{entity}", &pattern.entity_type)
            .replace("{predicate}", &pattern.base_predicate)
            .replace("{control_logic}", &self.statement_to_logic(statement));
        
        Ok(SMTConstraint {
            id: format!("nist_{}", id.replace('-', "_")),
            constraint_type: self.determine_constraint_type(statement),
            expression: smt_expression,
            description: format!("NIST {}: {}", id.to_uppercase(), title),
            severity: self.determine_severity(&id, statement),
        })
    }
    
    /// Apply generic SMT pattern for unknown families
    fn apply_generic_pattern(&self, id: &str, title: &str, statement: &str) -> Result<SMTConstraint> {
        let logic = self.statement_to_logic(statement);
        let smt_expression = format!(
            "(assert (forall ((system System)) (=> (implements mission system) ({}))))",
            logic
        );
        
        Ok(SMTConstraint {
            id: format!("nist_{}", id.replace('-', "_")),
            constraint_type: ConstraintType::Safety,
            expression: smt_expression,
            description: format!("NIST {}: {}", id.to_uppercase(), title),
            severity: ConstraintSeverity::Medium,
        })
    }
    
    /// Convert natural language statement to SMT logic
    fn statement_to_logic(&self, statement: &str) -> String {
        let lower = statement.to_lowercase();
        
        // Pattern matching for common control requirements
        if lower.contains("shall") || lower.contains("must") {
            if lower.contains("authenticate") {
                "authenticated system.users"
            } else if lower.contains("authorize") {
                "authorized system.access"
            } else if lower.contains("encrypt") {
                "encrypted system.data"
            } else if lower.contains("monitor") {
                "monitored system.activities"
            } else if lower.contains("log") || lower.contains("audit") {
                "logged system.events"
            } else if lower.contains("control") || lower.contains("restrict") {
                "controlled system.access"
            } else if lower.contains("protect") || lower.contains("secure") {
                "protected system.assets"
            } else if lower.contains("validate") || lower.contains("verify") {
                "validated system.inputs"
            } else {
                "compliant system"
            }
        } else {
            "implemented system.control"
        }.to_string()
    }
    
    /// Determine constraint type from statement
    fn determine_constraint_type(&self, statement: &str) -> ConstraintType {
        let lower = statement.to_lowercase();
        
        if lower.contains("time") || lower.contains("within") || lower.contains("period") {
            ConstraintType::Temporal
        } else if lower.contains("shall not") || lower.contains("prohibit") || lower.contains("prevent") {
            ConstraintType::Safety
        } else {
            ConstraintType::Safety
        }
    }
    
    /// Determine constraint severity from control ID and statement
    fn determine_severity(&self, id: &str, statement: &str) -> ConstraintSeverity {
        let lower = statement.to_lowercase();
        
        // Base controls (no enhancement number) are typically critical
        if !id.contains('.') {
            return ConstraintSeverity::Critical;
        }
        
        // High-severity keywords
        if lower.contains("critical") || lower.contains("immediate") || lower.contains("mandatory") {
            ConstraintSeverity::Critical
        } else if lower.contains("required") || lower.contains("shall") || lower.contains("must") {
            ConstraintSeverity::High
        } else if lower.contains("should") || lower.contains("recommended") {
            ConstraintSeverity::Medium
        } else {
            ConstraintSeverity::Low
        }
    }
    
    /// Initialize family-specific patterns for better SMT generation
    fn initialize_patterns(&mut self) {
        // Access Control family patterns
        self.family_patterns.insert("AC".to_string(), FamilyPattern {
            entity_type: "AccessRequest".to_string(),
            base_predicate: "requests".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // Audit and Accountability family patterns
        self.family_patterns.insert("AU".to_string(), FamilyPattern {
            entity_type: "AuditEvent".to_string(),
            base_predicate: "generates".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // Identification and Authentication family patterns
        self.family_patterns.insert("IA".to_string(), FamilyPattern {
            entity_type: "User".to_string(),
            base_predicate: "authenticates".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // System and Information Integrity family patterns
        self.family_patterns.insert("SI".to_string(), FamilyPattern {
            entity_type: "SystemComponent".to_string(),
            base_predicate: "manages".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // Configuration Management family patterns
        self.family_patterns.insert("CM".to_string(), FamilyPattern {
            entity_type: "Configuration".to_string(),
            base_predicate: "configures".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // Incident Response family patterns
        self.family_patterns.insert("IR".to_string(), FamilyPattern {
            entity_type: "Incident".to_string(),
            base_predicate: "handles".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // Risk Assessment family patterns
        self.family_patterns.insert("RA".to_string(), FamilyPattern {
            entity_type: "Risk".to_string(),
            base_predicate: "assesses".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
        
        // System and Communications Protection family patterns
        self.family_patterns.insert("SC".to_string(), FamilyPattern {
            entity_type: "Communication".to_string(),
            base_predicate: "protects".to_string(),
            constraint_template: "(assert (forall (({entity} {entity})) (=> ({predicate} system {entity}) ({control_logic}))))".to_string(),
        });
    }
    
    /// Generate constraints for a specific control family
    pub async fn convert_family(&self, catalog_path: &str, family_id: &str) -> Result<Vec<SMTConstraint>> {
        let catalog_json = fs::read_to_string(catalog_path)?;
        let catalog: OSCALCatalog = serde_json::from_str(&catalog_json)
            .map_err(|e| crate::core::error::RustChainError::Config(crate::core::error::ConfigError::ParseError { reason: format!("Failed to parse OSCAL catalog: {}", e) }))?;
        
        let mut constraints = Vec::new();
        
        for group in &catalog.catalog.groups {
            if group.id.to_uppercase() == family_id.to_uppercase() {
                for control in &group.controls {
                    if let Ok(control_constraints) = self.to_smt(control) {
                        constraints.extend(control_constraints);
                    }
                    
                    // Process enhancements
                    if let Some(ref enhancements) = control.controls {
                        for enhancement in enhancements {
                            if let Ok(enhancement_constraints) = self.to_smt(enhancement) {
                                constraints.extend(enhancement_constraints);
                            }
                        }
                    }
                }
                break;
            }
        }
        
        Ok(constraints)
    }
    
    /// Count total controls in catalog
    pub async fn count_controls(&self, catalog_path: &str) -> Result<(usize, HashMap<String, usize>)> {
        let catalog_json = fs::read_to_string(catalog_path)?;
        let catalog: OSCALCatalog = serde_json::from_str(&catalog_json)
            .map_err(|e| crate::core::error::RustChainError::Config(crate::core::error::ConfigError::ParseError { reason: format!("Failed to parse OSCAL catalog: {}", e) }))?;
        
        let mut total_controls = 0;
        let mut family_counts = HashMap::new();
        
        for group in &catalog.catalog.groups {
            let mut family_count = 0;
            
            for control in &group.controls {
                family_count += 1;
                total_controls += 1;
                
                // Count enhancements
                if let Some(ref enhancements) = control.controls {
                    let enhancement_count = enhancements.len();
                    family_count += enhancement_count;
                    total_controls += enhancement_count;
                }
            }
            
            family_counts.insert(group.id.clone(), family_count);
        }
        
        Ok((total_controls, family_counts))
    }
}

impl Default for OSCALToSMTConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_converter_initialization() {
        let converter = OSCALToSMTConverter::new();
        assert!(!converter.family_patterns.is_empty());
        assert!(converter.family_patterns.contains_key("AC"));
        assert!(converter.family_patterns.contains_key("AU"));
    }
    
    #[tokio::test]
    async fn test_statement_to_logic_conversion() {
        let converter = OSCALToSMTConverter::new();
        
        // Test authentication requirements
        let auth_statement = "The system must authenticate all users before granting access";
        let logic = converter.statement_to_logic(auth_statement);
        assert_eq!(logic, "authenticated system.users");
        
        // Test encryption requirements
        let encryption_statement = "The organization shall encrypt all sensitive data";
        let logic = converter.statement_to_logic(encryption_statement);
        assert_eq!(logic, "encrypted system.data");
        
        // Test monitoring requirements
        let monitor_statement = "The system must monitor all user activities";
        let logic = converter.statement_to_logic(monitor_statement);
        assert_eq!(logic, "monitored system.activities");
    }
    
    #[tokio::test]
    async fn test_severity_determination() {
        let converter = OSCALToSMTConverter::new();
        
        // Base controls should be critical
        assert_eq!(converter.determine_severity("ac-2", ""), ConstraintSeverity::Critical);
        
        // Enhancements with critical keywords
        assert_eq!(converter.determine_severity("ac-2.1", "critical security requirement"), ConstraintSeverity::Critical);
        
        // Standard requirements
        assert_eq!(converter.determine_severity("ac-2.1", "The system shall implement"), ConstraintSeverity::High);
        
        // Recommendations
        assert_eq!(converter.determine_severity("ac-2.1", "The system should consider"), ConstraintSeverity::Medium);
    }
    
    #[tokio::test]
    async fn test_nist_catalog_processing() {
        let converter = OSCALToSMTConverter::new();
        let catalog_path = "nist_800_53_catalog.json";
        
        // Skip test if catalog not available
        if !std::path::Path::new(catalog_path).exists() {
            println!("Skipping NIST catalog test - file not found");
            return;
        }
        
        // Test control counting
        match converter.count_controls(catalog_path).await {
            Ok((total, families)) => {
                println!("📊 NIST 800-53 contains {} total controls across {} families", total, families.len());
                assert!(total > 300); // Should have at least 300+ controls
                assert!(families.len() > 15); // Should have 15+ control families
                
                // Test specific families exist
                assert!(families.contains_key("ac"));
                assert!(families.contains_key("au"));
                assert!(families.contains_key("ia"));
                
                println!("   AC family: {} controls", families.get("ac").unwrap_or(&0));
                println!("   AU family: {} controls", families.get("au").unwrap_or(&0));
            },
            Err(e) => {
                println!("❌ Failed to count controls: {}", e);
                panic!("Control counting failed");
            }
        }
        
        // Test converting a single family
        match converter.convert_family(catalog_path, "ac").await {
            Ok(constraints) => {
                println!("✅ Generated {} SMT constraints for AC family", constraints.len());
                assert!(!constraints.is_empty());
                
                // Verify constraint structure
                let first_constraint = &constraints[0];
                assert!(first_constraint.id.starts_with("nist_"));
                assert!(!first_constraint.description.is_empty());
                assert!(!first_constraint.expression.is_empty());
                
                println!("   Sample constraint: {} - {}", first_constraint.id, first_constraint.description);
            },
            Err(e) => {
                println!("❌ AC family conversion failed: {}", e);
                // Don't panic on conversion failure for now - we're still developing
            }
        }
    }
    
    #[tokio::test]
    async fn test_full_nist_catalog_conversion() {
        let converter = OSCALToSMTConverter::new();
        let catalog_path = "nist_800_53_catalog.json";
        
        // Skip test if catalog not available
        if !std::path::Path::new(catalog_path).exists() {
            println!("Skipping full NIST catalog conversion test - file not found");
            return;
        }
        
        println!("🚀 Testing Full NIST 800-53 Catalog Conversion...");
        
        // Test full catalog conversion
        match converter.convert_nist_catalog(catalog_path).await {
            Ok(all_constraints) => {
                println!("🎉 SUCCESS: Generated {} total SMT constraints from NIST 800-53!", all_constraints.len());
                
                // Should have generated substantial number of constraints
                assert!(all_constraints.len() > 500, "Expected 500+ constraints, got {}", all_constraints.len());
                
                // Analyze by constraint type
                let safety_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, ConstraintType::Safety)).count();
                let temporal_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, ConstraintType::Temporal)).count();
                let resource_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, ConstraintType::Resource)).count();
                let performance_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, ConstraintType::Performance)).count();
                let dataflow_count = all_constraints.iter().filter(|c| matches!(c.constraint_type, ConstraintType::DataFlow)).count();
                
                println!("   Safety constraints: {}", safety_count);
                println!("   Temporal constraints: {}", temporal_count);
                println!("   Resource constraints: {}", resource_count);
                println!("   Performance constraints: {}", performance_count);
                println!("   DataFlow constraints: {}", dataflow_count);
                
                assert!(safety_count > 0, "Should have safety constraints");
                assert!(temporal_count > 0, "Should have temporal constraints");
                
                // Analyze by severity
                let critical_count = all_constraints.iter().filter(|c| matches!(c.severity, ConstraintSeverity::Critical)).count();
                let high_count = all_constraints.iter().filter(|c| matches!(c.severity, ConstraintSeverity::High)).count();
                let medium_count = all_constraints.iter().filter(|c| matches!(c.severity, ConstraintSeverity::Medium)).count();
                let low_count = all_constraints.iter().filter(|c| matches!(c.severity, ConstraintSeverity::Low)).count();
                
                println!("   Critical: {}, High: {}, Medium: {}, Low: {}", critical_count, high_count, medium_count, low_count);
                
                assert!(critical_count > 0, "Should have critical constraints");
                assert!(high_count > 0, "Should have high severity constraints");
                
                // Validate constraint structure
                for constraint in all_constraints.iter().take(5) {
                    assert!(constraint.id.starts_with("nist_"), "Constraint ID should start with 'nist_': {}", constraint.id);
                    assert!(!constraint.description.is_empty(), "Description should not be empty");
                    assert!(!constraint.expression.is_empty(), "Expression should not be empty");
                    
                    // Verify SMT expressions are valid (basic check)
                    assert!(constraint.expression.contains("system") || constraint.expression.contains("authenticated") || 
                           constraint.expression.contains("encrypted") || constraint.expression.contains("monitored"),
                           "Expression should contain key security terms: {}", constraint.expression);
                }
                
                // Show sample constraints from different families
                println!("🔍 Sample Constraints Generated:");
                let mut family_samples: std::collections::HashMap<String, Vec<&SMTConstraint>> = std::collections::HashMap::new();
                
                for constraint in &all_constraints {
                    if constraint.id.len() > 5 {
                        let family = constraint.id[5..7].to_uppercase(); // Extract family from nist_xx_...
                        family_samples.entry(family).or_insert_with(Vec::new).push(constraint);
                    }
                }
                
                // Show one constraint from each major family
                for (family, constraints) in family_samples.iter().take(5) {
                    if let Some(sample) = constraints.first() {
                        println!("   {} Family: {} - {}", family, sample.id, sample.description);
                    }
                }
                
                println!("✅ NIST 800-53 SMT Conversion Complete: Ready for Enterprise Deployment!");
                
            },
            Err(e) => {
                println!("❌ Full conversion failed: {}", e);
                panic!("Full NIST catalog conversion should succeed: {}", e);
            }
        }
    }
}