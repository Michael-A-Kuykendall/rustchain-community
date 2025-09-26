use crate::core::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait MemoryStore: Send + Sync {
    fn store(&mut self, key: &str, value: &str) -> Result<()>;
    fn retrieve(&self, key: &str) -> Result<Option<String>>;
    fn list_keys(&self) -> Result<Vec<String>>;
}

/// Enhanced memory entry with TTL support
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryEntry {
    value: String,
    created_at: u64,
    expires_at: Option<u64>,
}

impl MemoryEntry {
    fn new(value: String, ttl_seconds: Option<u64>) -> Self {
        let now_duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let now = now_duration.as_nanos() as u64;

        Self {
            value,
            created_at: now,
            expires_at: ttl_seconds.map(|ttl| now + (ttl * 1_000_000_000)), // Convert seconds to nanoseconds
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            now > expires_at
        } else {
            false
        }
    }
}

/// Enhanced in-memory store with TTL, cleanup, and additional operations
pub struct InMemoryStore {
    data: HashMap<String, MemoryEntry>,
    default_ttl: Option<u64>,
    max_entries: Option<usize>,
}

impl InMemoryStore {
    /// Create a new in-memory store with default settings
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: None,
            max_entries: None,
        }
    }

    /// Create a new in-memory store with TTL (overloaded for tests)
    pub fn with_ttl(ttl_seconds: u64) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: Some(ttl_seconds),
            max_entries: None,
        }
    }

    /// Create a new in-memory store with capacity limit
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: None,
            max_entries: Some(max_entries),
        }
    }

    /// Create a new in-memory store with both TTL and capacity limit
    pub fn with_ttl_and_capacity(ttl_seconds: u64, max_entries: usize) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: Some(ttl_seconds),
            max_entries: Some(max_entries),
        }
    }

    /// Clean up expired entries
    pub fn cleanup(&mut self) -> Result<()> {
        let expired_keys: Vec<String> = self
            .data
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.data.remove(&key);
        }

        Ok(())
    }

    /// Clear all entries
    pub fn clear(&mut self) -> Result<()> {
        self.data.clear();
        Ok(())
    }

    /// Get summary of memory store
    pub fn summarize(&self) -> Result<String> {
        let total_entries = self.data.len();
        let expired_entries = self
            .data
            .values()
            .filter(|entry| entry.is_expired())
            .count();
        let active_entries = total_entries - expired_entries;

        let total_size: usize = self.data.values().map(|entry| entry.value.len()).sum();

        Ok(format!(
            "Memory Store Summary: {} entries ({} active, {} expired), {} bytes total",
            total_entries, active_entries, expired_entries, total_size
        ))
    }

    /// Check if an entry exists and is not expired
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(entry) = self.data.get(key) {
            !entry.is_expired()
        } else {
            false
        }
    }

    /// Get memory usage statistics
    pub fn stats(&self) -> MemoryStats {
        let total_entries = self.data.len();
        let expired_entries = self
            .data
            .values()
            .filter(|entry| entry.is_expired())
            .count();
        let total_size: usize = self.data.values().map(|entry| entry.value.len()).sum();

        MemoryStats {
            total_entries,
            active_entries: total_entries - expired_entries,
            expired_entries,
            total_size_bytes: total_size,
            max_entries: self.max_entries,
            default_ttl: self.default_ttl,
        }
    }

    fn ensure_capacity(&mut self) -> Result<()> {
        if let Some(max_entries) = self.max_entries {
            // First try cleanup to free space
            self.cleanup()?;

            // If would exceed capacity after adding new entry, make room by removing oldest
            while self.data.len() >= max_entries {
                if let Some(oldest_key) = self
                    .data
                    .iter()
                    .min_by_key(|(_, entry)| entry.created_at)
                    .map(|(key, _)| key.clone())
                {
                    self.data.remove(&oldest_key);
                } else {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl MemoryStore for InMemoryStore {
    fn store(&mut self, key: &str, value: &str) -> Result<()> {
        // Ensure we don't exceed capacity
        self.ensure_capacity()?;

        let entry = MemoryEntry::new(value.to_string(), self.default_ttl);
        self.data.insert(key.to_string(), entry);
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Option<String>> {
        if let Some(entry) = self.data.get(key) {
            if entry.is_expired() {
                Ok(None)
            } else {
                Ok(Some(entry.value.clone()))
            }
        } else {
            Ok(None)
        }
    }

    fn list_keys(&self) -> Result<Vec<String>> {
        Ok(self
            .data
            .iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect())
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: usize,
    pub active_entries: usize,
    pub expired_entries: usize,
    pub total_size_bytes: usize,
    pub max_entries: Option<usize>,
    pub default_ttl: Option<u64>,
}

/// Conversation-specific memory for storing and managing chat history
#[derive(Debug, Clone)]
pub struct ConversationMemory {
    messages: VecDeque<ConversationMessage>,
    max_messages: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}

impl ConversationMessage {
    fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl ConversationMemory {
    /// Create a new conversation memory with specified capacity
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages,
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, role: &str, content: &str) -> Result<()> {
        // If max_messages is 0, don't store anything
        if self.max_messages == 0 {
            return Ok(());
        }

        let message = ConversationMessage::new(role, content);

        // Remove oldest message if at capacity
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }

        self.messages.push_back(message);
        Ok(())
    }

    /// Get the entire conversation as formatted strings
    pub fn get_conversation(&self) -> Result<Vec<String>> {
        Ok(self
            .messages
            .iter()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect())
    }

    /// Get the most recent N messages
    pub fn get_recent(&self, count: usize) -> Result<Vec<String>> {
        Ok(self
            .messages
            .iter()
            .rev()
            .take(count)
            .rev()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect())
    }

    /// Search for messages containing a specific term
    pub fn search(&self, term: &str) -> Result<Vec<String>> {
        let term_lower = term.to_lowercase();
        Ok(self
            .messages
            .iter()
            .filter(|msg| {
                msg.content.to_lowercase().contains(&term_lower)
                    || msg.role.to_lowercase().contains(&term_lower)
            })
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect())
    }

    /// Clear all messages
    pub fn clear(&mut self) -> Result<()> {
        self.messages.clear();
        Ok(())
    }

    /// Get summary of the conversation
    pub fn summarize(&self) -> Result<String> {
        let total_messages = self.messages.len();
        let roles: std::collections::HashSet<String> =
            self.messages.iter().map(|msg| msg.role.clone()).collect();

        Ok(format!(
            "Conversation summary: {} messages from {} participants",
            total_messages,
            roles.len()
        ))
    }

    /// Get conversation statistics
    pub fn stats(&self) -> ConversationStats {
        let mut role_counts: HashMap<String, usize> = HashMap::new();
        let mut total_chars = 0;

        for msg in &self.messages {
            *role_counts.entry(msg.role.clone()).or_insert(0) += 1;
            total_chars += msg.content.len();
        }

        ConversationStats {
            total_messages: self.messages.len(),
            role_counts,
            total_characters: total_chars,
            max_capacity: self.max_messages,
        }
    }
}

/// Conversation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStats {
    pub total_messages: usize,
    pub role_counts: HashMap<String, usize>,
    pub total_characters: usize,
    pub max_capacity: usize,
}

/// ContextLite Memory Store - Persistent storage backend
#[cfg(feature = "contextlite")]
pub struct ContextLiteStore {
    _endpoint: String,
    _agent_id: String,
    _client: reqwest::Client,
}

#[cfg(feature = "contextlite")]
impl ContextLiteStore {
    pub fn new(endpoint: String, agent_id: String) -> Self {
        Self {
            _endpoint: endpoint,
            _agent_id: agent_id,
            _client: reqwest::Client::new(),
        }
    }
}

#[cfg(feature = "contextlite")]
impl MemoryStore for ContextLiteStore {
    fn store(&mut self, key: &str, value: &str) -> Result<()> {
        // Synchronous implementation using tokio::task::block_in_place for async HTTP calls
        use tracing::{debug, error};
        
        debug!("Storing key '{}' in ContextLite (agent: {})", key, self._agent_id);
        
        let endpoint = self._endpoint.clone();
        let agent_id = self._agent_id.clone();
        let client = self._client.clone();
        let key_owned = key.to_string();
        let value_owned = value.to_string();
        
        // Use block_in_place to run async code in sync context
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let url = format!("{}/api/v1/agents/{}/memory", endpoint, agent_id);
                
                let payload = serde_json::json!({
                    "key": key_owned,
                    "value": value_owned,
                    "metadata": {
                        "timestamp": std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        "source": "rustchain"
                    }
                });
                
                let response = client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .timeout(std::time::Duration::from_millis(5000))
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            debug!("Successfully stored key '{}' in ContextLite", key_owned);
                            Ok(())
                        } else {
                            let status = resp.status();
                            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                            error!("ContextLite store failed with status {}: {}", status, error_text);
                            Err(crate::core::error::RustChainError::Memory(
                                crate::core::error::MemoryError::InvalidOperation {
                                    operation: format!("store key '{}'", key_owned),
                                    store_type: format!("ContextLite (status: {}, error: {})", status, error_text),
                                }
                            ))
                        }
                    }
                    Err(e) => {
                        error!("HTTP request to ContextLite failed: {}", e);
                        Err(crate::core::error::RustChainError::Memory(
                            crate::core::error::MemoryError::InvalidOperation {
                                operation: "HTTP request to ContextLite".to_string(),
                                store_type: format!("ContextLite (error: {})", e),
                            }
                        ))
                    }
                }
            })
        });
        
        result
    }
    
    fn retrieve(&self, key: &str) -> Result<Option<String>> {
        // Synchronous implementation using tokio::task::block_in_place for async HTTP calls
        use tracing::{debug, error, warn};
        
        debug!("Retrieving key '{}' from ContextLite (agent: {})", key, self._agent_id);
        
        let endpoint = self._endpoint.clone();
        let agent_id = self._agent_id.clone();
        let client = self._client.clone();
        let key_owned = key.to_string();
        
        // Use block_in_place to run async code in sync context
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let url = format!("{}/api/v1/agents/{}/memory/{}", endpoint, agent_id, 
                    urlencoding::encode(&key_owned));
                
                let response = client
                    .get(&url)
                    .header("Accept", "application/json")
                    .timeout(std::time::Duration::from_millis(5000))
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        let status = resp.status();
                        if status.is_success() {
                            let response_text = resp.text().await.unwrap_or_default();
                            
                            // Try to parse JSON response
                            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_text) {
                                if let Some(value) = json_value.get("value") {
                                    if let Some(value_str) = value.as_str() {
                                        debug!("Successfully retrieved key '{}' from ContextLite", key_owned);
                                        return Ok(Some(value_str.to_string()));
                                    }
                                }
                                // If no "value" field, return the whole response as string
                                Ok(Some(response_text))
                            } else {
                                // If not JSON, return raw text
                                Ok(Some(response_text))
                            }
                        } else if status == reqwest::StatusCode::NOT_FOUND {
                            debug!("Key '{}' not found in ContextLite", key_owned);
                            Ok(None)
                        } else {
                            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                            warn!("ContextLite retrieve failed with status {}: {}", status, error_text);
                            Err(crate::core::error::RustChainError::Memory(
                                crate::core::error::MemoryError::InvalidOperation {
                                    operation: format!("retrieve key '{}'", key_owned),
                                    store_type: format!("ContextLite (status: {}, error: {})", status, error_text),
                                }
                            ))
                        }
                    }
                    Err(e) => {
                        error!("HTTP request to ContextLite failed: {}", e);
                        // Return None for connectivity issues to allow graceful degradation
                        warn!("ContextLite connectivity issue, returning None: {}", e);
                        Ok(None)
                    }
                }
            })
        });
        
        result
    }
    
    fn list_keys(&self) -> Result<Vec<String>> {
        // Synchronous implementation using tokio::task::block_in_place for async HTTP calls
        use tracing::{debug, error, warn};
        
        debug!("Listing keys from ContextLite (agent: {})", self._agent_id);
        
        let endpoint = self._endpoint.clone();
        let agent_id = self._agent_id.clone();
        let client = self._client.clone();
        
        // Use block_in_place to run async code in sync context
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let url = format!("{}/api/v1/agents/{}/memory", endpoint, agent_id);
                
                let response = client
                    .get(&url)
                    .header("Accept", "application/json")
                    .timeout(std::time::Duration::from_millis(10000)) // Longer timeout for list operations
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        let status = resp.status();
                        if status.is_success() {
                            let response_text = resp.text().await.unwrap_or_default();
                            
                            // Try to parse JSON response containing list of keys
                            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response_text) {
                                let mut keys = Vec::new();
                                
                                // Handle different possible response formats
                                if let Some(keys_array) = json_value.get("keys") {
                                    if let Some(array) = keys_array.as_array() {
                                        for item in array {
                                            if let Some(key_str) = item.as_str() {
                                                keys.push(key_str.to_string());
                                            }
                                        }
                                    }
                                } else if let Some(data_array) = json_value.get("data") {
                                    if let Some(array) = data_array.as_array() {
                                        for item in array {
                                            if let Some(key) = item.get("key") {
                                                if let Some(key_str) = key.as_str() {
                                                    keys.push(key_str.to_string());
                                                }
                                            }
                                        }
                                    }
                                } else if let Some(array) = json_value.as_array() {
                                    // Direct array of keys or objects
                                    for item in array {
                                        if let Some(key_str) = item.as_str() {
                                            keys.push(key_str.to_string());
                                        } else if let Some(key) = item.get("key") {
                                            if let Some(key_str) = key.as_str() {
                                                keys.push(key_str.to_string());
                                            }
                                        }
                                    }
                                }
                                
                                debug!("Successfully listed {} keys from ContextLite", keys.len());
                                Ok(keys)
                            } else {
                                warn!("ContextLite list_keys returned non-JSON response");
                                Ok(Vec::new())
                            }
                        } else if status == reqwest::StatusCode::NOT_FOUND {
                            debug!("Agent '{}' not found in ContextLite, returning empty list", agent_id);
                            Ok(Vec::new())
                        } else {
                            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                            warn!("ContextLite list_keys failed with status {}: {}", status, error_text);
                            Err(crate::core::error::RustChainError::Memory(
                                crate::core::error::MemoryError::InvalidOperation {
                                    operation: "list_keys".to_string(),
                                    store_type: format!("ContextLite (status: {}, error: {})", status, error_text),
                                }
                            ))
                        }
                    }
                    Err(e) => {
                        error!("HTTP request to ContextLite failed: {}", e);
                        // Return empty list for connectivity issues to allow graceful degradation
                        warn!("ContextLite connectivity issue, returning empty list: {}", e);
                        Ok(Vec::new())
                    }
                }
            })
        });
        
        result
    }
}

// Include the tests module
#[cfg(test)]
mod tests;

