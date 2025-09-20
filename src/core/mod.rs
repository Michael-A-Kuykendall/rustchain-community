use crate::assert_invariant;
#[cfg(feature = "rag")]
use crate::rag::RagSystem;
#[cfg(feature = "sandbox")]
use crate::sandbox::EnhancedSandbox;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use uuid::Uuid;

// Error handling
pub mod error;
pub mod error_formatting;
pub use error::*;
pub use error_formatting::*;

// Mission system
pub mod mission;
pub use mission::*;

// Executor system
pub mod executor;
pub use executor::*;

// Enhanced audit system
pub mod audit;
pub use audit::*;

// Memory system
pub mod memory;
pub use memory::*;

// Agent system
pub mod agent;
pub use agent::*;

// Chain system
pub mod chain;
pub use chain::*;

// LLM system
pub mod llm;
pub use llm::*;

// Tools system
pub mod tools;
pub use tools::*;

// Web search tools
#[cfg(feature = "tools")]
pub mod web_search_tools;

// Document loaders
#[cfg(feature = "tools")]
pub mod document_loaders;
#[cfg(feature = "tools")]
pub use document_loaders::*;

// Vector stores
#[cfg(feature = "rag")]
pub mod pinecone_vector_store;

#[cfg(feature = "rag")]
pub mod chroma_vector_store;

// Code interpreters
#[cfg(feature = "tools")]
pub mod python_interpreter;
#[cfg(feature = "tools")]
pub use python_interpreter::*;

// Developer toolkits
#[cfg(feature = "tools")]
pub mod github_toolkit;

// Plugin system for enterprise features
pub mod plugin;
pub use plugin::*;

// Feature detection and boundary enforcement
pub mod features;
pub use features::*;

/// Central runtime context that holds all system state
#[derive(Clone)]
pub struct RuntimeContext {
    pub config: Arc<RwLock<Config>>,
    pub audit: Arc<AuditSink>,
    pub tool_registry: Arc<RwLock<ToolRegistry>>,
    pub model_manager: Option<Arc<ModelManager>>,
    pub sandbox: Arc<AgentSandbox>,
    pub policy_engine: Arc<PolicyEngine>,
    pub perf_collector: Arc<RwLock<PerfCollector>>,
    pub plugin_manager: Arc<RwLock<PluginManager>>,
    pub feature_detector: Arc<FeatureDetector>,
    #[cfg(feature = "rag")]
    pub rag_system: Option<Arc<RwLock<RagSystem>>>,
    #[cfg(feature = "sandbox")]
    pub enhanced_sandbox: Option<Arc<EnhancedSandbox>>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        assert_invariant!(true, "RuntimeContext created", Some("core"));

