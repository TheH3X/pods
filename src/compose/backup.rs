use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

/// Backup directory name inside the stack root.
const BACKUP_DIR: &str = ".stacks-backups";

/// Maximum number of backup snapshots to keep per stack.
const MAX_BACKUPS: usize = 10;

/// Create a timestamped snapshot of a stack's compose files.
///
/// The snapshot is a copy of all compose YAML files into:
/// `<stack_root>/.stacks-backups/<timestamp>/`
pub fn create_snapshot(stack_path: &Path) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_dir = stack_path.join(BACKUP_DIR).join(&timestamp);

    fs::create_dir_all(&backup_dir).with_context(|| {
        format!(
            "Failed to create backup directory: {}",
            backup_dir.display()
        )
    })?;

    // Copy compose files
    copy_compose_files(stack_path, &backup_dir)?;

    log::info!(
        "Created backup snapshot: {} -> {}",
        stack_path.display(),
        backup_dir.display()
    );

    // Prune old snapshots
    let _ = prune_old_snapshots(stack_path, MAX_BACKUPS);

    Ok(backup_dir)
}

/// Copy all compose-related files from the stack to the backup directory.
fn copy_compose_files(src: &Path, dest: &Path) -> Result<()> {
    // Copy root-level compose files
    for name in &["docker-compose.yml", "compose.yml", ".env"] {
        let src_file = src.join(name);
        if src_file.exists() {
            let dest_file = dest.join(name);
            fs::copy(&src_file, &dest_file).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    src_file.display(),
                    dest_file.display()
                )
            })?;
        }
    }

    // Copy service subdirectory compose files (for nested layout)
    if let Ok(entries) = fs::read_dir(src) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && !path
                    .file_name()
                    .map_or(false, |n| n.to_string_lossy().starts_with('.'))
            {
                let compose_file = path.join("compose.yml");
                if compose_file.exists() {
                    let service_name = path.file_name().unwrap().to_string_lossy();
                    let dest_service_dir = dest.join(service_name.as_ref());
                    fs::create_dir_all(&dest_service_dir)?;
                    fs::copy(&compose_file, dest_service_dir.join("compose.yml"))?;
                }

                // Also copy docker-compose.yml in subdirs
                let docker_compose_file = path.join("docker-compose.yml");
                if docker_compose_file.exists() {
                    let service_name = path.file_name().unwrap().to_string_lossy();
                    let dest_service_dir = dest.join(service_name.as_ref());
                    fs::create_dir_all(&dest_service_dir)?;
                    fs::copy(
                        &docker_compose_file,
                        dest_service_dir.join("docker-compose.yml"),
                    )?;
                }
            }
        }
    }

    Ok(())
}

/// Prune old backup snapshots, keeping only the most recent `keep` snapshots.
pub fn prune_old_snapshots(stack_path: &Path, keep: usize) -> Result<()> {
    let backup_root = stack_path.join(BACKUP_DIR);
    if !backup_root.exists() {
        return Ok(());
    }

    let mut snapshots: Vec<PathBuf> = fs::read_dir(&backup_root)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();

    // Sort by name (timestamp-based names sort chronologically)
    snapshots.sort();

    if snapshots.len() > keep {
        let to_remove = snapshots.len() - keep;
        for snapshot in snapshots.into_iter().take(to_remove) {
            log::info!("Pruning old backup: {}", snapshot.display());
            fs::remove_dir_all(&snapshot)
                .with_context(|| format!("Failed to remove old backup: {}", snapshot.display()))?;
        }
    }

    Ok(())
}

/// List all backup snapshots for a stack, newest first.
pub fn list_snapshots(stack_path: &Path) -> Result<Vec<PathBuf>> {
    let backup_root = stack_path.join(BACKUP_DIR);
    if !backup_root.exists() {
        return Ok(Vec::new());
    }

    let mut snapshots: Vec<PathBuf> = fs::read_dir(&backup_root)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();

    snapshots.sort();
    snapshots.reverse(); // Newest first
    Ok(snapshots)
}

/// Restore a stack from a backup snapshot.
pub fn restore_snapshot(stack_path: &Path, snapshot_path: &Path) -> Result<()> {
    // First create a snapshot of the current state
    let _ = create_snapshot(stack_path);

    // Then copy backup files over current files
    copy_compose_files(snapshot_path, stack_path)?;

    log::info!(
        "Restored stack from snapshot: {} <- {}",
        stack_path.display(),
        snapshot_path.display()
    );

    Ok(())
}
