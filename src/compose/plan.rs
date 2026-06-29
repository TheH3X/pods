use indexmap::IndexMap;
use serde_json::Value;
use std::collections::HashMap;

use super::models::{ComposeService, Network, Stack, StackLayout};
use super::yaml_io;

/// Represents an in-memory editing session for a stack.
/// Tracks changes against the on-disk state and produces diffs.
#[derive(Debug, Clone)]
pub struct PlanState {
    /// The original stack state (loaded from disk)
    pub original: StackSnapshot,
    /// The working copy (reflects current edits)
    pub working: StackSnapshot,
}

/// A snapshot of a stack's compose state at a point in time.
#[derive(Debug, Clone)]
pub struct StackSnapshot {
    pub services: IndexMap<String, ComposeService>,
    pub networks: IndexMap<String, Network>,
}

/// A single edit operation on a service.
#[derive(Debug, Clone)]
pub enum ServiceEdit {
    Add(String, ComposeService),
    Remove(String),
    Modify(String, ComposeService),
    Rename(String, String),    // old_name, new_name
    Duplicate(String, String), // source_name, new_name
}

/// Summary of changes between original and working state.
#[derive(Debug, Clone)]
pub struct ChangeSummary {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
    pub renamed: Vec<(String, String)>,
    pub networks_changed: bool,
}

impl PlanState {
    /// Create a new PlanState from a stack's current on-disk state.
    pub fn from_stack(stack: &Stack) -> Self {
        let mut services = IndexMap::new();
        for svc in &stack.services {
            services.insert(svc.name.clone(), svc.clone());
        }

        let mut networks = IndexMap::new();
        for net in &stack.networks {
            networks.insert(net.name.clone(), net.clone());
        }

        let snapshot = StackSnapshot { services, networks };

        PlanState {
            original: snapshot.clone(),
            working: snapshot,
        }
    }

    /// Apply an edit operation to the working copy.
    pub fn apply_edit(&mut self, edit: ServiceEdit) {
        match edit {
            ServiceEdit::Add(name, service) => {
                self.working.services.insert(name, service);
            }
            ServiceEdit::Remove(name) => {
                self.working.services.shift_remove(&name);
            }
            ServiceEdit::Modify(name, service) => {
                self.working.services.insert(name, service);
            }
            ServiceEdit::Rename(old_name, new_name) => {
                if let Some(mut svc) = self.working.services.shift_remove(&old_name) {
                    svc.name = new_name.clone();
                    self.working.services.insert(new_name, svc);
                }
            }
            ServiceEdit::Duplicate(source_name, new_name) => {
                if let Some(svc) = self.working.services.get(&source_name) {
                    let mut new_svc = svc.clone();
                    new_svc.name = new_name.clone();
                    new_svc.container_name = None; // Clear container name to avoid conflict
                    self.working.services.insert(new_name, new_svc);
                }
            }
        }
    }

    /// Add a network to the working copy.
    pub fn add_network(&mut self, name: String, network: Network) {
        self.working.networks.insert(name, network);
    }

    /// Remove a network from the working copy.
    pub fn remove_network(&mut self, name: &str) {
        self.working.networks.shift_remove(name);
    }

    /// Check if there are any unsaved changes.
    pub fn is_dirty(&self) -> bool {
        let summary = self.change_summary();
        !summary.added.is_empty()
            || !summary.removed.is_empty()
            || !summary.modified.is_empty()
            || !summary.renamed.is_empty()
            || summary.networks_changed
    }

    /// Check if a specific service has been modified.
    pub fn is_service_dirty(&self, name: &str) -> bool {
        match (
            self.original.services.get(name),
            self.working.services.get(name),
        ) {
            (Some(orig), Some(work)) => {
                // Compare serialized forms for deep equality
                let orig_yaml = serde_yaml::to_string(orig).unwrap_or_default();
                let work_yaml = serde_yaml::to_string(work).unwrap_or_default();
                orig_yaml != work_yaml
            }
            (None, Some(_)) => true, // Added
            (Some(_), None) => true, // Removed
            (None, None) => false,
        }
    }