        Self {
            config: Arc::new(RwLock::new(Config::default())),
            audit: Arc::new(AuditSink::new()),
            tool_registry: Arc::new(RwLock::new(ToolRegistry::new())),
            model_manager: None,
            sandbox: Arc::new(AgentSandbox::new()),
            policy_engine: Arc::new(PolicyEngine::new()),
            perf_collector: Arc::new(RwLock::new(PerfCollector::new())),
            plugin_manager: Arc::new(RwLock::new(PluginManager::new())),
            feature_detector: Arc::new(FeatureDetector::new()),
            #[cfg(feature = "rag")]
            rag_system: None,
            #[cfg(feature = "sandbox")]
            enhanced_sandbox: None,
        }
    }

    pub async fn audit_action(&self, agent_id: &str, action: &str, outcome: &str) {
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            actor: agent_id.to_string(),
            action: action.to_string(),
            outcome: outcome.to_string(),
            reason: None,
        };
        self.audit.log(entry).await;
    }

    /// Check if an enterprise feature is available through plugins
    pub async fn has_enterprise_feature(&self, feature: &str) -> bool {
        if cfg!(feature = "enterprise") {
            self.plugin_manager.read().await.has_feature(feature)
        } else {
            false
        }
    }

    /// Get list of all available enterprise features
    pub async fn get_enterprise_features(&self) -> Vec<String> {
        if cfg!(feature = "enterprise") {
            self.plugin_manager.read().await.enabled_features()
        } else {
            vec![]
        }
    }

    /// Get list of all available core features
    pub async fn get_available_features(&self) -> Vec<String> {
        let mut features = Vec::new();
        
        // Core features that are always available
        features.push("mission_execution".to_string());
        features.push("safety_validation".to_string());
        features.push("audit_logging".to_string());
        features.push("policy_engine".to_string());
        
        // Feature-gated components
        #[cfg(feature = "llm")]
        features.push("llm_integration".to_string());
        
        #[cfg(feature = "tools")]
        features.push("tool_system".to_string());
        
        #[cfg(feature = "rag")]
        features.push("rag_system".to_string());
        
        #[cfg(feature = "sandbox")]
        features.push("sandbox".to_string());
        
        #[cfg(feature = "server")]
        features.push("api_server".to_string());
        
        #[cfg(feature = "compliance")]
        features.push("compliance_checking".to_string());
        
        features
    }

    /// Load enterprise plugins (not available in community edition)
    pub async fn load_enterprise_plugins(&self) -> crate::core::error::Result<()> {
        // Community edition: No enterprise plugins available
        Ok(())
    }

    /// Enhanced feature detection with detailed status
    pub async fn check_feature_status(&self, feature: &str) -> FeatureStatus {
        self.feature_detector.is_feature_available(self, feature).await
    }

    /// Require a feature or return detailed error
    pub async fn require_feature(&self, feature: &str) -> crate::core::error::Result<()> {
        self.feature_detector.require_feature(self, feature).await
    }

    /// Get comprehensive feature summary for this installation
    pub async fn get_feature_summary(&self) -> FeatureSummary {
        self.feature_detector.get_feature_summary(self).await
    }

    /// Get status for all features in a category
    pub async fn get_category_status(&self, category: &str) -> Vec<FeatureStatus> {
        self.feature_detector.get_category_status(self, category).await
    }

    /// Check if running enterprise edition with full features
    pub async fn is_enterprise_complete(&self) -> bool {
        self.get_feature_summary().await.is_enterprise_complete()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub mission_timeout_seconds: u64,
    pub max_parallel_steps: usize,
    pub audit_enabled: bool,
    pub network_policy: NetworkPolicy,
    pub agent_id: String,
    pub max_tool_calls: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mission_timeout_seconds: 300,
            max_parallel_steps: 4,
            audit_enabled: true,
            network_policy: NetworkPolicy::Offline,
            agent_id: "rustchain-agent".to_string(),
            max_tool_calls: 100,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkPolicy {
    Offline,
    AllowList(Vec<String>),
}

/// Enhanced audit sink with cryptographic chain integrity
pub struct AuditSink {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
}

impl AuditSink {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn log(&self, entry: AuditEntry) {
        self.entries.write().await.push(entry);
    }

    pub async fn get_chain_hash(&self) -> String {
        let entries = self.entries.read().await;
        if entries.is_empty() {
            return "genesis".to_string();
        }

        let mut hasher = Sha256::new();
        for entry in entries.iter() {
            hasher.update(
                format!(
                    "{}{}{}{}",
                    entry.timestamp.to_rfc3339(),
                    entry.actor,
                    entry.action,
                    entry.outcome
                )
                .as_bytes(),
            );
        }
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub outcome: String,
    pub reason: Option<String>,
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, tool: Box<dyn Tool + Send + Sync>) {
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool + Send + Sync>> {
        self.tools.get(name)
    }
}

pub trait Tool {
    fn name(&self) -> &str;
    fn invoke(&self, args: serde_json::Value) -> anyhow::Result<serde_json::Value>;
}

/// Performance metrics collection
#[derive(Debug, Clone)]
pub struct PerfMetric {
    pub name: String,
    pub duration_ms: u128,
}

pub struct PerfCollector {
    active: HashMap<String, Instant>,
    pub completed: Vec<PerfMetric>,
}

impl PerfCollector {
    pub fn new() -> Self {
        Self {
            active: HashMap::new(),
            completed: vec![],
        }
    }

    pub fn start(&mut self, name: &str) {
        self.active.insert(name.to_string(), Instant::now());
    }

    pub fn end(&mut self, name: &str) {
        if let Some(start) = self.active.remove(name) {
            let duration = start.elapsed().as_millis();
            self.completed.push(PerfMetric {
                name: name.to_string(),
                duration_ms: duration,
            });
        }
    }

    pub fn summary(&self) -> String {
        self.completed
            .iter()
            .map(|m| format!("{}: {}ms", m.name, m.duration_ms))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct ModelManager {
    // Will be implemented in Gate 6
    #[cfg(feature = "llm")]
    llm_manager: Option<crate::llm::LLMManager>,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "llm")]
            llm_manager: None,
        }
    }

    #[cfg(feature = "llm")]
    pub fn with_llm_manager(mut self, manager: crate::llm::LLMManager) -> Self {
        self.llm_manager = Some(manager);
        self
    }

    #[cfg(feature = "llm")]
    pub async fn complete(
        &self,
        request: crate::llm::LLMRequest,
        provider: Option<&str>,
    ) -> anyhow::Result<crate::llm::LLMResponse> {
        if let Some(ref manager) = self.llm_manager {
            manager.complete(request, provider).await
        } else {
            Err(anyhow::anyhow!("LLM manager not initialized"))
        }
    }
}

pub struct AgentSandbox {
    #[allow(dead_code)]
    allowed_paths: Vec<std::path::PathBuf>,
    #[allow(dead_code)]
    timeout_seconds: u64,
}

impl AgentSandbox {
    pub fn new() -> Self {
        // Safe default path handling - fallback to current directory or root
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
            
        Self {
            allowed_paths: vec![current_dir],
            timeout_seconds: 30,
        }
    }

    pub fn execute(&self, code: &str) -> std::result::Result<String, String> {
        // Placeholder sandbox execution
        Ok(format!("Executed in sandbox: {}", code))
    }
}

pub struct PolicyEngine {
    policies: Vec<String>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    pub fn validate(&self, action: &str) -> bool {
        // Basic policy validation - will be replaced by enhanced engine
        !self.policies.iter().any(|p| action.contains(p))
    }

    pub fn add_policy(&mut self, policy: String) {
        self.policies.push(policy);
    }
}
