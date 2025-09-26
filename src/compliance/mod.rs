//! Compliance verification integration for RustChain missions
//! 
//! Self-contained mathematical compliance verification using SMT solving.
//! Supports NIST 800-53, GDPR, HIPAA, SOC2, ISO27001, PCI-DSS, FedRAMP, FISMA.

pub mod engine;
pub mod standards;

#[cfg(feature = "compliance")]
pub mod compliance_integration {
    use crate::core::Result;
    use crate::engine::Mission;
    use super::engine::{ComplianceSystem, ComplianceReport};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// RustChain compliance integration wrapper
    pub struct RustChainCompliance {
        system: Arc<RwLock<ComplianceSystem>>,
    }

    impl RustChainCompliance {
        /// Create new compliance integration
        pub async fn new() -> Result<Self> {
            let mut system = ComplianceSystem::new();
            system.initialize().await
                .map_err(|e| crate::core::RustChainError::Unknown { message: format!("Compliance init failed: {}", e) })?;
            
            Ok(Self {
                system: Arc::new(RwLock::new(system)),
            })
        }

        /// Verify RustChain mission against compliance standard
        pub async fn verify_mission(&self, mission: &Mission, standard: &str) -> Result<ComplianceReport> {
            let system = self.system.read().await;
            system.verify_compliance(standard, mission).await
                .map_err(|e| crate::core::RustChainError::Unknown { message: format!("Compliance verification failed: {}", e) })
        }

        /// Verify against all available standards
        pub async fn verify_all_standards(&self, mission: &Mission) -> Result<Vec<ComplianceReport>> {
            let standards = ["NIST_800_53", "GDPR", "HIPAA", "SOC2", "ISO27001", "PCI_DSS", "FedRAMP", "FISMA"];
            let mut reports = Vec::new();

            for standard in standards {
                let report = self.verify_mission(mission, standard).await?;
                reports.push(report);
            }

            Ok(reports)
        }

    }

    /// CLI command integration for compliance verification
    pub async fn verify_mission_compliance(mission_path: &str, standard: Option<String>) -> Result<()> {
        use crate::engine::MissionLoader;

        // Load RustChain mission
        let mission = MissionLoader::load_from_file(mission_path)?;
        
        // Initialize compliance system
        let compliance = RustChainCompliance::new().await?;

        // Verify compliance
        let reports = if let Some(std) = standard {
            vec![compliance.verify_mission(&mission, &std).await?]
        } else {
            compliance.verify_all_standards(&mission).await?
        };

        // Display results
        for report in reports {
            println!("📋 Mission: {}", mission.name);
            println!("📊 Standard: {}", report.standard);
            println!("🎯 Risk Score: {}/100", report.risk_score);
            
            if report.compliant {
                println!("✅ COMPLIANT: No violations detected");
            } else {
                println!("❌ VIOLATIONS FOUND:");
                for violation in &report.violations {
                    println!("  ⚠️  {}", violation);
                }
            }
            
            println!("🏆 Mathematical proof: {}", report.mathematical_proof.as_deref().unwrap_or("None"));
            println!("📈 Constraints: {}/{} passed", 
                    report.passed_constraints,
                    report.total_constraints);
            println!();
        }

        Ok(())
    }
}

#[cfg(not(feature = "compliance"))]
pub mod compliance_integration {
    use crate::core::Result;

