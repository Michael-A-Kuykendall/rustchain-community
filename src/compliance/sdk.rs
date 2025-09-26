//! Compliance SDK for programmatic integration

use crate::compliance::ComplianceSystem;
use crate::core::Result;
use crate::engine::Mission;
use crate::smt::constraints::SMTConstraint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enterprise Compliance SDK for AI Systems
/// 
/// This SDK provides programmatic access to compliance verification
/// for AI systems against major standards (GDPR, NIST 800-53, etc.)
#[derive(Debug)]
pub struct ComplianceSDK {
    system: ComplianceSystem,
    initialized: bool,
}

/// SDK Configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDKConfig {
    /// Path to NIST 800-53 catalog
    pub nist_catalog_path: Option<String>,
    
    /// Enable automatic catalog download
    pub auto_download: bool,
    
    /// Cache constraints in memory
    pub enable_caching: bool,
    
    /// Maximum cache size (number of constraint sets)
    pub max_cache_size: usize,
    
    /// API endpoint for remote compliance service
    pub remote_endpoint: Option<String>,
    
    /// API key for remote service
    pub api_key: Option<String>,
}

impl Default for SDKConfig {
    fn default() -> Self {
        Self {
            nist_catalog_path: Some("nist_800_53_catalog.json".to_string()),
            auto_download: false,
            enable_caching: true,
            max_cache_size: 10,
            remote_endpoint: None,
            api_key: None,
        }
    }
}

/// Compliance verification result with detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub standard: String,
    pub compliant: bool,
    pub risk_score: f64,
    pub constraint_count: usize,
    pub violations_count: usize,
    pub verification_time_ms: u64,
    pub details: ComplianceDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDetails {
    pub constraints_checked: Vec<String>,
    pub violations: Vec<ComplianceViolation>,
    pub recommendations: Vec<String>,
    pub severity_breakdown: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: String,
    pub severity: String,
    pub description: String,
    pub constraint: String,
    pub recommendation: String,
}

impl ComplianceSDK {
    /// Create new SDK instance with default configuration
    pub fn new() -> Self {
        Self {
            system: ComplianceSystem::new(),
            initialized: false,
        }
    }
    
    /// Create SDK with custom configuration
    pub fn with_config(config: SDKConfig) -> Self {
        let mut sdk = Self::new();
        // Apply configuration settings
        sdk
    }
    
    /// Initialize SDK with compliance standards
    pub async fn initialize(&mut self) -> Result<()> {
        if !self.initialized {
            self.system.initialize().await?;
            self.initialized = true;
        }
        Ok(())
    }
    
