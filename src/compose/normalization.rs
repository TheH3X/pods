use anyhow::Result;
use std::path::Path;

use super::models::ComposeService;

/// Result of a portability check on a service's bind mounts.
#[derive(Debug, Clone)]
pub struct PortabilityReport {
    pub service_name: String,
    pub issues: Vec<PortabilityIssue>,
}

/// A specific portability issue with a bind mount.
#[derive(Debug, Clone)]
pub struct PortabilityIssue {
    pub volume_spec: String,
    pub issue_type: IssueType,
    pub suggestion: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    /// Absolute path outside the stack directory
    AbsolutePathOutsideStack,
    /// Home directory reference (~/)
    HomeDirReference,
    /// Hardcoded /etc, /var, etc.
    SystemPathReference,
    /// Can be migrated to appdata/
    MigratableToAppdata,
}

/// Check all bind mounts in a service for portability issues.
///
/// A portable service should have all its bind mounts relative to its
/// service directory (or using the `appdata/` convention).
pub fn check_bind_portability(service: &ComposeService, stack_root: &Path) -> PortabilityReport {
    let mut issues = Vec::new();

    for volume_spec in &service.volumes {
        // Parse the volume spec — format is typically "host:container[:options]"
        let parts: Vec<&str> = volume_spec.splitn(3, ':').collect();
        if parts.len() < 2 {
            continue; // Named volume or invalid, skip
        }

        let host_path = parts[0];

        // Check for absolute paths
        if host_path.starts_with('/') {
            let abs_path = Path::new(host_path);

            // Check if it's within the stack directory
            if !abs_path.starts_with(stack_root) {
                // System paths
                if host_path.starts_with("/etc")
                    || host_path.starts_with("/var")
                    || host_path.starts_with("/usr")
                    || host_path.starts_with("/opt")
                {
                    issues.push(PortabilityIssue {
                        volume_spec: volume_spec.clone(),
                        issue_type: IssueType::SystemPathReference,
                        suggestion: format!(
                            "Consider using a relative path like ./appdata{} instead of {}",
                            host_path, host_path
                        ),
                    });
                } else {
                    issues.push(PortabilityIssue {
                        volume_spec: volume_spec.clone(),
                        issue_type: IssueType::AbsolutePathOutsideStack,
                        suggestion: format!(
                            "Move data to ./appdata/ and use a relative bind mount for portability"
                        ),
                    });
                }
            }
        }

        // Check for home directory references
        if host_path.starts_with("~/") || host_path.starts_with("$HOME") {
            issues.push(PortabilityIssue {
                volume_spec: volume_spec.clone(),
                issue_type: IssueType::HomeDirReference,
                suggestion: format!(
                    "Replace {} with a relative path like ./appdata/{}",
                    host_path,
                    host_path
                        .trim_start_matches("~/")
                        .trim_start_matches("$HOME/")
                ),
            });
        }

        // Check if a volume could be migrated to appdata/
        if host_path.starts_with("./") && !host_path.starts_with("./appdata") {
            issues.push(PortabilityIssue {
                volume_spec: volume_spec.clone(),
                issue_type: IssueType::MigratableToAppdata,
                suggestion: format!(
                    "Consider moving {} under ./appdata/ for consistent data organization",
                    host_path
                ),
            });
        }
    }

    PortabilityReport {
        service_name: service.name.clone(),
        issues,
    }
}

/// Suggest appdata directory moves for all services in a stack.
pub fn suggest_appdata_moves(
    services: &[ComposeService],
    stack_root: &Path,
) -> Vec<PortabilityReport> {
    services
        .iter()
        .map(|svc| check_bind_portability(svc, stack_root))
        .filter(|report| !report.issues.is_empty())
        .collect()
}

/// Rewrite a volume spec to use the appdata convention.
pub fn rewrite_to_appdata(volume_spec: &str, service_name: &str) -> Option<String> {
    let parts: Vec<&str> = volume_spec.splitn(3, ':').collect();
    if parts.len() < 2 {
        return None;
    }

    let host_path = parts[0];
    let container_path = parts[1];
    let options = parts.get(2).copied();

    // Determine the appdata subpath from the container path
    let subpath = container_path.trim_start_matches('/');
    let new_host_path = format!("./appdata/{}", subpath);

    let mut new_spec = format!("{}:{}", new_host_path, container_path);
    if let Some(opts) = options {
        new_spec.push(':');
        new_spec.push_str(opts);
    }

    Some(new_spec)
}
