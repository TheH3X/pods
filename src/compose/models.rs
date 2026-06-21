use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackLayout {
    Flat,   // A single docker-compose.yml with all services
    Nested, // A folder with per-service subdirectories containing compose.yml
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub name: String,
    pub root_path: std::path::PathBuf,
    pub layout: StackLayout,
    pub services: Vec<ComposeService>,
    pub networks: Vec<Network>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeService {
    #[serde(skip)]
    pub name: String, // Service name from the compose map key
    #[serde(skip)]
    pub folder_path: Option<std::path::PathBuf>, // For nested layout

    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub environment: IndexMap<String, Value>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub labels: IndexMap<String, String>,

    // For networks, we might have lists or maps. Keep it simple as a list of strings for now,
    // or handle robustly via raw extra merge if complex.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub networks: Vec<String>,

    // All unmodeled keys go here to be preserved when writing back
    #[serde(flatten)]
    pub extras: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    #[serde(skip)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,

    #[serde(flatten)]
    pub extras: IndexMap<String, Value>,
}
