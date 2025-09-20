use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredModel {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub format: ModelFormat,
    pub metadata: Option<ModelMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelFormat {
    Gguf,
    SafeTensors,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub architecture: Option<String>,
    pub parameter_count: Option<u64>,
    pub quantization: Option<String>,
}

pub struct ModelDiscovery {
    search_paths: Vec<PathBuf>,
}

impl ModelDiscovery {
    pub fn new() -> Self {
        let mut search_paths = Vec::new();
        
        // Add environment variable paths
        if let Ok(base_path) = std::env::var("SHIMMY_BASE_GGUF") {
            if let Some(parent) = Path::new(&base_path).parent() {
                search_paths.push(parent.to_path_buf());
            }
        }
        
        // Add common model directories
        if let Ok(home) = std::env::var("HOME") {
            search_paths.push(PathBuf::from(home).join(".cache/huggingface"));
            search_paths.push(PathBuf::from(home).join("models"));
        }
        
        Self { search_paths }
    }

    pub fn discover_models(&self) -> Result<Vec<DiscoveredModel>> {
        let mut models = Vec::new();
        
        for search_path in &self.search_paths {
            if search_path.exists() {
                self.scan_directory(search_path, &mut models)?;
            }
        }
        
        Ok(models)
    }

    fn scan_directory(&self, dir: &Path, models: &mut Vec<DiscoveredModel>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.scan_directory(&path, models)?;
            } else if self.is_model_file(&path) {
                if let Ok(model) = self.analyze_model_file(&path) {
                    models.push(model);
                }
            }
        }
        Ok(())
    }

    fn is_model_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_str(), Some("gguf") | Some("safetensors"))
        } else {
            false
        }
    }

    fn analyze_model_file(&self, path: &Path) -> Result<DiscoveredModel> {
        let metadata = fs::metadata(path)?;
        let format = self.detect_format(path)?;
        
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(DiscoveredModel {
            name,
            path: path.to_path_buf(),
            size_bytes: metadata.len(),
            format,
            metadata: self.extract_metadata(path).ok(),
        })
    }

    fn detect_format(&self, path: &Path) -> Result<ModelFormat> {
        match path.extension().and_then(|s| s.to_str()) {
            Some("gguf") => Ok(ModelFormat::Gguf),
            Some("safetensors") => Ok(ModelFormat::SafeTensors),
            _ => Ok(ModelFormat::Unknown),
        }
    }

    fn extract_metadata(&self, path: &Path) -> Result<ModelMetadata> {
        // Returns basic metadata structure
        // Future enhancement: GGUF header parsing for detailed model information
        Ok(ModelMetadata {
            architecture: None,
            parameter_count: None,
            quantization: None,
        })
    }
}