    /// Produce a summary of all changes.
    pub fn change_summary(&self) -> ChangeSummary {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        // Find added and modified services
        for (name, work_svc) in &self.working.services {
            match self.original.services.get(name) {
                Some(orig_svc) => {
                    let orig_yaml = serde_yaml::to_string(orig_svc).unwrap_or_default();
                    let work_yaml = serde_yaml::to_string(work_svc).unwrap_or_default();
                    if orig_yaml != work_yaml {
                        modified.push(name.clone());
                    }
                }
                None => {
                    added.push(name.clone());
                }
            }
        }

        // Find removed services
        for name in self.original.services.keys() {
            if !self.working.services.contains_key(name) {
                removed.push(name.clone());
            }
        }

        // Check if networks changed
        let orig_nets_yaml = serde_yaml::to_string(&self.original.networks).unwrap_or_default();
        let work_nets_yaml = serde_yaml::to_string(&self.working.networks).unwrap_or_default();
        let networks_changed = orig_nets_yaml != work_nets_yaml;

        ChangeSummary {
            added,
            removed,
            modified,
            renamed: Vec::new(), // Renames are tracked as remove+add in the diff
            networks_changed,
        }
    }

    /// Generate a unified diff string comparing original vs working state.
    pub fn generate_diff(&self) -> String {
        let mut diff = String::new();
        let summary = self.change_summary();

        // Header
        diff.push_str(&format!(
            "# Plan Summary: {} added, {} removed, {} modified\n\n",
            summary.added.len(),
            summary.removed.len(),
            summary.modified.len()
        ));

        // Per-service diffs
        for name in &summary.added {
            if let Some(svc) = self.working.services.get(name) {
                let yaml = serde_yaml::to_string(svc).unwrap_or_default();
                diff.push_str(&format!("--- /dev/null\n+++ services/{name}\n"));
                for line in yaml.lines() {
                    diff.push_str(&format!("+{line}\n"));
                }
                diff.push('\n');
            }
        }

        for name in &summary.removed {
            if let Some(svc) = self.original.services.get(name) {
                let yaml = serde_yaml::to_string(svc).unwrap_or_default();
                diff.push_str(&format!("--- services/{name}\n+++ /dev/null\n"));
                for line in yaml.lines() {
                    diff.push_str(&format!("-{line}\n"));
                }
                diff.push('\n');
            }
        }

        for name in &summary.modified {
            if let (Some(orig), Some(work)) = (
                self.original.services.get(name),
                self.working.services.get(name),
            ) {
                let orig_yaml = serde_yaml::to_string(orig).unwrap_or_default();
                let work_yaml = serde_yaml::to_string(work).unwrap_or_default();
                diff.push_str(&format!(
                    "--- services/{name} (original)\n+++ services/{name} (modified)\n"
                ));
                // Simple line-by-line diff
                let orig_lines: Vec<&str> = orig_yaml.lines().collect();
                let work_lines: Vec<&str> = work_yaml.lines().collect();
                for line in &orig_lines {
                    if !work_lines.contains(line) {
                        diff.push_str(&format!("-{line}\n"));
                    }
                }
                for line in &work_lines {
                    if !orig_lines.contains(line) {
                        diff.push_str(&format!("+{line}\n"));
                    }
                }
                // Unchanged lines
                for line in &work_lines {
                    if orig_lines.contains(line) {
                        diff.push_str(&format!(" {line}\n"));
                    }
                }
                diff.push('\n');
            }
        }

        if summary.networks_changed {
            let orig_nets = serde_yaml::to_string(&self.original.networks).unwrap_or_default();
            let work_nets = serde_yaml::to_string(&self.working.networks).unwrap_or_default();
            diff.push_str("--- networks (original)\n+++ networks (modified)\n");
            for line in orig_nets.lines() {
                diff.push_str(&format!("-{line}\n"));
            }
            for line in work_nets.lines() {
                diff.push_str(&format!("+{line}\n"));
            }
        }

        diff
    }

    /// Reset working copy to original state (discard all edits).
    pub fn reset(&mut self) {
        self.working = self.original.clone();
    }

    /// Commit working state as new original (after successful deploy).
    pub fn commit(&mut self) {
        self.original = self.working.clone();
    }

    /// Get a service from the working copy.
    pub fn get_service(&self, name: &str) -> Option<&ComposeService> {
        self.working.services.get(name)
    }

    /// Get a mutable service from the working copy.
    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut ComposeService> {
        self.working.services.get_mut(name)
    }
}
