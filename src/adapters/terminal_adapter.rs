use std::process::{Command, ExitStatus, Stdio};

use anyhow::{Context, Result};
use log::info;

use crate::domain::models::Direction;
use crate::errors::TerminalError;
use crate::ports::TerminalPort;

pub struct TerminalAdapter {
    dot_dir: String,
    output_filename: String,
    error_filename: String,
}

impl TerminalAdapter {
    pub fn new(dot_dir: String, output_filename: String, error_filename: String) -> Self {
        Self {
            dot_dir,
            output_filename,
            error_filename,
        }
    }
}

impl TerminalPort for TerminalAdapter {
    fn open_pane(&self, direction: Direction, size: i32) -> Result<String> {
        info!("Get or open wezterm panel: {}", direction.to_string());
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
            .with_context(|| TerminalError::OpenPane("Failed to run wezterm command".to_string()))?;
            
        let stdout = String::from_utf8(output.stdout)
            .with_context(|| TerminalError::OpenPane("Failed to parse wezterm output".to_string()))?;
            
        let pane_id = stdout.trim();
        if pane_id.is_empty() {
            Err(TerminalError::OpenPane(format!(
                "There is no pane {direction}"
            )).into())
        } else {
            Ok(pane_id.to_string())
        }
    }

    fn close_pane(&self, pane_id: &str) -> Result<()> {
        Command::new("wezterm")
            .args(["cli", "kill-pane", "--pane-id", pane_id])
            .output()
            .with_context(|| TerminalError::ClosePane(format!("Failed to close pane {}", pane_id)))?;
        Ok(())
    }

    fn display_logs_in_pane(&self, pane_id: &str) -> Result<()> {
        info!("Displaying logs in pane with id {}", pane_id);
        let error_file = format!("{}/{}", self.dot_dir, self.error_filename);
        let output_file = format!("{}/{}", self.dot_dir, self.output_filename);
        let arg = format!("tail -f -n 20 {error_file} {output_file} | bat --paging=never -l log");

        let echo_cmd = Command::new("echo")
            .arg(arg)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| TerminalError::DisplayLogs(format!("Failed to create echo command for pane {}", pane_id)))?;

        Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", pane_id, "--no-paste"])
            .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
            .stdout(Stdio::inherit())
            .spawn()
            .with_context(|| TerminalError::DisplayLogs(format!("Failed to send text to pane {}", pane_id)))?;
            
        Ok(())
    }

    fn pipe_text_to_pane(&self, args: Vec<String>, pane_id: String) -> Result<ExitStatus> {
        let project_task = Command::new("echo")
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| TerminalError::PipeText(format!("Failed to create echo command for pane {}", pane_id)))?;

        let output = Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", &pane_id, "--no-paste"])
            .stdin(Stdio::from(project_task.stdout.unwrap()))
            .stdout(Stdio::inherit())
            .spawn()
            .and_then(|c| c.wait_with_output())
            .with_context(|| TerminalError::PipeText(format!("Failed to pipe text to pane {}", pane_id)))?;
            
        Ok(output.status)
    }
}
