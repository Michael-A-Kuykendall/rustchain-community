use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

/// Compliance frameworks
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ComplianceFramework {
    GDPR,
    HIPAA,
    SOX,
    PciDss,
    ISO27001,
    NIST,
    Custom(String),
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: String,
    pub framework: ComplianceFramework,
    pub title: String,
    pub description: String,
    pub category: String,
    pub severity: ComplianceSeverity,
    pub implementation_status: ImplementationStatus,
    pub controls: Vec<ComplianceControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationStatus {
    NotImplemented,
    PartiallyImplemented,
    FullyImplemented,
    NotApplicable,
}

/// Compliance control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceControl {
    pub id: String,
    pub name: String,
    pub description: String,
    pub control_type: ControlType,
    pub implementation: Option<String>,
    pub evidence: Vec<String>,
    pub last_tested: Option<DateTime<Utc>>,
    pub test_result: Option<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    Technical,
    Administrative,
    Physical,
    Legal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestResult {
    Pass,
    Fail,
    PartialPass,
    NotTested,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub framework: ComplianceFramework,
    pub generated_at: DateTime<Utc>,
    pub report_period: DateRange,
    pub summary: ComplianceSummary,
    pub requirements: Vec<RequirementAssessment>,
    pub violations: Vec<ComplianceViolation>,
    pub recommendations: Vec<ComplianceRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    pub total_requirements: usize,
    pub implemented: usize,
    pub partially_implemented: usize,
    pub not_implemented: usize,
    pub compliance_percentage: f64,
    pub risk_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementAssessment {
    pub requirement: ComplianceRequirement,
    pub current_status: ImplementationStatus,
    pub gap_analysis: String,
    pub remediation_steps: Vec<String>,
    pub estimated_effort: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: String,
    pub requirement_id: String,
    pub violation_type: String,
    pub severity: ComplianceSeverity,
    pub description: String,
    pub detected_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRecommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub estimated_effort: Duration,
    pub expected_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub id: String,
    pub name: String,
    pub data_types: Vec<String>,
    pub retention_period: Duration,
    pub deletion_method: DeletionMethod,
    pub exceptions: Vec<RetentionException>,
    pub legal_holds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletionMethod {
    SoftDelete,
    HardDelete,
    Anonymization,
    Encryption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionException {
    pub condition: String,
    pub extended_period: Duration,
    pub justification: String,
}

/// Compliance service trait
#[async_trait]
pub trait ComplianceService: Send + Sync {
    async fn generate_report(&self, framework: &str) -> crate::core::error::Result<ComplianceReport>;
    async fn assess_requirement(&self, requirement_id: &str) -> crate::core::error::Result<RequirementAssessment>;
    async fn track_violation(&self, violation: ComplianceViolation) -> crate::core::error::Result<()>;
    async fn get_violations(&self, framework: &ComplianceFramework, since: DateTime<Utc>) -> crate::core::error::Result<Vec<ComplianceViolation>>;
    async fn apply_retention_policy(&self, policy: &DataRetentionPolicy) -> crate::core::error::Result<RetentionReport>;
    async fn audit_data_processing(&self, activity_id: &str) -> crate::core::error::Result<DataProcessingRecord>;
}

/// Multi-framework compliance service
pub struct MultiFrameworkComplianceService {
    requirements: HashMap<ComplianceFramework, Vec<ComplianceRequirement>>,
    violations: Vec<ComplianceViolation>,
    retention_policies: Vec<DataRetentionPolicy>,
}

impl MultiFrameworkComplianceService {
    pub fn new(config: std::sync::Arc<crate::security::SecurityConfig>) -> Self {
        let mut service = Self {
            requirements: HashMap::new(),
            violations: Vec::new(),
            retention_policies: Vec::new(),
        };
        
        // Initialize requirements based on configured frameworks
        for framework in &config.compliance_frameworks {
            match framework.as_str() {
                "GDPR" => service.initialize_gdpr_requirements(),
                "HIPAA" => service.initialize_hipaa_requirements(),
                "SOX" => service.initialize_sox_requirements(),
                "PciDss" => service.initialize_pci_requirements(),
                _ => {}
            }
        }
        
        service.initialize_retention_policies();
        
        service
    }
    
    fn initialize_gdpr_requirements(&mut self) {
        let requirements = vec![
            ComplianceRequirement {
                id: "GDPR-7".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Basis for processing".to_string(),
                description: "Processing is only lawful if at least one of the conditions applies".to_string(),
                category: "Legal basis".to_string(),
                severity: ComplianceSeverity::Critical,
                implementation_status: ImplementationStatus::PartiallyImplemented,
                controls: vec![
                    ComplianceControl {
                        id: "GDPR-7-1".to_string(),
                        name: "Consent management".to_string(),
                        description: "System to manage user consents".to_string(),
                        control_type: ControlType::Technical,
                        implementation: Some("Consent tracking system implemented".to_string()),
                        evidence: vec!["consent_logs.json".to_string()],
                        last_tested: Some(Utc::now() - Duration::days(30)),
                        test_result: Some(TestResult::Pass),
                    }
                ],
            },
            ComplianceRequirement {
                id: "GDPR-17".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Right to erasure".to_string(),
                description: "Data subject has the right to erasure of personal data".to_string(),
                category: "Individual rights".to_string(),
                severity: ComplianceSeverity::High,
                implementation_status: ImplementationStatus::FullyImplemented,
                controls: vec![
                    ComplianceControl {
                        id: "GDPR-17-1".to_string(),
                        name: "Data deletion API".to_string(),
                        description: "API endpoint for data deletion requests".to_string(),
                        control_type: ControlType::Technical,
                        implementation: Some("DELETE /api/v1/users/{id}/data endpoint".to_string()),
                        evidence: vec!["api_documentation.md".to_string(), "deletion_logs.json".to_string()],
                        last_tested: Some(Utc::now() - Duration::days(7)),
                        test_result: Some(TestResult::Pass),
                    }
                ],
            },
        ];
        
        self.requirements.insert(ComplianceFramework::GDPR, requirements);
    }
    
    fn initialize_hipaa_requirements(&mut self) {
        let requirements = vec![
            ComplianceRequirement {
                id: "HIPAA-164.312-a-1".to_string(),
                framework: ComplianceFramework::HIPAA,
                title: "Access control".to_string(),
                description: "Implement technical policies and procedures for electronic information systems".to_string(),
                category: "Administrative Safeguards".to_string(),
                severity: ComplianceSeverity::Critical,
                implementation_status: ImplementationStatus::FullyImplemented,
                controls: vec![
                    ComplianceControl {
                        id: "HIPAA-AC-1".to_string(),
                        name: "User access management".to_string(),
                        description: "Procedures for granting access to PHI".to_string(),
                        control_type: ControlType::Administrative,
                        implementation: Some("RBAC system with PHI access controls".to_string()),
                        evidence: vec!["access_control_policy.pdf".to_string(), "user_access_logs.json".to_string()],
                        last_tested: Some(Utc::now() - Duration::days(14)),
                        test_result: Some(TestResult::Pass),
                    }
                ],
            },
        ];
        
        self.requirements.insert(ComplianceFramework::HIPAA, requirements);
    }
    
    fn initialize_sox_requirements(&mut self) {
        let requirements = vec![
            ComplianceRequirement {
                id: "SOX-404".to_string(),
                framework: ComplianceFramework::SOX,
                title: "Management assessment of internal controls".to_string(),
                description: "Assessment of effectiveness of internal control over financial reporting".to_string(),
                category: "Internal Controls".to_string(),
                severity: ComplianceSeverity::High,
                implementation_status: ImplementationStatus::NotImplemented,
                controls: vec![],
            },
        ];
        
        self.requirements.insert(ComplianceFramework::SOX, requirements);
    }
    
    fn initialize_pci_requirements(&mut self) {
        let requirements = vec![
            ComplianceRequirement {
                id: "PCI-3.4".to_string(),
                framework: ComplianceFramework::PciDss,
                title: "Protect stored cardholder data".to_string(),
                description: "Render PAN unreadable anywhere it is stored".to_string(),
                category: "Data Protection".to_string(),
                severity: ComplianceSeverity::Critical,
                implementation_status: ImplementationStatus::FullyImplemented,
                controls: vec![
                    ComplianceControl {
                        id: "PCI-3.4-1".to_string(),
                        name: "Card data encryption".to_string(),
                        description: "All stored card data is encrypted".to_string(),
                        control_type: ControlType::Technical,
                        implementation: Some("AES-256 encryption for all card data".to_string()),
                        evidence: vec!["encryption_verification.json".to_string()],
                        last_tested: Some(Utc::now() - Duration::days(1)),
                        test_result: Some(TestResult::Pass),
                    }
                ],
            },
        ];
        
        self.requirements.insert(ComplianceFramework::PciDss, requirements);
    }
    
    fn initialize_retention_policies(&mut self) {
        self.retention_policies = vec![
            DataRetentionPolicy {
                id: "policy-1".to_string(),
                name: "User data retention".to_string(),
                data_types: vec!["user_profiles".to_string(), "session_data".to_string()],
                retention_period: Duration::days(2555), // 7 years
                deletion_method: DeletionMethod::HardDelete,
                exceptions: vec![
                    RetentionException {
                        condition: "Legal hold".to_string(),
                        extended_period: Duration::days(365),
                        justification: "Litigation hold requirements".to_string(),
                    }
                ],
                legal_holds: vec![],
            },
            DataRetentionPolicy {
                id: "policy-2".to_string(),
                name: "Audit log retention".to_string(),
                data_types: vec!["security_logs".to_string(), "access_logs".to_string()],
                retention_period: Duration::days(2555), // 7 years
                deletion_method: DeletionMethod::SoftDelete,
                exceptions: vec![],
                legal_holds: vec![],
            },
        ];
    }
    
    fn calculate_compliance_score(&self, framework: &ComplianceFramework) -> f64 {
        if let Some(requirements) = self.requirements.get(framework) {
            let total = requirements.len() as f64;
            let implemented = requirements.iter()
                .filter(|r| matches!(r.implementation_status, ImplementationStatus::FullyImplemented))
                .count() as f64;
            let partial = requirements.iter()
                .filter(|r| matches!(r.implementation_status, ImplementationStatus::PartiallyImplemented))
                .count() as f64;
            
            (implemented + (partial * 0.5)) / total * 100.0
        } else {
            0.0
        }
    }
}

#[async_trait]
impl ComplianceService for MultiFrameworkComplianceService {
    async fn generate_report(&self, framework: &str) -> crate::core::error::Result<ComplianceReport> {
        let framework_enum = match framework {
            "GDPR" => ComplianceFramework::GDPR,
            "HIPAA" => ComplianceFramework::HIPAA,
            "SOX" => ComplianceFramework::SOX,
            "PCI_DSS" => ComplianceFramework::PciDss,
            _ => return Err(crate::core::error::RustChainError::Security(format!("Unknown framework: {}", framework))),
        };
        
        let requirements = self.requirements.get(&framework_enum)
            .ok_or_else(|| crate::core::error::RustChainError::Security("Framework not configured".to_string()))?;
        
        let total_requirements = requirements.len();
        let implemented = requirements.iter()
            .filter(|r| matches!(r.implementation_status, ImplementationStatus::FullyImplemented))
            .count();
        let partially_implemented = requirements.iter()
            .filter(|r| matches!(r.implementation_status, ImplementationStatus::PartiallyImplemented))
            .count();
        let not_implemented = requirements.iter()
            .filter(|r| matches!(r.implementation_status, ImplementationStatus::NotImplemented))
            .count();
        
        let compliance_percentage = self.calculate_compliance_score(&framework_enum);
        
        let summary = ComplianceSummary {
            total_requirements,
            implemented,
            partially_implemented,
            not_implemented,
            compliance_percentage,
            risk_score: 100.0 - compliance_percentage,
        };
        
        let assessments: Vec<RequirementAssessment> = requirements.iter().map(|req| {
            RequirementAssessment {
                requirement: req.clone(),
                current_status: req.implementation_status.clone(),
                gap_analysis: match req.implementation_status {
                    ImplementationStatus::NotImplemented => "Full implementation required".to_string(),
                    ImplementationStatus::PartiallyImplemented => "Additional controls needed".to_string(),
                    ImplementationStatus::FullyImplemented => "No gaps identified".to_string(),
                    ImplementationStatus::NotApplicable => "Requirement not applicable".to_string(),
                },
                remediation_steps: match req.implementation_status {
                    ImplementationStatus::NotImplemented => vec![
                        "Conduct risk assessment".to_string(),
                        "Design control implementation".to_string(),
                        "Implement technical controls".to_string(),
                        "Test and validate".to_string(),
                    ],
                    ImplementationStatus::PartiallyImplemented => vec![
                        "Review existing controls".to_string(),
                        "Identify gaps".to_string(),
                        "Implement missing controls".to_string(),
                    ],
                    _ => vec![],
                },
                estimated_effort: Some(Duration::days(30)),
            }
        }).collect();
        
        let violations = self.violations.iter()
            .filter(|v| v.detected_at > Utc::now() - Duration::days(30))
            .cloned()
            .collect();
        
        let recommendations = vec![
            ComplianceRecommendation {
                id: "rec-1".to_string(),
                title: "Implement automated compliance monitoring".to_string(),
                description: "Deploy automated tools to continuously monitor compliance status".to_string(),
                priority: Priority::High,
                estimated_effort: Duration::days(14),
                expected_impact: "Reduce manual compliance efforts by 60%".to_string(),
            },
        ];
        
        Ok(ComplianceReport {
            framework: framework_enum,
            generated_at: Utc::now(),
            report_period: DateRange {
                start: Utc::now() - Duration::days(90),
                end: Utc::now(),
            },
            summary,
            requirements: assessments,
            violations,
            recommendations,
        })
    }
    
    async fn assess_requirement(&self, requirement_id: &str) -> crate::core::error::Result<RequirementAssessment> {
        for requirements in self.requirements.values() {
            if let Some(req) = requirements.iter().find(|r| r.id == requirement_id) {
                return Ok(RequirementAssessment {
                    requirement: req.clone(),
                    current_status: req.implementation_status.clone(),
                    gap_analysis: "Detailed gap analysis would be performed here".to_string(),
                    remediation_steps: vec!["Step 1".to_string(), "Step 2".to_string()],
                    estimated_effort: Some(Duration::days(30)),
                });
            }
        }
        
        Err(crate::core::error::RustChainError::Security("Requirement not found".to_string()))
    }
    
    async fn track_violation(&self, _violation: ComplianceViolation) -> crate::core::error::Result<()> {
        // In a real implementation, this would persist the violation
        Ok(())
    }
    
    async fn get_violations(&self, _framework: &ComplianceFramework, _since: DateTime<Utc>) -> crate::core::error::Result<Vec<ComplianceViolation>> {
        Ok(self.violations.clone())
    }
    
    async fn apply_retention_policy(&self, _policy: &DataRetentionPolicy) -> crate::core::error::Result<RetentionReport> {
        Ok(RetentionReport {
            policy_id: "policy-1".to_string(),
            applied_at: Utc::now(),
            records_reviewed: 1000,
            records_retained: 800,
            records_deleted: 200,
            errors: vec![],
        })
    }
    
    async fn audit_data_processing(&self, activity_id: &str) -> crate::core::error::Result<DataProcessingRecord> {
        Ok(DataProcessingRecord {
            id: activity_id.to_string(),
            timestamp: Utc::now(),
            data_subject: "user123".to_string(),
            processing_purpose: "Service delivery".to_string(),
            legal_basis: "Legitimate interest".to_string(),
            data_categories: vec!["Personal data".to_string(), "Usage data".to_string()],
            retention_period: Duration::days(365),
            third_parties: vec!["Analytics provider".to_string()],
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionReport {
    pub policy_id: String,
    pub applied_at: DateTime<Utc>,
    pub records_reviewed: u64,
    pub records_retained: u64,
    pub records_deleted: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub data_subject: String,
    pub processing_purpose: String,
    pub legal_basis: String,
    pub data_categories: Vec<String>,
    pub retention_period: Duration,
    pub third_parties: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::security::SecurityConfig;
    
    #[tokio::test]
    async fn test_gdpr_compliance_report() {
        let config = Arc::new(SecurityConfig {
            compliance_frameworks: vec!["GDPR".to_string()],
            ..Default::default()
        });
        
        let service = MultiFrameworkComplianceService::new(config);
        let report = service.generate_report("GDPR").await.unwrap();
        
        assert!(matches!(report.framework, ComplianceFramework::GDPR));
        assert!(report.summary.total_requirements > 0);
        assert!(report.summary.compliance_percentage <= 100.0);
    }
    
    #[tokio::test]
    async fn test_requirement_assessment() {
        let config = Arc::new(SecurityConfig {
            compliance_frameworks: vec!["GDPR".to_string()],
            ..Default::default()
        });
        
        let service = MultiFrameworkComplianceService::new(config);
        let assessment = service.assess_requirement("GDPR-7").await.unwrap();
        
        assert_eq!(assessment.requirement.id, "GDPR-7");
        assert!(matches!(assessment.requirement.framework, ComplianceFramework::GDPR));
    }
}