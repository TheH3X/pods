use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use super::models::{Stack, StackLayout, ComposeService};
use super::yaml_io;

pub fn scan_root(root_path: &Path) -> Result<Vec<Stack>> {
    let mut stacks = Vec::new();
    if !root_path.exists() || !root_path.is_dir() {
        return Ok(stacks);
    }

    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Check if it's a valid stack
            if let Ok(stack) = scan_stack(&path) {
                stacks.push(stack);
            }
        }
    }
    
    stacks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(stacks)
}

pub fn scan_stack(stack_path: &Path) -> Result<Stack> {
    let name = stack_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let layout = detect_layout(stack_path);
    let mut services = Vec::new();
    let mut networks = Vec::new();

    match layout {
        StackLayout::Flat => {
            // Read docker-compose.yml
            let compose_file = stack_path.join("docker-compose.yml");
            let (mut s, mut n) = yaml_io::read_compose_file(&compose_file)?;
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
        StackLayout::Nested => {
            // Read each subdirectory's compose.yml
            for entry in fs::read_dir(stack_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let compose_file = path.join("compose.yml");
                    if compose_file.exists() {
                        let (mut s, mut n) = yaml_io::read_compose_file(&compose_file).unwrap_or_default();
                        for (svc_name, mut svc) in s.into_iter() {
                            svc.name = svc_name;
                            svc.folder_path = Some(path.clone());
                            services.push(svc);
                        }
                        for (net_name, mut net) in n.into_iter() {
                            net.name = net_name;
                            networks.push(net);
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

pub fn detect_layout(stack_path: &Path) -> StackLayout {
    if stack_path.join("docker-compose.yml").exists() || stack_path.join("compose.yml").exists() {
        StackLayout::Flat
    } else {
        StackLayout::Nested
    }
}