    /// Placeholder when compliance feature is disabled
    pub async fn verify_mission_compliance(_mission_path: &str, _standard: Option<String>) -> Result<()> {
        println!("⚠️ Compliance verification requires --features compliance");
        println!("💡 Run: cargo run --features compliance -- compliance verify mission.yaml");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "compliance")]
    use super::compliance_integration::*;
    use crate::engine::{Mission, MissionStep, StepType};

    #[tokio::test]
    #[cfg(feature = "compliance")]
    async fn test_compliance_system_initialization() {
        let compliance = RustChainCompliance::new().await;
        assert!(compliance.is_ok(), "Compliance system should initialize successfully");
    }

    #[tokio::test]
    #[cfg(feature = "compliance")]
    async fn test_gdpr_compliance_verification() {
        let compliance = RustChainCompliance::new().await.expect("Failed to initialize compliance");
        
        // Create a GDPR-safe test mission
        let mission = create_test_mission();
        
        let result = compliance.verify_mission(&mission, "GDPR").await;
        assert!(result.is_ok(), "GDPR verification should succeed: {:?}", result.err());
        
        let report = result.unwrap();
        assert_eq!(report.standard, "GDPR");
        assert!(report.risk_score <= 100.0);
    }

    #[tokio::test]
    #[cfg(feature = "compliance")]
    async fn test_nist_compliance_verification() {
        let compliance = RustChainCompliance::new().await.expect("Failed to initialize compliance");
        
        let mission = create_test_mission();
        
        let result = compliance.verify_mission(&mission, "NIST_800_53").await;
        assert!(result.is_ok(), "NIST verification should succeed: {:?}", result.err());
        
        let report = result.unwrap();
        assert_eq!(report.standard, "NIST_800_53");
    }

    #[tokio::test]
    #[cfg(feature = "compliance")]
    async fn test_all_standards_verification() {
        let compliance = RustChainCompliance::new().await.expect("Failed to initialize compliance");
        
        let mission = create_test_mission();
        
        let result = compliance.verify_all_standards(&mission).await;
        assert!(result.is_ok(), "All standards verification should succeed: {:?}", result.err());
        
        let reports = result.unwrap();
        assert_eq!(reports.len(), 8, "Should generate reports for all 8 standards");
        
        // Verify all major standards are covered
        let standards: Vec<String> = reports.iter().map(|r| r.standard.clone()).collect();
        assert!(standards.contains(&"GDPR".to_string()));
        assert!(standards.contains(&"NIST_800_53".to_string()));
        assert!(standards.contains(&"HIPAA".to_string()));
        assert!(standards.contains(&"SOC2".to_string()));
        assert!(standards.contains(&"ISO27001".to_string()));
        assert!(standards.contains(&"PCI_DSS".to_string()));
        assert!(standards.contains(&"FedRAMP".to_string()));
        assert!(standards.contains(&"FISMA".to_string()));
    }

    #[tokio::test]
    #[cfg(feature = "compliance")]
    async fn test_hipaa_compliance_verification() {
        let compliance = RustChainCompliance::new().await.expect("Failed to initialize compliance");
        
        let mission = create_test_mission();
        
        let result = compliance.verify_mission(&mission, "HIPAA").await;
        assert!(result.is_ok(), "HIPAA verification should succeed");
        
        let report = result.unwrap();
        assert_eq!(report.standard, "HIPAA");
        assert!(report.mathematical_proof.is_some() || report.mathematical_proof.as_deref() == Some(""));
    }

    #[cfg(feature = "compliance")]
    fn create_test_mission() -> Mission {
        Mission {
            version: "1.0".to_string(),
            name: "Compliance Test Mission".to_string(),
            description: Some("Testing RustChain compliance integration".to_string()),
            steps: vec![
                MissionStep {
                    id: "safe_operation".to_string(),
                    name: "Safe File Operation".to_string(),
                    step_type: StepType::CreateFile,
                    continue_on_error: None,
                    parameters: serde_json::json!({
                        "path": "/tmp/compliance_test.txt",
                        "content": "Safe compliance testing"
                    }),
                    depends_on: Some(vec![]),
                    timeout_seconds: None,
                }
            ],
            config: None,
        }
    }

    #[test]
    #[cfg(not(feature = "compliance"))]
    fn test_compliance_disabled_message() {
        // This test runs when compliance feature is disabled
        // Just ensure the module compiles correctly
        assert!(true, "Compliance module compiles when feature is disabled");
    }
}
