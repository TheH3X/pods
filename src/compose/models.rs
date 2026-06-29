use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Layout type for a stack on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackLayout {
    /// A single docker-compose.yml with all services
    Flat,
    /// A folder with per-service subdirectories containing compose.yml
    Nested,
}

/// A discovered stack: a collection of services with a shared root path.
#[derive(Debug, Clone)]
pub struct Stack {
    pub name: String,
    pub root_path: std::path::PathBuf,
    pub layout: StackLayout,
    pub services: Vec<ComposeService>,
    pub networks: Vec<Network>,
}

/// A single compose service definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeService {
    /// Service name from the compose map key (not serialized into YAML)
    #[serde(skip)]
    pub name: String,
    /// For nested layout, the path to the service's directory
    #[serde(skip)]
    pub folder_path: Option<std::path::PathBuf>,

    /// Container image
    pub image: Option<String>,
    /// Explicit container name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// Restart policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    /// Build context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Value>,
    /// Command override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Value>,
    /// Entrypoint override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Value>,
    /// Depends-on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,

    /// Port mappings (e.g., "8080:80")
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
    /// Volume/bind mounts (e.g., "./data:/app/data")
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<String>,
    /// Environment variables
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub environment: IndexMap<String, Value>,
    /// Labels
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub labels: IndexMap<String, String>,

    /// Networks this service belongs to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub networks: Vec<String>,

    /// All unmodeled keys go here to be preserved when writing back (raw-merge)
    #[serde(flatten)]
    pub extras: IndexMap<String, Value>,
}

impl Default for ComposeService {
    fn default() -> Self {
        Self {
            name: String::new(),
            folder_path: None,
            image: None,
            container_name: None,
            restart: Some("unless-stopped".to_string()),
            build: None,
            command: None,
            entrypoint: None,
            depends_on: Vec::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            environment: IndexMap::new(),
            labels: IndexMap::new(),
            networks: Vec::new(),
            extras: IndexMap::new(),
        }
    }
}

impl ComposeService {
    /// Create a new service with just a name and image.
    pub fn new(name: &str, image: &str) -> Self {
        Self {
            name: name.to_string(),
            image: Some(image.to_string()),
            ..Default::default()
        }
    }

    /// Get the first port mapping's external port, if any.
    pub fn first_external_port(&self) -> Option<String> {
        self.ports.first().and_then(|p| {
            let parts: Vec<&str> = p.split(':').collect();
            parts.first().map(|s| s.to_string())
        })
    }

    /// Add a port mapping in "host:container" format.
    pub fn add_port(&mut self, host: &str, container: &str) {
        self.ports.push(format!("{}:{}", host, container));
    }

    /// Add a volume/bind mount in "host:container" format.
    pub fn add_volume(&mut self, host: &str, container: &str) {
        self.volumes.push(format!("{}:{}", host, container));
    }

    /// Add an environment variable.
    pub fn add_env(&mut self, key: &str, value: &str) {
        self.environment
            .insert(key.to_string(), Value::String(value.to_string()));
    }

    /// Add a label.
    pub fn add_label(&mut self, key: &str, value: &str) {
        self.labels.insert(key.to_string(), value.to_string());
    }
}

/// A Docker network definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Network name from the compose map key (not serialized into YAML)
    #[serde(skip)]
    pub name: String,

    /// Network driver (e.g., "bridge", "overlay")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Whether this is an external network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    /// IPAM configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Value>,
    /// Enable IPv6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,

    /// All unmodeled keys
    #[serde(flatten)]
    pub extras: IndexMap<String, Value>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            name: String::new(),
            driver: Some("bridge".to_string()),
            external: None,
            ipam: None,
            enable_ipv6: None,
            extras: IndexMap::new(),
        }
    }
}

impl Network {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Create an external network reference.
    pub fn external(name: &str) -> Self {
        Self {
            name: name.to_string(),
            external: Some(true),
            driver: None,
            ..Default::default()
        }
    }

    /// Extract subnet from IPAM config, if present.
    pub fn subnet(&self) -> Option<String> {
        self.ipam.as_ref().and_then(|ipam| {
            ipam.get("config")
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|entry| entry.get("subnet"))
                .and_then(|s| s.as_str())
                .map(String::from)
        })
    }
}
