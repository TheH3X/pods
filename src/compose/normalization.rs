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
pub fn rewrite_to_appdata(volume_spec: &str, _service_name: &str) -> Option<String> {
    let parts: Vec<&str> = volume_spec.splitn(3, ':').collect();
    if parts.len() < 2 {
        return None;
    }

    let _host_path = parts[0];
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::models::ComposeService;
    use std::path::Path;

    fn make_service_with_volumes(name: &str, volumes: &[&str]) -> ComposeService {
        let mut svc = ComposeService::new(name, "nginx:latest");
        svc.volumes = volumes.iter().map(|v| v.to_string()).collect();
        svc
    }

    #[test]
    fn test_absolute_path_outside_stack_flagged() {
        let svc = make_service_with_volumes("web", &["/data/myapp:/app/data"]);
        let report = check_bind_portability(&svc, Path::new("/stacks/mystack"));
        assert!(!report.issues.is_empty());
        assert!(report.issues.iter().any(|i| i.issue_type == IssueType::AbsolutePathOutsideStack));
    }

    #[test]
    fn test_system_path_flagged() {
        let svc = make_service_with_volumes("web", &["/etc/nginx:/etc/nginx:ro"]);
        let report = check_bind_portability(&svc, Path::new("/stacks/mystack"));
        assert!(report.issues.iter().any(|i| i.issue_type == IssueType::SystemPathReference));
    }

    #[test]
    fn test_home_dir_reference_flagged() {
        let svc = make_service_with_volumes("web", &["~/data:/app/data"]);
        let report = check_bind_portability(&svc, Path::new("/stacks/mystack"));
        assert!(report.issues.iter().any(|i| i.issue_type == IssueType::HomeDirReference));
    }

    #[test]
    fn test_migratable_to_appdata_flagged() {
        let svc = make_service_with_volumes("web", &["./data:/app/data"]);
        let report = check_bind_portability(&svc, Path::new("/stacks/mystack"));
        assert!(report.issues.iter().any(|i| i.issue_type == IssueType::MigratableToAppdata));
    }

    #[test]
    fn test_appdata_mount_clean() {
        let svc = make_service_with_volumes("web", &["./appdata/nginx:/app/data"]);
        let report = check_bind_portability(&svc, Path::new("/stacks/mystack"));
        // ./appdata/ is the canonical location — no portability issues
        assert!(report.issues.is_empty());
    }

    #[test]
    fn test_no_volumes_no_issues() {
        let svc = ComposeService::new("web", "nginx:latest");
        let report = check_bind_portability(&svc, Path::new("/stacks/mystack"));
        assert!(report.issues.is_empty());
    }

    #[test]
    fn test_rewrite_to_appdata_basic() {
        let result = rewrite_to_appdata("./data:/var/lib/data", "myservice");
        assert!(result.is_some());
        let spec = result.unwrap();
        assert!(spec.starts_with("./appdata/var/lib/data:"));
    }

    #[test]
    fn test_rewrite_to_appdata_with_options() {
        let result = rewrite_to_appdata("./data:/var/lib/data:ro", "myservice");
        assert!(result.is_some());
        let spec = result.unwrap();
        assert!(spec.ends_with(":ro"));
    }

    #[test]
    fn test_suggest_appdata_moves_filters_clean() {
        let services = vec![
            make_service_with_volumes("web", &["./appdata/nginx:/app"]),
            make_service_with_volumes("db", &["/etc/pgdata:/var/lib/postgresql/data"]),
        ];
        let reports = suggest_appdata_moves(&services, Path::new("/stacks/mystack"));
        // Only db has an issue
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].service_name, "db");
    }
}

