//! REST API for Compliance SDK

use crate::compliance::{ComplianceSystem, ComplianceReport};
use crate::core::Result;
use crate::engine::Mission;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// REST API endpoints for compliance verification
#[derive(Clone)]
pub struct ComplianceAPI {
    system: Arc<Mutex<ComplianceSystem>>,
}

/// API request for mission verification
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub mission: Mission,
    pub standard: String,
}

/// API response for verification
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub success: bool,
    pub report: Option<ComplianceReport>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// API request for batch verification
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchVerifyRequest {
    pub missions: Vec<Mission>,
    pub standards: Vec<String>,
}

/// API response for batch verification
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchVerifyResponse {
    pub success: bool,
    pub reports: Vec<ComplianceReport>,
    pub errors: Vec<String>,
    pub total_missions: usize,
    pub compliant_missions: usize,
    pub execution_time_ms: u64,
}

/// Standards listing response
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardsResponse {
    pub standards: Vec<StandardInfo>,
    pub total_constraints: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardInfo {
    pub name: String,
    pub constraint_count: usize,
    pub description: String,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub standards_loaded: usize,
    pub total_constraints: usize,
    pub uptime_seconds: u64,
}

impl ComplianceAPI {
    pub fn new() -> Self {
        Self {
            system: Arc::new(Mutex::new(ComplianceSystem::new())),
        }
    }
    
    /// Initialize API with compliance system
    pub async fn initialize(&self) -> Result<()> {
        let mut system = self.system.lock().await;
        system.initialize().await
    }
    
    /// GET /api/v1/standards - List available compliance standards
    pub async fn list_standards(&self) -> Result<StandardsResponse> {
        let system = self.system.lock().await;
        let standards = system.list_standards();
        
        let standard_infos: Vec<StandardInfo> = standards.iter().map(|name| {
            let count = system.get_constraint_count(name);
            let description = match name.as_str() {
                "GDPR" => "EU General Data Protection Regulation - Privacy and data protection",
                "NIST_800_53" => "NIST SP 800-53 - Security Controls for Federal Information Systems",
                "HIPAA" => "Health Insurance Portability and Accountability Act - Healthcare privacy",
                "SOC2" => "SOC 2 Type II - Service Organization Control for trust services",
                "ISO27001" => "ISO/IEC 27001 - Information Security Management System",
                "PCI_DSS" => "Payment Card Industry Data Security Standard - Payment processing security",
                _ => "Custom compliance standard",
            };
            
            StandardInfo {
                name: name.clone(),
                constraint_count: count,
                description: description.to_string(),
            }
        }).collect();
        
        let total_constraints = standard_infos.iter().map(|s| s.constraint_count).sum();
        
        Ok(StandardsResponse {
            standards: standard_infos,
            total_constraints,
        })
    }
    
    /// POST /api/v1/verify - Verify single mission compliance
    pub async fn verify_mission(&self, request: VerifyRequest) -> Result<VerifyResponse> {
        let start_time = std::time::Instant::now();
        let system = self.system.lock().await;
        
        match system.verify_compliance(&request.standard, &request.mission).await {
            Ok(report) => {
                Ok(VerifyResponse {
                    success: true,
                    report: Some(report),
                    error: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                })
            },
            Err(e) => {
                Ok(VerifyResponse {
                    success: false,
                    report: None,
                    error: Some(format!("{}", e)),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                })
            }
        }
    }
    
    /// POST /api/v1/verify/batch - Verify multiple missions
    pub async fn verify_batch(&self, request: BatchVerifyRequest) -> Result<BatchVerifyResponse> {
        let start_time = std::time::Instant::now();
        let system = self.system.lock().await;
        
        let mut reports = Vec::new();
        let mut errors = Vec::new();
        let mut compliant_count = 0;
        
        for mission in &request.missions {
            for standard in &request.standards {
                match system.verify_compliance(standard, mission).await {
                    Ok(report) => {
                        if report.compliant {
                            compliant_count += 1;
                        }
                        reports.push(report);
                    },
                    Err(e) => {
                        errors.push(format!("Mission '{}' / Standard '{}': {}", mission.name, standard, e));
                    }
                }
            }
        }
        
        Ok(BatchVerifyResponse {
            success: errors.is_empty(),
            reports,
            errors,
            total_missions: request.missions.len(),
            compliant_missions: compliant_count,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }
    
    /// GET /api/v1/health - Health check endpoint
    pub async fn health_check(&self, uptime_seconds: u64) -> Result<HealthResponse> {
        let system = self.system.lock().await;
        let standards = system.list_standards();
        let total_constraints = standards.iter()
            .map(|s| system.get_constraint_count(s))
            .sum();
        
        Ok(HealthResponse {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            standards_loaded: standards.len(),
            total_constraints,
            uptime_seconds,
        })
    }
    
    /// POST /api/v1/download/nist - Download NIST catalog
    pub async fn download_nist_catalog(&self) -> Result<String> {
        use reqwest;
        
        let url = "https://raw.githubusercontent.com/usnistgov/OSCAL/main/src/content/nist.gov/SP800-53/rev5/json/NIST_SP-800-53_rev5_catalog.json";
        
        let response = reqwest::get(url).await
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to download NIST catalog: {}", e)
                }
            ))?;
        
        let content = response.text().await
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::ParseError {
                    reason: format!("Failed to read response: {}", e)
                }
            ))?;
        
        // Save to file
        std::fs::write("nist_800_53_catalog.json", &content)?;
        
        // Reinitialize system to load new catalog
        let mut system = self.system.lock().await;
        system.initialize().await?;
        
        Ok(content)
    }
}