    /// Verify single mission against specific standard
    pub async fn verify_mission(&self, mission: &Mission, standard: &str) -> Result<ComplianceResult> {
        if !self.initialized {
            return Err(crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: "SDK not initialized - call initialize() first".to_string()
                }
            ));
        }
        
        let start_time = std::time::Instant::now();
        
        let report = self.system.verify_compliance(standard, mission).await?;
        
        let verification_time = start_time.elapsed().as_millis() as u64;
        
        Ok(ComplianceResult {
            standard: standard.to_string(),
            compliant: report.compliant,
            risk_score: report.risk_score,
            constraint_count: self.system.get_constraint_count(standard),
            violations_count: report.violations.len(),
            verification_time_ms: verification_time,
            details: ComplianceDetails {
                constraints_checked: report.constraints_checked,
                violations: report.violations.into_iter().map(|v| ComplianceViolation {
                    id: v.constraint_id,
                    severity: format!("{:?}", v.severity),
                    description: v.description,
                    constraint: v.constraint,
                    recommendation: v.recommendation,
                }).collect(),
                recommendations: report.recommendations,
                severity_breakdown: report.severity_breakdown,
            },
        })
    }
    
    /// Verify mission against all available standards
    pub async fn verify_comprehensive(&self, mission: &Mission) -> Result<Vec<ComplianceResult>> {
        let mut results = Vec::new();
        
        for standard in self.system.list_standards() {
            if let Ok(result) = self.verify_mission(mission, &standard).await {
                results.push(result);
            }
        }
        
        Ok(results)
    }
    
    /// Get available compliance standards
    pub fn list_standards(&self) -> Vec<String> {
        self.system.list_standards()
    }
    
    /// Get constraint count for standard
    pub fn get_constraint_count(&self, standard: &str) -> usize {
        self.system.get_constraint_count(standard)
    }
    
    /// Load mission from file path
    pub async fn load_mission(&self, path: &str) -> Result<Mission> {
        let content = std::fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to parse mission: {}", e)
                }
            ))
    }
    
    /// Batch verify multiple missions
    pub async fn verify_batch(&self, missions: &[Mission], standard: &str) -> Result<Vec<ComplianceResult>> {
        let mut results = Vec::new();
        
        for mission in missions {
            match self.verify_mission(mission, standard).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    // Continue with other missions, log error
                    eprintln!("Warning: Failed to verify mission '{}': {}", mission.name, e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Generate aggregate compliance metrics
    pub fn generate_metrics(&self, results: &[ComplianceResult]) -> ComplianceMetrics {
        let total_missions = results.len();
        let compliant_missions = results.iter().filter(|r| r.compliant).count();
        let compliance_rate = if total_missions > 0 {
            (compliant_missions as f64 / total_missions as f64) * 100.0
        } else {
            0.0
        };
        
        let avg_risk_score = if total_missions > 0 {
            results.iter().map(|r| r.risk_score).sum::<f64>() / total_missions as f64
        } else {
            0.0
        };
        
        let total_violations: usize = results.iter().map(|r| r.violations_count).sum();
        
        ComplianceMetrics {
            total_missions,
            compliant_missions,
            compliance_rate,
            avg_risk_score,
            total_violations,
            standards_covered: results.iter().map(|r| r.standard.clone()).collect::<std::collections::HashSet<_>>().len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub total_missions: usize,
    pub compliant_missions: usize,
    pub compliance_rate: f64,
    pub avg_risk_score: f64,
    pub total_violations: usize,
    pub standards_covered: usize,
}

impl Default for ComplianceSDK {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{MissionStep, StepType};
    
    #[tokio::test]
    async fn test_sdk_initialization() {
        let mut sdk = ComplianceSDK::new();
        let result = sdk.initialize().await;
        assert!(result.is_ok());
        assert!(sdk.initialized);
    }
    
    #[tokio::test]
    async fn test_sdk_verification() {
        let mut sdk = ComplianceSDK::new();
        let _ = sdk.initialize().await;
        
        // Create test mission
        let mission = Mission {
            version: "1.0".to_string(),
            name: "SDK Test Mission".to_string(),
            description: Some("Test mission for SDK".to_string()),
            steps: vec![
                MissionStep {
                    id: "test_step".to_string(),
                    name: "Test Step".to_string(),
                    step_type: StepType::Noop,
                    depends_on: vec![],
                    continue_on_error: None,
                parameters: serde_json::json!({}),
                }
            ],
            config: None,
        };
        
        // Test GDPR verification
        let result = sdk.verify_mission(&mission, "GDPR").await;
        assert!(result.is_ok());
        
        let compliance_result = result.unwrap();
        assert_eq!(compliance_result.standard, "GDPR");
        assert!(compliance_result.verification_time_ms < 5000); // Should be fast
    }
    
    #[tokio::test]
    async fn test_comprehensive_verification() {
        let mut sdk = ComplianceSDK::new();
        let _ = sdk.initialize().await;
        
        // Create test mission
        let mission = Mission {
            version: "1.0".to_string(),
            name: "Comprehensive Test".to_string(),
            description: None,
            steps: vec![],
            config: None,
        };
        
        // Test comprehensive verification
        let results = sdk.verify_comprehensive(&mission).await;
        assert!(results.is_ok());
        
        let compliance_results = results.unwrap();
        assert!(!compliance_results.is_empty());
        
        // Test metrics generation
        let metrics = sdk.generate_metrics(&compliance_results);
        assert_eq!(metrics.total_missions, 1);
        assert!(metrics.standards_covered > 0);
    }
}