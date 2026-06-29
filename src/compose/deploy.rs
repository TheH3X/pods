use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::backup;
use super::models::{ComposeService, Network, Stack, StackLayout};
use super::plan::PlanState;
use super::yaml_io::{self, ComposeFile};

/// Deployment options controlling how the plan state is written to disk.
#[derive(Debug, Clone)]
pub struct DeployOptions {
    /// Whether to create a backup before writing.
    pub create_backup: bool,
    /// Whether to run `docker compose up -d` after writing.
    pub compose_up: bool,
    /// Whether to pull images before composing up.
    pub pull_first: bool,
}

impl Default for DeployOptions {
    fn default() -> Self {
        Self {
            create_backup: true,
            compose_up: true,
            pull_first: false,
        }
    }
}

/// Write the plan state to disk, using the stack's layout (flat or nested).
///
/// This is the main entry point for deploying a plan.
pub fn write_to_disk(
    stack_path: &Path,
    layout: &StackLayout,
    plan: &PlanState,
    options: &DeployOptions,
) -> Result<Vec<PathBuf>> {
    // Create backup if requested
    if options.create_backup {
        let _ = backup::create_snapshot(stack_path);
    }

    let mut written_files = Vec::new();

    match layout {
        StackLayout::Flat => {
            let compose_path = find_compose_file(stack_path);
            let doc = build_stack_doc(&plan.working.services, &plan.working.networks);
            yaml_io::write_compose_file(&compose_path, &doc.services, &doc.networks)?;
            written_files.push(compose_path);
        }
        StackLayout::Nested => {
            for (name, service) in &plan.working.services {
                let service_dir = stack_path.join(name);
                fs::create_dir_all(&service_dir).with_context(|| {
                    format!(
                        "Failed to create service directory: {}",
                        service_dir.display()
                    )
                })?;

                let compose_path = service_dir.join("compose.yml");
                let doc = build_service_doc(name, service, &plan.working.networks);
                yaml_io::write_compose_file(&compose_path, &doc.services, &doc.networks)?;
                written_files.push(compose_path);

                // Create appdata dir if it doesn't exist
                let appdata_dir = service_dir.join("appdata");
                let _ = fs::create_dir_all(&appdata_dir);
            }

            // Remove directories for deleted services
            for name in plan.original.services.keys() {
                if !plan.working.services.contains_key(name) {
                    let service_dir = stack_path.join(name);
                    if service_dir.exists() {
                        // Don't delete — just log. User should explicitly clean up.
                        log::warn!(
                            "Service '{}' removed from plan but directory still exists: {}",
                            name,
                            service_dir.display()
                        );
                    }
                }
            }
        }
    }

    Ok(written_files)
}

/// Build a ComposeFile document for a single service (nested layout).
pub fn build_service_doc(
    name: &str,
    service: &ComposeService,
    all_networks: &IndexMap<String, Network>,
) -> ComposeFile {
    let mut services = IndexMap::new();
    services.insert(name.to_string(), service.clone());

    // Only include networks that this service references
    let mut networks = IndexMap::new();
    for net_name in &service.networks {
        if let Some(net) = all_networks.get(net_name) {
            networks.insert(net_name.clone(), net.clone());
        }
    }

    ComposeFile {
        version: None,
        services,
        networks,
        extras: IndexMap::new(),
    }
}

/// Build a ComposeFile document for an entire stack (flat layout).
pub fn build_stack_doc(
    services: &IndexMap<String, ComposeService>,
    networks: &IndexMap<String, Network>,
) -> ComposeFile {
    ComposeFile {
        version: None,
        services: services.clone(),
        networks: networks.clone(),
        extras: IndexMap::new(),
    }
}

/// Generate a manifest listing all services and their files.
/// Useful for tracking what files belong to a stack.
pub fn build_manifest(
    stack_path: &Path,
    layout: &StackLayout,
    services: &IndexMap<String, ComposeService>,
) -> String {
    let mut manifest = String::new();
    manifest.push_str(&format!("# Stacks Manifest — {}\n", stack_path.display()));
    manifest.push_str(&format!("# Layout: {:?}\n", layout));
    manifest.push_str(&format!("# Services: {}\n\n", services.len()));

    match layout {
        StackLayout::Flat => {
            let compose_file = find_compose_file(stack_path);
            manifest.push_str(&format!("{}\n", compose_file.display()));
        }
        StackLayout::Nested => {
            for name in services.keys() {
                let compose_file = stack_path.join(name).join("compose.yml");
                manifest.push_str(&format!("{}\n", compose_file.display()));

                let appdata = stack_path.join(name).join("appdata");
                if appdata.exists() {
                    manifest.push_str(&format!("  appdata/  (data directory)\n"));
                }
            }
        }
    }

    manifest
}

/// Find the compose file in a flat-layout stack directory.
fn find_compose_file(stack_path: &Path) -> PathBuf {
    let docker_compose = stack_path.join("docker-compose.yml");
    let compose = stack_path.join("compose.yml");

    if docker_compose.exists() {
        docker_compose
    } else {
        compose
    }
}
