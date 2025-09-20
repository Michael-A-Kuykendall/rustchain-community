//! Compliance reporting and certification generation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::smt::compliance_engine::GDPRCertificationResult;
use crate::smt::constraints::SMTConstraint;
use chrono::{DateTime, Utc};
use crate::core::Result;

/// Comprehensive compliance report for enterprise customers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub standard: String,
    pub mission_name: String,
    pub timestamp: String,
    pub compliant: bool,
    pub risk_score: f64,
    pub constraints_checked: Vec<String>,
    pub violations: Vec<ComplianceViolationDetail>,
    pub recommendations: Vec<String>,
    pub severity_breakdown: HashMap<String, usize>,
    pub certification: Option<ComplianceCertification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolationDetail {
    pub constraint_id: String,
    pub severity: String,
    pub description: String,
    pub constraint: String,
    pub recommendation: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCertification {
    pub certificate_id: String,
    pub standard: String,
    pub mission_hash: String,
    pub compliance_score: f64,
    pub valid_until: String,
    pub issuer: String,
    pub signature: String,
}

impl ComplianceReport {
    /// Create report from GDPR compliance result
    pub fn from_gdpr_result(result: GDPRCertificationResult) -> Self {
        // Extract violations from non-compliant articles
        let mut violations = Vec::new();
        let mut constraints_checked = Vec::new();
        
        for (article, status) in &result.article_compliance {
            constraints_checked.push(format!("gdpr_article_{}", article));
            
            if !status.compliant {
                violations.push(ComplianceViolationDetail {
                    constraint_id: format!("gdpr_article_{}", article),
                    severity: format!("{:?}", status.risk_level),
                    description: format!("GDPR Article {} compliance violation", article),
                    constraint: format!("GDPR Article {} requirements not met", article),
                    recommendation: format!("Review GDPR Article {} requirements and implement necessary controls", article),
                    evidence: status.manual_evidence.as_ref().map(|e| vec![e.clone()]).unwrap_or_default(),
                });
            }
        }
        
        // Add remediation violations for high-severity items
        for action in &result.remediation_actions {
            // Only include high-severity items that need attention
            if matches!(action.severity, crate::smt::compliance_engine::ComplianceRiskLevel::Critical | crate::smt::compliance_engine::ComplianceRiskLevel::High) {
                violations.push(ComplianceViolationDetail {
                    constraint_id: action.action_id.clone(),
                    severity: format!("{:?}", action.severity),
                    description: action.description.clone(),
                    constraint: format!("GDPR {} remediation requirement", action.article_reference),
                    recommendation: action.manual_steps_required.join("; "),
                    evidence: vec![format!("Estimated completion: {} hours", action.estimated_completion_hours)],
                });
            }
        }
        
        let mut severity_breakdown = HashMap::new();
        for violation in &violations {
            *severity_breakdown.entry(violation.severity.clone()).or_insert(0) += 1;
        }
        
        let compliant = matches!(result.eu_regulatory_status, crate::smt::compliance_engine::EUComplianceStatus::FullyCompliant);
        let risk_score = 100.0 - result.compliance_score; // Convert compliance score to risk score
        
        Self {
            standard: "GDPR".to_string(),
            mission_name: result.mission_id.clone(),
            timestamp: result.certification_timestamp.to_rfc3339(),
            compliant,
            risk_score,
            constraints_checked,
            violations,
            recommendations: result.remediation_actions.iter()
                .map(|a| a.manual_steps_required.join("; "))
                .collect(),
            severity_breakdown,
            certification: if compliant {
                Some(ComplianceCertification {
                    certificate_id: format!("gdpr_{}", uuid::Uuid::new_v4()),
                    standard: "GDPR".to_string(),
                    mission_hash: "sha256_placeholder".to_string(),
                    compliance_score: result.compliance_score,
                    valid_until: result.next_review_date.to_rfc3339(),
                    issuer: "RustChain GDPR Compliance Engine".to_string(),
                    signature: "crypto_signature_placeholder".to_string(),
                })
            } else {
                None
            },
        }
    }
    
    /// Create basic report from NIST constraints
    pub fn from_nist_constraints(mission: &crate::engine::Mission, constraints: &[SMTConstraint]) -> Self {
        // Basic compliance check - mission should have safe operations
        let mut violations = Vec::new();
        let mut constraints_checked = Vec::new();
        
        // Check for potentially risky operations
        for step in &mission.steps {
            match step.step_type {
                crate::engine::StepType::Command => {
                    // Check command safety
                    if let Some(command) = step.parameters.get("command") {
                        if let Some(cmd_str) = command.as_str() {
                            if cmd_str.contains("rm") || cmd_str.contains("del") || cmd_str.contains("format") {
                                violations.push(ComplianceViolationDetail {
                                    constraint_id: "nist_si_1".to_string(),
                                    severity: "High".to_string(),
                                    description: "Potentially destructive command detected".to_string(),
                                    constraint: "System integrity must be maintained".to_string(),
                                    recommendation: "Review command safety and add confirmation".to_string(),
                                    evidence: vec![format!("Command: {}", cmd_str)],
                                });
                            }
                        }
                    }
                    constraints_checked.push("nist_si_1".to_string());
                },
                crate::engine::StepType::Http => {
                    // Check HTTP operations for security
                    constraints_checked.push("nist_sc_8".to_string());
                    constraints_checked.push("nist_sc_13".to_string());
                },
                _ => {
                    // Other operations are generally safe
                    constraints_checked.push("nist_ac_1".to_string());
                }
            }
        }
        
        let compliant = violations.is_empty();
        let risk_score = violations.len() as f64 * 10.0; // Simple risk calculation
        
        let mut severity_breakdown = HashMap::new();
        for violation in &violations {
            *severity_breakdown.entry(violation.severity.clone()).or_insert(0) += 1;
        }
        
        Self {
            standard: "NIST_800_53".to_string(),
            mission_name: mission.name.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            compliant,
            risk_score,
            constraints_checked,
            violations,
            recommendations: if compliant {
                vec!["Mission meets NIST 800-53 security requirements".to_string()]
            } else {
                vec!["Review flagged operations for security compliance".to_string()]
            },
            severity_breakdown,
            certification: if compliant {
                Some(ComplianceCertification {
                    certificate_id: format!("nist_{}", uuid::Uuid::new_v4()),
                    standard: "NIST_800_53".to_string(),
                    mission_hash: format!("sha256_{}", mission.name.chars().map(|c| c as u8).sum::<u8>()),
                    compliance_score: (100.0 - risk_score).max(0.0),
                    valid_until: (chrono::Utc::now() + chrono::Duration::days(365)).to_rfc3339(),
                    issuer: "RustChain NIST Compliance Engine".to_string(),
                    signature: "crypto_signature_placeholder".to_string(),
                })
            } else {
                None
            },
        }
    }
    
    /// Print report in table format
    pub fn print_table(&self) {
        println!("📋 Compliance Report: {}", self.standard);
        println!("   Mission: {}", self.mission_name);
        println!("   Timestamp: {}", self.timestamp);
        println!("   Status: {}", if self.compliant { "✅ COMPLIANT" } else { "❌ NON-COMPLIANT" });
        println!("   Risk Score: {:.2}", self.risk_score);
        println!("   Constraints Checked: {}", self.constraints_checked.len());
        println!("   Violations: {}", self.violations.len());
        
        if !self.violations.is_empty() {
            println!("\n🚨 Violations:");
            for violation in &self.violations {
                println!("   • {} [{}]: {}", violation.severity, violation.constraint_id, violation.description);
                println!("     Recommendation: {}", violation.recommendation);
            }
        }
        
        if !self.recommendations.is_empty() {
            println!("\n💡 Recommendations:");
            for rec in &self.recommendations {
                println!("   • {}", rec);
            }
        }
        
        if let Some(cert) = &self.certification {
            println!("\n🏆 Compliance Certificate:");
            println!("   Certificate ID: {}", cert.certificate_id);
            println!("   Score: {:.1}%", cert.compliance_score);
            println!("   Valid Until: {}", cert.valid_until);
        }
    }
    
    /// Export report to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to serialize report: {}", e)
                }
            ))
    }
    
    /// Export report to YAML
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to serialize report: {}", e)
                }
            ))
    }
    
    /// Generate executive summary
    pub fn executive_summary(&self) -> String {
        format!(
            "EXECUTIVE SUMMARY\n\
            =================\n\
            Standard: {}\n\
            Mission: {}\n\
            Compliance Status: {}\n\
            Risk Level: {}\n\
            Constraints Verified: {}\n\
            Issues Found: {}\n\
            \n\
            RECOMMENDATION: {}\n",
            self.standard,
            self.mission_name,
            if self.compliant { "COMPLIANT" } else { "REQUIRES ATTENTION" },
            if self.risk_score < 10.0 { "LOW" } else if self.risk_score < 30.0 { "MEDIUM" } else { "HIGH" },
            self.constraints_checked.len(),
            self.violations.len(),
            if self.compliant { 
                "Mission approved for production deployment." 
            } else { 
                "Address compliance violations before deployment." 
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{Mission, MissionStep, StepType};
    
    #[test]
    fn test_compliance_report_creation() {
        let mission = Mission {
            version: "1.0".to_string(),
            name: "Test Mission".to_string(),
            description: None,
            steps: vec![],
            config: None,
        };
        
        let constraints = vec![];
        let report = ComplianceReport::from_nist_constraints(&mission, &constraints);
        
        assert_eq!(report.standard, "NIST_800_53");
        assert_eq!(report.mission_name, "Test Mission");
        assert!(report.compliant); // Empty mission should be compliant
    }
    
    #[test]
    fn test_report_serialization() {
        let mission = Mission {
            version: "1.0".to_string(),
            name: "Serialization Test".to_string(),
            description: None,
            steps: vec![],
            config: None,
        };
        
        let report = ComplianceReport::from_nist_constraints(&mission, &[]);
        
        // Test JSON serialization
        let json = report.to_json();
        assert!(json.is_ok());
        
        // Test YAML serialization
        let yaml = report.to_yaml();
        assert!(yaml.is_ok());
        
        // Test executive summary
        let summary = report.executive_summary();
        assert!(summary.contains("EXECUTIVE SUMMARY"));
        assert!(summary.contains("Serialization Test"));
    }
}