//! ContextLite Configuration and Factory

use crate::core::memory::ContextLiteStore;

#[derive(Debug, Clone)]
pub struct ContextLiteConfig {
    pub endpoint: String,
    pub agent_id: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub enable_semantic_search: bool,
}

impl Default for ContextLiteConfig {
    fn default() -> Self {
        Self {
            endpoint: std::env::var("CONTEXTLITE_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            agent_id: std::env::var("CONTEXTLITE_AGENT_ID")
                .unwrap_or_else(|_| "rustchain_agent".to_string()),
            timeout_ms: 5000,
            max_retries: 3,
            enable_semantic_search: true,
        }
    }
}

impl ContextLiteConfig {
    pub fn create_store(&self) -> ContextLiteStore {
        ContextLiteStore::new(
            self.endpoint.clone(),
            self.agent_id.clone(),
        )
    }
}
