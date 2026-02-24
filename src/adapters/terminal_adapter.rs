use std::process::{Command, ExitStatus, Stdio};

use anyhow::{Context, Result};
use log::info;

use crate::domain::models::{Direction, TaskClose, TerminalError};
use crate::ports::TerminalPort;

pub struct TerminalAdapter {}

impl TerminalAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl TerminalPort for TerminalAdapter {
    fn open_pane(&self, direction: Direction, size: i32) -> Result<String> {
        info!("Open wezterm panel: {}", direction.to_string());
        let pane_size = size.to_string();
        let args = match direction {
            Direction::Right => {
                vec!["cli", "split-pane", "--horizontal", "--percent", &pane_size]
            }
            Direction::Down => vec!["cli", "split-pane", "--percent", &pane_size],
        };

        let output = Command::new("wezterm")
            .args(args)
            .output()
            .with_context(|| {
                TerminalError::OpenPane("Failed to run wezterm command".to_string())
            })?;

        let stdout = String::from_utf8(output.stdout).with_context(|| {
            TerminalError::OpenPane("Failed to parse wezterm output".to_string())
        })?;

        let pane_id = stdout.trim();
        if pane_id.is_empty() {
            Err(TerminalError::OpenPane(format!("There is no pane {direction}")).into())
        } else {
            Ok(pane_id.to_string())
        }
    }

    fn close_pane(&self, pane_id: &str) -> Result<()> {
        Command::new("wezterm")
            .args(["cli", "kill-pane", "--pane-id", pane_id])
            .output()
            .with_context(|| {
                TerminalError::ClosePane(format!("Failed to close pane {}", pane_id))
            })?;
        Ok(())
    }

    fn pipe_text_to_pane(
        &self,
        args: Vec<String>,
        pane_id: &str,
        close: TaskClose,
    ) -> Result<ExitStatus> {
        let base_cmd = args.join(" ");
        let full_cmd = match close {
            TaskClose::Never => base_cmd,
            TaskClose::OnSuccess => {
                format!("{base_cmd}; if ($env.LAST_EXIT_CODE == 0) {{ wezterm cli kill-pane }}")
            }
            TaskClose::Always => format!("{base_cmd}; wezterm cli kill-pane"),
        };

        info!("Executing task {full_cmd}");

        let project_task = Command::new("echo")
            .arg(&full_cmd)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| {
                TerminalError::PipeText(format!(
                    "Failed to create echo command for pane {}",
                    pane_id
                ))
            })?;

        let stdout = project_task
            .stdout
            .expect("Could not get project task STDOUT");

        let output = Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", pane_id, "--no-paste"])
            .stdin(Stdio::from(stdout))
            .spawn()
            .and_then(|c| c.wait_with_output())
            .with_context(|| {
                TerminalError::PipeText(format!("Failed to pipe text to pane {}", pane_id))
            })?;

        let exit_code = output.status;

        info!("Command exited with code {exit_code}");

        Ok(exit_code)
    }
}