impl Default for ComplianceAPI {
    fn default() -> Self {
        Self::new()
    }
}

/// Enterprise pricing and licensing information
#[derive(Debug, Serialize, Deserialize)]
pub struct PricingInfo {
    pub plans: Vec<PricingPlan>,
    pub enterprise_contact: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingPlan {
    pub name: String,
    pub price_annual: u32,
    pub features: Vec<String>,
    pub constraint_limit: Option<usize>,
    pub api_calls_per_month: Option<u32>,
}

impl ComplianceAPI {
    /// GET /api/v1/pricing - Enterprise pricing information
    pub fn get_pricing_info(&self) -> PricingInfo {
        PricingInfo {
            plans: vec![
                PricingPlan {
                    name: "Starter".to_string(),
                    price_annual: 25000,
                    features: vec![
                        "GDPR compliance verification".to_string(),
                        "Basic NIST 800-53 controls".to_string(),
                        "CLI interface".to_string(),
                        "JSON/YAML reports".to_string(),
                    ],
                    constraint_limit: Some(500),
                    api_calls_per_month: Some(10000),
                },
                PricingPlan {
                    name: "Professional".to_string(),
                    price_annual: 50000,
                    features: vec![
                        "All Starter features".to_string(),
                        "Complete NIST 800-53 (1,196 controls)".to_string(),
                        "HIPAA compliance".to_string(),
                        "SOC 2 compliance".to_string(),
                        "REST API access".to_string(),
                        "PDF reports".to_string(),
                        "Batch verification".to_string(),
                    ],
                    constraint_limit: Some(2000),
                    api_calls_per_month: Some(50000),
                },
                PricingPlan {
                    name: "Enterprise".to_string(),
                    price_annual: 75000,
                    features: vec![
                        "All Professional features".to_string(),
                        "ISO 27001 compliance".to_string(),
                        "PCI-DSS compliance".to_string(),
                        "Custom compliance standards".to_string(),
                        "Dedicated support".to_string(),
                        "SLA guarantees".to_string(),
                        "On-premise deployment".to_string(),
                        "Custom integrations".to_string(),
                    ],
                    constraint_limit: None,
                    api_calls_per_month: None,
                },
            ],
            enterprise_contact: "enterprise@rustchain.dev".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{MissionStep, StepType};
    
    #[tokio::test]
    async fn test_api_initialization() {
        let api = ComplianceAPI::new();
        let result = api.initialize().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_list_standards_api() {
        let api = ComplianceAPI::new();
        let _ = api.initialize().await;
        
        let response = api.list_standards().await;
        assert!(response.is_ok());
        
        let standards_response = response.unwrap();
        assert!(!standards_response.standards.is_empty());
        assert!(standards_response.total_constraints > 0);
    }
    
    #[tokio::test]
    async fn test_verify_mission_api() {
        let api = ComplianceAPI::new();
        let _ = api.initialize().await;
        
        let mission = Mission {
            version: "1.0".to_string(),
            name: "API Test Mission".to_string(),
            description: None,
            steps: vec![
                MissionStep {
                    id: "safe_step".to_string(),
                    name: "Safe Step".to_string(),
                    step_type: StepType::Noop,
                    depends_on: vec![],
                    continue_on_error: None,
                parameters: serde_json::json!({}),
                }
            ],
            config: None,
        };
        
        let request = VerifyRequest {
            mission,
            standard: "GDPR".to_string(),
        };
        
        let response = api.verify_mission(request).await;
        assert!(response.is_ok());
        
        let verify_response = response.unwrap();
        assert!(verify_response.success);
        assert!(verify_response.report.is_some());
        assert!(verify_response.execution_time_ms < 5000);
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let api = ComplianceAPI::new();
        let _ = api.initialize().await;
        
        let response = api.health_check(3600).await;
        assert!(response.is_ok());
        
        let health = response.unwrap();
        assert_eq!(health.status, "healthy");
        assert_eq!(health.uptime_seconds, 3600);
        assert!(health.standards_loaded > 0);
    }
}