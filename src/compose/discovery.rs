use super::models::{Stack, StackLayout};
use super::yaml_io;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Known compose file names, in priority order.
const COMPOSE_FILE_NAMES: &[&str] = &[
    "docker-compose.yml",
    "docker-compose.yaml",
    "compose.yml",
    "compose.yaml",
];

/// Scan a root directory for stacks (each subdirectory is potentially a stack).
pub fn scan_root(root_path: &Path) -> Result<Vec<Stack>> {
    let mut stacks = Vec::new();
    if !root_path.exists() || !root_path.is_dir() {
        return Ok(stacks);
    }

    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Skip hidden directories
            if path
                .file_name()
                .map_or(false, |n| n.to_string_lossy().starts_with('.'))
            {
                continue;
            }

            // Check if it's a valid stack
            if let Ok(stack) = scan_stack(&path) {
                stacks.push(stack);
            }
        }
    }

    stacks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(stacks)
}

/// Scan a single directory as a stack, detecting its layout and parsing compose files.
pub fn scan_stack(stack_path: &Path) -> Result<Stack> {
    let name = stack_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let layout = detect_layout(stack_path);
    let mut services = Vec::new();
    let mut networks = Vec::new();

    match layout {
        StackLayout::Flat => {
            // Find and read the compose file
            if let Some(compose_file) = find_compose_file(stack_path) {
                let (s, n) = yaml_io::read_compose_file(&compose_file)?;
                for (svc_name, mut svc) in s.into_iter() {
                    svc.name = svc_name;
                    svc.folder_path = None; // Flat layout has no per-service folder
                    services.push(svc);
                }
                for (net_name, mut net) in n.into_iter() {
                    net.name = net_name;
                    networks.push(net);
                }
            }
        }
        StackLayout::Nested => {
            // Read each subdirectory's compose file
            let mut entries: Vec<_> = fs::read_dir(stack_path)?.filter_map(|e| e.ok()).collect();
            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let path = entry.path();
                if path.is_dir() {
                    // Skip hidden directories
                    if path
                        .file_name()
                        .map_or(false, |n| n.to_string_lossy().starts_with('.'))
                    {
                        continue;
                    }

                    if let Some(compose_file) = find_compose_file(&path) {
                        let (s, n) = yaml_io::read_compose_file(&compose_file).unwrap_or_default();
                        for (svc_name, mut svc) in s.into_iter() {
                            svc.name = svc_name;
                            svc.folder_path = Some(path.clone());
                            services.push(svc);
                        }
                        for (net_name, mut net) in n.into_iter() {
                            net.name = net_name.clone();
                            // Deduplicate networks
                            if !networks
                                .iter()
                                .any(|n: &super::models::Network| n.name == net_name)
                            {
                                networks.push(net);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(Stack {
        name,
        root_path: stack_path.to_path_buf(),
        layout,
        services,
        networks,
    })
}

/// Scan a specific stack by name from a root directory.
pub fn focused_stack(root_path: &Path, stack_name: &str) -> Result<Stack> {
    let stack_path = root_path.join(stack_name);
    scan_stack(&stack_path)
}

/// Detect whether a stack uses FLAT or NESTED layout.
///
/// - FLAT: The stack directory itself contains a compose file
/// - NESTED: The stack directory contains subdirectories with compose files
pub fn detect_layout(stack_path: &Path) -> StackLayout {
    // If the root has a compose file, it's FLAT
    if find_compose_file(stack_path).is_some() {
        return StackLayout::Flat;
    }

    // Otherwise check if any subdirectory has a compose file → NESTED
    if let Ok(entries) = fs::read_dir(stack_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && find_compose_file(&path).is_some() {
                return StackLayout::Nested;
            }
        }
    }

    // Default to Nested (allows creating new nested stacks in empty dirs)
    StackLayout::Nested
}

/// Find a compose file in a directory, checking multiple naming variants.
pub fn find_compose_file(dir: &Path) -> Option<PathBuf> {
    for name in COMPOSE_FILE_NAMES {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Check if a directory looks like it could be a stack root
/// (has compose files directly or in subdirectories).
pub fn is_stack_directory(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    // Direct compose file
    if find_compose_file(path).is_some() {
        return true;
    }

    // Subdirectories with compose files
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let child = entry.path();
            if child.is_dir() && find_compose_file(&child).is_some() {
                return true;
            }
        }
    }

    false
}

/// Get a list of all compose files in a stack (for both layouts).
pub fn list_compose_files(stack_path: &Path, layout: &StackLayout) -> Vec<PathBuf> {
    let mut files = Vec::new();

    match layout {
        StackLayout::Flat => {
            if let Some(f) = find_compose_file(stack_path) {
                files.push(f);
            }
        }
        StackLayout::Nested => {
            if let Ok(entries) = fs::read_dir(stack_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(f) = find_compose_file(&path) {
                            files.push(f);
                        }
                    }
                }
            }
            files.sort();
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    #[test]
    fn test_detect_flat_layout() {
        let flat_path = fixtures_dir().join("flat_stack");
        if flat_path.exists() {
            assert_eq!(detect_layout(&flat_path), StackLayout::Flat);
        }
    }

    #[test]
    fn test_detect_nested_layout() {
        let nested_path = fixtures_dir().join("nested_stack");
        if nested_path.exists() {
            assert_eq!(detect_layout(&nested_path), StackLayout::Nested);
        }
    }

    #[test]
    fn test_scan_flat_stack() {
        let flat_path = fixtures_dir().join("flat_stack");
        if flat_path.exists() {
            let stack = scan_stack(&flat_path).unwrap();
            assert_eq!(stack.name, "flat_stack");
            assert_eq!(stack.layout, StackLayout::Flat);
            assert_eq!(stack.services.len(), 2); // nginx + php
            assert_eq!(stack.networks.len(), 2); // frontend + backend
        }
    }

    #[test]
    fn test_scan_nested_stack() {
        let nested_path = fixtures_dir().join("nested_stack");
        if nested_path.exists() {
            let stack = scan_stack(&nested_path).unwrap();
            assert_eq!(stack.name, "nested_stack");
            assert_eq!(stack.layout, StackLayout::Nested);
            assert!(stack.services.len() >= 2); // grafana + prometheus
        }
    }

    #[test]
    fn test_scan_root_multi_stack() {
        let root_path = fixtures_dir().join("docker_stacks_root");
        if root_path.exists() {
            let stacks = scan_root(&root_path).unwrap();
            assert!(stacks.len() >= 2); // monitoring + web
            assert!(stacks.iter().any(|s| s.name == "monitoring"));
            assert!(stacks.iter().any(|s| s.name == "web"));
        }
    }

    #[test]
    fn test_is_stack_directory() {
        let flat_path = fixtures_dir().join("flat_stack");
        if flat_path.exists() {
            assert!(is_stack_directory(&flat_path));
        }

        let nested_path = fixtures_dir().join("nested_stack");
        if nested_path.exists() {
            assert!(is_stack_directory(&nested_path));
        }
    }

    #[test]
    fn test_find_compose_file_variants() {
        let flat_path = fixtures_dir().join("flat_stack");
        if flat_path.exists() {
            let found = find_compose_file(&flat_path);
            assert!(found.is_some());
            let path = found.unwrap();
            assert!(
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("compose")
            );
        }
    }

    #[test]
    fn test_scan_nonexistent_root() {
        let stacks = scan_root(Path::new("/nonexistent/path")).unwrap();
        assert!(stacks.is_empty());
    }

    #[test]
    fn test_focused_stack() {
        let root = fixtures_dir().join("docker_stacks_root");
        if root.exists() {
            let stack = focused_stack(&root, "web");
            assert!(stack.is_ok());
            let stack = stack.unwrap();
            assert_eq!(stack.name, "web");
        }
    }

    /// Verifies that when both a root-level compose file AND service subdirectories exist,
    /// the layout is detected as FLAT (root compose file takes priority over subdirs).
    #[test]
    fn test_detect_mixed_layout() {
        let mixed_path = fixtures_dir().join("mixed_stack");
        if mixed_path.exists() {
            // mixed_stack has docker-compose.yml at root AND a web/ subdir with compose.yml
            // Current policy: root-level compose → FLAT takes priority
            assert_eq!(detect_layout(&mixed_path), StackLayout::Flat);
        }
    }

    /// Verifies that a mixed stack scanned as FLAT reads only the root compose file.
    #[test]
    fn test_scan_mixed_stack() {
        let mixed_path = fixtures_dir().join("mixed_stack");
        if mixed_path.exists() {
            let stack = scan_stack(&mixed_path).unwrap();
            assert_eq!(stack.name, "mixed_stack");
            assert_eq!(stack.layout, StackLayout::Flat);
            // Only the root compose file's service (proxy) should be read
            assert!(stack.services.iter().any(|s| s.name == "proxy"));
            // The web/ subdir service should NOT appear (flat layout only reads root file)
            assert!(!stack.services.iter().any(|s| s.name == "nginx"));
        }
    }
}
