use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata extracted from an OCI/Docker image configuration.
/// Used to auto-populate service configurations from image declarations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImageMetadata {
    /// Declared volumes from OCI Config.Volumes (e.g., ["/data", "/config"])
    pub declared_volumes: Vec<String>,
    /// Exposed ports from OCI Config.ExposedPorts (e.g., ["8080/tcp", "443/tcp"])
    pub exposed_ports: Vec<String>,
    /// Default environment variables from OCI Config.Env
    pub env_defaults: HashMap<String, String>,
    /// Working directory from OCI Config
    pub working_dir: String,
    /// Image-level labels
    pub labels: HashMap<String, String>,
    /// Entrypoint
    pub entrypoint: Vec<String>,
    /// Default command
    pub cmd: Vec<String>,
}

impl ImageMetadata {
    /// Extract metadata from a Docker image inspection result (from Bollard).
    ///
    /// This parses the OCI image config to extract declared volumes,
    /// exposed ports, environment variable defaults, etc.
    pub fn from_inspect_response(inspect: &serde_json::Value) -> Self {
        let mut meta = ImageMetadata::default();

        // Extract from Config object
        if let Some(config) = inspect.get("Config").or_else(|| inspect.get("config")) {
            // Volumes: {"Volumes": {"/data": {}, "/config": {}}}
            if let Some(volumes) = config.get("Volumes").and_then(|v| v.as_object()) {
                meta.declared_volumes = volumes.keys().cloned().collect();
                meta.declared_volumes.sort();
            }

            // ExposedPorts: {"ExposedPorts": {"8080/tcp": {}, "443/tcp": {}}}
            if let Some(ports) = config.get("ExposedPorts").and_then(|v| v.as_object()) {
                meta.exposed_ports = ports.keys().cloned().collect();
                meta.exposed_ports.sort();
            }

            // Env: ["KEY=VALUE", ...]
            if let Some(env) = config.get("Env").and_then(|v| v.as_array()) {
                for entry in env {
                    if let Some(s) = entry.as_str() {
                        if let Some((key, val)) = s.split_once('=') {
                            meta.env_defaults.insert(key.to_string(), val.to_string());
                        }
                    }
                }
            }

            // WorkingDir
            if let Some(wd) = config.get("WorkingDir").and_then(|v| v.as_str()) {
                meta.working_dir = wd.to_string();
            }

            // Labels
            if let Some(labels) = config.get("Labels").and_then(|v| v.as_object()) {
                for (k, v) in labels {
                    if let Some(val) = v.as_str() {
                        meta.labels.insert(k.clone(), val.to_string());
                    }
                }
            }

            // Entrypoint
            if let Some(ep) = config.get("Entrypoint").and_then(|v| v.as_array()) {
                meta.entrypoint = ep
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }

            // Cmd
            if let Some(cmd) = config.get("Cmd").and_then(|v| v.as_array()) {
                meta.cmd = cmd
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
            }
        }

        meta
    }

    /// Generate bind mount suggestions from the image's declared volumes.
    ///
    /// Maps each declared volume to `./appdata/<volume_path>`.
    pub fn volume_suggestions(&self, service_name: &str) -> Vec<(String, String)> {
        self.declared_volumes
            .iter()
            .map(|vol| {
                let subpath = vol.trim_start_matches('/');
                let host_path = format!("./appdata/{}", subpath);
                (host_path, vol.clone())
            })
            .collect()
    }

    /// Generate port mapping suggestions from the image's exposed ports.
    ///
    /// Suggests host:container mappings using the same port number.
    pub fn port_suggestions(&self) -> Vec<(String, String)> {
        self.exposed_ports
            .iter()
            .filter_map(|port_spec| {
                let port_num = port_spec.split('/').next().unwrap_or(port_spec);
                Some((port_num.to_string(), port_num.to_string()))
            })
            .collect()
    }

    /// Get environment variable defaults as a display-friendly list.
    pub fn env_display(&self) -> Vec<(String, String)> {
        let mut envs: Vec<_> = self
            .env_defaults
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        envs.sort_by(|a, b| a.0.cmp(&b.0));
        envs
    }
}
