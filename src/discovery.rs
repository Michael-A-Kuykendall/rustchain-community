use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredModel {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub format: ModelFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelFormat {
    Gguf,
    SafeTensors,
    Unknown,
}

pub struct ModelDiscovery {
    search_paths: Vec<PathBuf>,
}

impl ModelDiscovery {
    pub fn new() -> Self {
        Self {
            search_paths: vec![],
        }
    }

    pub fn discover_models(&self) -> anyhow::Result<Vec<DiscoveredModel>> {
        Ok(vec![])
    }
}
