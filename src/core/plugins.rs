use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;
use serde::{Serialize, Deserialize};

use super::RuntimeContext;

/// Trait for enterprise plugins that extend RustChain functionality
#[async_trait]
pub trait EnterprisePlugin: Send + Sync {
    /// Plugin identifier
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Features provided by this plugin
    fn features(&self) -> Vec<String>;
    
    /// Initialize the plugin with runtime context
    async fn initialize(&mut self, context: &mut RuntimeContext) -> Result<()>;
    
    /// Shutdown the plugin gracefully
    async fn shutdown(&mut self) -> Result<()>;
    
    /// Check if plugin is healthy/operational
    async fn health_check(&self) -> Result<PluginHealth>;
}

/// Plugin health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHealth {
    pub status: PluginStatus,
    pub message: String,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Plugin metadata for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
}

/// Manages enterprise plugins at runtime
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn EnterprisePlugin>>,
    enabled_features: HashSet<String>,
    metadata: HashMap<String, PluginMetadata>,
    initialized: bool,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            enabled_features: HashSet::new(),
            metadata: HashMap::new(),
            initialized: false,
        }
    }

    /// Register a new enterprise plugin
    pub fn register_plugin(&mut self, plugin: Box<dyn EnterprisePlugin>, metadata: PluginMetadata) -> Result<()> {
        let name = plugin.name().to_string();
        let features = plugin.features();
        
        // Validate plugin doesn't already exist
        if self.plugins.contains_key(&name) {
            return Err(anyhow::anyhow!("Plugin '{}' is already registered", name));
        }
        
        // Add features to enabled set
        for feature in features {
            self.enabled_features.insert(feature);
        }
        
        // Store plugin and metadata
        self.metadata.insert(name.clone(), metadata);
        self.plugins.insert(name, plugin);
        
        Ok(())
    }

    /// Initialize all registered plugins
    pub async fn initialize_all(&mut self, context: &mut RuntimeContext) -> Result<()> {
        for (name, plugin) in &mut self.plugins {
            match plugin.initialize(context).await {
                Ok(()) => {
                    tracing::info!("Enterprise plugin '{}' initialized successfully", name);
                }
                Err(e) => {
                    tracing::error!("Failed to initialize enterprise plugin '{}': {}", name, e);
                    return Err(anyhow::anyhow!("Plugin initialization failed: {}", e));
                }
            }
        }
        
        self.initialized = true;
        tracing::info!("All enterprise plugins initialized successfully");
        Ok(())
    }

    /// Shutdown all plugins gracefully
    pub async fn shutdown_all(&mut self) -> Result<()> {
        for (name, plugin) in &mut self.plugins {
            if let Err(e) = plugin.shutdown().await {
                tracing::warn!("Failed to shutdown plugin '{}': {}", name, e);
            }
        }
        
        self.initialized = false;
        Ok(())
    }

    /// Check if a specific feature is available
    pub fn has_feature(&self, feature: &str) -> bool {
        self.enabled_features.contains(feature)
    }

    /// Get list of all enabled features
    pub fn enabled_features(&self) -> Vec<String> {
        self.enabled_features.iter().cloned().collect()
    }

    /// Get plugin metadata by name
    pub fn get_metadata(&self, name: &str) -> Option<&PluginMetadata> {
        self.metadata.get(name)
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Check health of all plugins
    pub async fn health_check_all(&self) -> HashMap<String, PluginHealth> {
        let mut health_status = HashMap::new();
        
        for (name, plugin) in &self.plugins {
            match plugin.health_check().await {
                Ok(health) => {
                    health_status.insert(name.clone(), health);
                }
                Err(e) => {
                    health_status.insert(name.clone(), PluginHealth {
                        status: PluginStatus::Unhealthy,
                        message: format!("Health check failed: {}", e),
                        details: HashMap::new(),
                    });
                }
            }
        }
        
        health_status
    }

    /// Check if plugin system is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get count of registered plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub implementations for community edition
/// These provide no-op implementations of enterprise features

#[cfg(not(feature = "enterprise"))]
pub mod community_stubs {
    use super::*;
    
    /// Community edition plugin manager (no-op)
    pub struct CommunityPluginManager;
    
    impl CommunityPluginManager {
        pub fn new() -> Self {
            Self
        }
        
        pub fn has_feature(&self, _feature: &str) -> bool {
            false // Community edition has no enterprise features
        }
        
        pub fn enabled_features(&self) -> Vec<String> {
            vec![] // No enterprise features in community
        }
        
        pub async fn initialize_all(&mut self, _context: &mut RuntimeContext) -> Result<()> {
            // No-op for community edition
            Ok(())
        }
        
        pub async fn shutdown_all(&mut self) -> Result<()> {
            // No-op for community edition
            Ok(())
        }
        
        pub fn list_plugins(&self) -> Vec<String> {
            vec![] // No enterprise plugins in community
        }
        
        pub async fn health_check_all(&self) -> HashMap<String, PluginHealth> {
            HashMap::new() // No plugins to check in community
        }
        
        pub fn is_initialized(&self) -> bool {
            true // Always "initialized" (no-op)
        }
        
        pub fn plugin_count(&self) -> usize {
            0 // No plugins in community
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestPlugin {
        name: String,
        features: Vec<String>,
    }
    
    #[async_trait]
    impl EnterprisePlugin for TestPlugin {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn version(&self) -> &str {
            "1.0.0"
        }
        
        fn features(&self) -> Vec<String> {
            self.features.clone()
        }
        
        async fn initialize(&mut self, _context: &mut RuntimeContext) -> Result<()> {
            Ok(())
        }
        
        async fn shutdown(&mut self) -> Result<()> {
            Ok(())
        }
        
        async fn health_check(&self) -> Result<PluginHealth> {
            Ok(PluginHealth {
                status: PluginStatus::Healthy,
                message: "Test plugin is healthy".to_string(),
                details: HashMap::new(),
            })
        }
    }
    
    #[tokio::test]
    async fn test_plugin_manager_registration() {
        let mut manager = PluginManager::new();
        
        let plugin = Box::new(TestPlugin {
            name: "test_plugin".to_string(),
            features: vec!["test_feature".to_string()],
        });
        
        let metadata = PluginMetadata {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "RustChain".to_string(),
            license: "Commercial".to_string(),
            dependencies: vec![],
            capabilities: vec!["test".to_string()],
        };
        
        assert!(manager.register_plugin(plugin, metadata).is_ok());
        assert!(manager.has_feature("test_feature"));
        assert_eq!(manager.plugin_count(), 1);
    }
    
    #[tokio::test]
    async fn test_plugin_initialization() {
        let mut manager = PluginManager::new();
        let mut context = RuntimeContext::new();
        
        let plugin = Box::new(TestPlugin {
            name: "test_plugin".to_string(),
            features: vec!["test_feature".to_string()],
        });
        
        let metadata = PluginMetadata {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "RustChain".to_string(),
            license: "Commercial".to_string(),
            dependencies: vec![],
            capabilities: vec!["test".to_string()],
        };
        
        manager.register_plugin(plugin, metadata).unwrap();
        assert!(manager.initialize_all(&mut context).await.is_ok());
        assert!(manager.is_initialized());
    }
    
    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_community_stubs() {
        let manager = community_stubs::CommunityPluginManager::new();
        assert!(!manager.has_feature("any_feature"));
        assert_eq!(manager.enabled_features().len(), 0);
        assert_eq!(manager.plugin_count(), 0);
        assert!(manager.is_initialized());
    }
}