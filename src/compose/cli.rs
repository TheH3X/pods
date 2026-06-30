use anyhow::{Context, Result};
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

/// Result of running a compose command.
#[derive(Debug, Clone)]
pub struct ComposeResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Which compose subcommand to run.
#[derive(Debug, Clone)]
pub enum ComposeAction {
    Up { detach: bool, build: bool },
    Down { remove_volumes: bool },
    Pull,
    Restart,
    Stop,
    Start,
    Logs { follow: bool, tail: Option<u32> },
    Ps,
    Config,
}

impl ComposeAction {
    fn to_args(&self) -> Vec<String> {
        match self {
            ComposeAction::Up { detach, build } => {
                let mut args = vec!["up".to_string()];
                if *detach {
                    args.push("-d".to_string());
                }
                if *build {
                    args.push("--build".to_string());
                }
                args
            }
            ComposeAction::Down { remove_volumes } => {
                let mut args = vec!["down".to_string()];
                if *remove_volumes {
                    args.push("-v".to_string());
                }
                args
            }
            ComposeAction::Pull => vec!["pull".to_string()],
            ComposeAction::Restart => vec!["restart".to_string()],
            ComposeAction::Stop => vec!["stop".to_string()],
            ComposeAction::Start => vec!["start".to_string()],
            ComposeAction::Logs { follow, tail } => {
                let mut args = vec!["logs".to_string()];
                if *follow {
                    args.push("-f".to_string());
                }
                if let Some(n) = tail {
                    args.push("--tail".to_string());
                    args.push(n.to_string());
                }
                args
            }
            ComposeAction::Ps => vec!["ps".to_string()],
            ComposeAction::Config => vec!["config".to_string()],
        }
    }
}

/// Find the `docker` binary.
fn find_docker_binary() -> String {
    // Prefer DOCKER_HOST env var's associated binary
    std::env::var("DOCKER_COMPOSE_BIN").unwrap_or_else(|_| "docker".to_string())
}

/// Run a `docker compose` command in the given stack directory.
///
/// This shells out to the `docker compose` CLI, preserving full compatibility
/// with upstream Docker Compose behavior.
pub async fn run_compose_command(
    stack_path: &Path,
    action: ComposeAction,
    compose_file: Option<&str>,
) -> Result<ComposeResult> {
    let docker = find_docker_binary();
    let mut cmd = Command::new(&docker);

    // Base args: docker compose -f <file> <action>
    cmd.arg("compose");

    if let Some(file) = compose_file {
        cmd.arg("-f").arg(file);
    }

    cmd.args(action.to_args());
    cmd.current_dir(stack_path);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    log::info!(
        "Running: {} compose {} in {}",
        docker,
        action.to_args().join(" "),
        stack_path.display()
    );

    let output = cmd.output().await.with_context(|| {
        format!(
            "Failed to execute docker compose in {}",
            stack_path.display()
        )
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    if !output.status.success() {
        log::warn!("docker compose exited with code {}: {}", exit_code, stderr);
    }

    Ok(ComposeResult {
        success: output.status.success(),
        stdout,
        stderr,
        exit_code,
    })
}

/// Run `docker compose up -d` for a stack.
pub async fn compose_up(stack_path: &Path) -> Result<ComposeResult> {
    run_compose_command(
        stack_path,
        ComposeAction::Up {
            detach: true,
            build: false,
        },
        None,
    )
    .await
}

/// Run `docker compose down` for a stack.
pub async fn compose_down(stack_path: &Path) -> Result<ComposeResult> {
    run_compose_command(
        stack_path,
        ComposeAction::Down {
            remove_volumes: false,
        },
        None,
    )
    .await
}

/// Run `docker compose pull` for a stack.
pub async fn compose_pull(stack_path: &Path) -> Result<ComposeResult> {
    run_compose_command(stack_path, ComposeAction::Pull, None).await
}

/// Run `docker compose ps` and return output.
pub async fn compose_ps(stack_path: &Path) -> Result<ComposeResult> {
    run_compose_command(stack_path, ComposeAction::Ps, None).await
}

/// Run `docker compose config` to validate/render the compose file.
pub async fn compose_config(stack_path: &Path) -> Result<ComposeResult> {
    run_compose_command(stack_path, ComposeAction::Config, None).await
}
