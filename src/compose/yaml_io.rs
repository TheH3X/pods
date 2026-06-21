use std::fs;
use std::path::{Path, PathBuf};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::{ComposeService, Network, Stack, StackLayout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub services: IndexMap<String, ComposeService>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub networks: IndexMap<String, Network>,
    
    // Everything else
    #[serde(flatten)]
    pub extras: IndexMap<String, Value>,
}

pub fn read_compose_file(path: &Path) -> Result<(IndexMap<String, ComposeService>, IndexMap<String, Network>), anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let compose: ComposeFile = serde_yaml::from_str(&content)?;
    Ok((compose.services, compose.networks))
}

pub fn write_compose_file(path: &Path, services: &IndexMap<String, ComposeService>, networks: &IndexMap<String, Network>) -> Result<(), anyhow::Error> {
    let mut compose = if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str::<ComposeFile>(&content).unwrap_or(ComposeFile {
            version: None,
            services: IndexMap::new(),
            networks: IndexMap::new(),
            extras: IndexMap::new(),
        })
    } else {
        ComposeFile {
            version: None,
            services: IndexMap::new(),
            networks: IndexMap::new(),
            extras: IndexMap::new(),
        }
    };

    compose.services = services.clone();
    compose.networks = networks.clone();

    let yaml = serde_yaml::to_string(&compose)?;
    fs::write(path, yaml)?;
    Ok(())
}
