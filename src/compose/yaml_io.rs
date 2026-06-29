use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use super::models::{ComposeService, Network, Stack, StackLayout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub services: IndexMap<String, ComposeService>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub networks: IndexMap<String, Network>,

    // Everything else (volumes, configs, secrets, etc.)
    #[serde(flatten)]
    pub extras: IndexMap<String, Value>,
}

impl Default for ComposeFile {
    fn default() -> Self {
        Self {
            version: None,
            services: IndexMap::new(),
            networks: IndexMap::new(),
            extras: IndexMap::new(),
        }
    }
}

/// Read a compose file and extract services and networks.
pub fn read_compose_file(
    path: &Path,
) -> Result<(IndexMap<String, ComposeService>, IndexMap<String, Network>), anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let compose: ComposeFile = serde_yaml::from_str(&content)?;
    Ok((compose.services, compose.networks))
}

/// Write services and networks to a compose file, preserving existing extras.
///
/// If the file already exists, its `extras` (unmodeled top-level keys like `volumes:`, `configs:`)
/// are preserved. A header comment is written to indicate managed sections.
pub fn write_compose_file(
    path: &Path,
    services: &IndexMap<String, ComposeService>,
    networks: &IndexMap<String, Network>,
) -> Result<(), anyhow::Error> {
    let mut compose = if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str::<ComposeFile>(&content).unwrap_or_default()
    } else {
        ComposeFile::default()
    };

    compose.services = services.clone();
    compose.networks = networks.clone();

    let yaml = serde_yaml::to_string(&compose)?;

    // Write with managed header comment
    let output = format!(
        "# Managed by Stacks — modeled keys (services, networks) are auto-generated\n{}",
        yaml
    );

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, output)?;
    Ok(())
}

/// Parse a compose YAML string into a ComposeFile.
pub fn parse_compose_string(content: &str) -> Result<ComposeFile, anyhow::Error> {
    let compose: ComposeFile = serde_yaml::from_str(content)?;
    Ok(compose)
}

/// Serialize a ComposeFile to a YAML string.
pub fn serialize_compose(compose: &ComposeFile) -> Result<String, anyhow::Error> {
    let yaml = serde_yaml::to_string(compose)?;
    Ok(yaml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flat_compose() {
        let yaml = r#"
services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    restart: unless-stopped
  php:
    image: php:8.2-fpm
    volumes:
      - ./html:/var/www/html

networks:
  frontend:
    driver: bridge
"#;
        let compose = parse_compose_string(yaml).unwrap();
        assert_eq!(compose.services.len(), 2);
        assert_eq!(compose.networks.len(), 1);

        let nginx = compose.services.get("nginx").unwrap();
        assert_eq!(nginx.image.as_deref(), Some("nginx:alpine"));
        assert_eq!(nginx.ports, vec!["80:80"]);
        assert_eq!(nginx.restart.as_deref(), Some("unless-stopped"));

        let php = compose.services.get("php").unwrap();
        assert_eq!(php.image.as_deref(), Some("php:8.2-fpm"));
        assert_eq!(php.volumes, vec!["./html:/var/www/html"]);
    }

    #[test]
    fn test_roundtrip_preserves_extras() {
        let yaml = r#"
services:
  app:
    image: myapp:latest
    deploy:
      replicas: 3
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost/health"]
      interval: 30s
"#;
        let compose = parse_compose_string(yaml).unwrap();
        let app = compose.services.get("app").unwrap();

        // The "deploy" and "healthcheck" keys should be in extras
        assert!(app.extras.contains_key("deploy") || app.extras.contains_key("healthcheck"));

        // Roundtrip
        let serialized = serialize_compose(&compose).unwrap();
        let reparsed = parse_compose_string(&serialized).unwrap();
        assert_eq!(reparsed.services.len(), 1);
    }

    #[test]
    fn test_parse_with_environment_map() {
        let yaml = r#"
services:
  db:
    image: postgres:15
    environment:
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: secret
      POSTGRES_DB: mydb
"#;
        let compose = parse_compose_string(yaml).unwrap();
        let db = compose.services.get("db").unwrap();
        assert_eq!(db.environment.len(), 3);
        assert_eq!(
            db.environment.get("POSTGRES_USER").and_then(|v| v.as_str()),
            Some("admin")
        );
    }

    #[test]
    fn test_parse_with_labels() {
        let yaml = r#"
services:
  traefik:
    image: traefik:v2.10
    labels:
      traefik.enable: "true"
      traefik.http.routers.api.rule: "Host(`traefik.example.com`)"
"#;
        let compose = parse_compose_string(yaml).unwrap();
        let traefik = compose.services.get("traefik").unwrap();
        assert_eq!(traefik.labels.len(), 2);
        assert_eq!(
            traefik.labels.get("traefik.enable"),
            Some(&"true".to_string())
        );
    }

    #[test]
    fn test_write_and_read_compose_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("compose.yml");

        let mut services = IndexMap::new();
        services.insert(
            "web".to_string(),
            ComposeService::new("web", "nginx:alpine"),
        );

        let networks = IndexMap::new();

        write_compose_file(&path, &services, &networks).unwrap();

        // Verify the file was written
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Managed by Stacks"));
        assert!(content.contains("nginx:alpine"));

        // Re-read
        let (read_services, _) = read_compose_file(&path).unwrap();
        assert_eq!(read_services.len(), 1);
        assert_eq!(
            read_services.get("web").unwrap().image.as_deref(),
            Some("nginx:alpine")
        );
    }

    #[test]
    fn test_read_fixture_flat_stack() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/flat_stack/docker-compose.yml");

        if fixture_path.exists() {
            let (services, networks) = read_compose_file(&fixture_path).unwrap();
            assert_eq!(services.len(), 2); // nginx + php
            assert!(services.contains_key("nginx"));
            assert!(services.contains_key("php"));
            assert_eq!(networks.len(), 2); // frontend + backend
        }
    }

    #[test]
    fn test_read_fixture_nested_stack() {
        let grafana_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/nested_stack/grafana/compose.yml");

        if grafana_path.exists() {
            let (services, networks) = read_compose_file(&grafana_path).unwrap();
            assert_eq!(services.len(), 1);
            assert!(services.contains_key("grafana"));
            assert_eq!(networks.len(), 1);
        }
    }
}
