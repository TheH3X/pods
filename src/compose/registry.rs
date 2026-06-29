use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Registry browser for searching and browsing container registries.
/// Supports Docker Hub and GHCR APIs.
#[derive(Debug, Clone)]
pub struct RegistryBrowser {
    pub registry: RegistryType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegistryType {
    DockerHub,
    Ghcr,
    Custom(String),
}

/// Search result from a registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearchResult {
    pub name: String,
    pub description: String,
    pub stars: u64,
    pub official: bool,
    pub automated: bool,
}

/// Tag information from a registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryTag {
    pub name: String,
    pub digest: Option<String>,
    pub last_updated: Option<String>,
    pub size: Option<u64>,
}

impl RegistryBrowser {
    pub fn docker_hub() -> Self {
        Self {
            registry: RegistryType::DockerHub,
        }
    }

    pub fn ghcr() -> Self {
        Self {
            registry: RegistryType::Ghcr,
        }
    }

    /// Search for images in the registry.
    ///
    /// Note: This is a minimal implementation. The existing Pods
    /// `ImageRemoteSelectionPage` and image search infrastructure handles
    /// the actual HTTP calls via Bollard/Docker API. This module provides
    /// additional structured access when needed.
    pub fn search_url(&self, query: &str) -> String {
        match &self.registry {
            RegistryType::DockerHub => {
                format!(
                    "https://hub.docker.com/v2/search/repositories/?query={}&page_size=25",
                    urlencoded(query)
                )
            }
            RegistryType::Ghcr => {
                // GHCR doesn't have a public search API,
                // but we can construct the packages URL
                format!(
                    "https://github.com/orgs/{}/packages?type=container",
                    urlencoded(query)
                )
            }
            RegistryType::Custom(base_url) => {
                format!("{}/v2/_catalog", base_url)
            }
        }
    }

    /// Get the tags URL for an image.
    pub fn tags_url(&self, image: &str) -> String {
        match &self.registry {
            RegistryType::DockerHub => {
                let name = if image.contains('/') {
                    image.to_string()
                } else {
                    format!("library/{}", image)
                };
                format!(
                    "https://hub.docker.com/v2/repositories/{}/tags/?page_size=100",
                    name
                )
            }
            RegistryType::Ghcr => {
                format!("https://ghcr.io/v2/{}/tags/list", image)
            }
            RegistryType::Custom(base_url) => {
                format!("{}/v2/{}/tags/list", base_url, image)
            }
        }
    }

    /// Parse a Docker Hub search response.
    pub fn parse_docker_hub_search(json: &str) -> Result<Vec<RegistrySearchResult>> {
        let response: serde_json::Value =
            serde_json::from_str(json).context("Failed to parse Docker Hub search response")?;

        let results = response
            .get("results")
            .and_then(|r| r.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(RegistrySearchResult {
                            name: item.get("repo_name")?.as_str()?.to_string(),
                            description: item
                                .get("short_description")
                                .and_then(|d| d.as_str())
                                .unwrap_or("")
                                .to_string(),
                            stars: item.get("star_count").and_then(|s| s.as_u64()).unwrap_or(0),
                            official: item
                                .get("is_official")
                                .and_then(|b| b.as_bool())
                                .unwrap_or(false),
                            automated: item
                                .get("is_automated")
                                .and_then(|b| b.as_bool())
                                .unwrap_or(false),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(results)
    }

    /// Parse a Docker Hub tags response.
    pub fn parse_docker_hub_tags(json: &str) -> Result<Vec<RegistryTag>> {
        let response: serde_json::Value =
            serde_json::from_str(json).context("Failed to parse Docker Hub tags response")?;

        let tags = response
            .get("results")
            .and_then(|r| r.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(RegistryTag {
                            name: item.get("name")?.as_str()?.to_string(),
                            digest: item
                                .get("digest")
                                .and_then(|d| d.as_str())
                                .map(String::from),
                            last_updated: item
                                .get("last_updated")
                                .and_then(|d| d.as_str())
                                .map(String::from),
                            size: item.get("full_size").and_then(|s| s.as_u64()),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(tags)
    }
}

/// Simple URL encoding for query parameters.
fn urlencoded(s: &str) -> String {
    s.replace(' ', "+")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('#', "%23")
}
