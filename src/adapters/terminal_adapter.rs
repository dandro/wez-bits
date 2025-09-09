use std::process::{Command, ExitStatus, Stdio};

use log::info;

use crate::domain::models::{AppError, Direction};
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
    fn open_pane(&self, direction: Direction, size: i32) -> Result<String, AppError> {
        info!("Get or open wezterm panel: {}", direction.to_string());
        let pane_size = size.to_string();
        let args = match direction {
            Direction::Right => {
                vec!["cli", "split-pane", "--horizontal", "--percent", &pane_size]
            }
            Direction::Down => vec!["cli", "split-pane", "--percent", &pane_size],
        };

        Command::new("wezterm")
            .args(args)
            .output()
            .map_err(|e| AppError::TerminalOperationError(e.to_string()))
            .and_then(|o| {
                String::from_utf8(o.stdout)
                    .map_err(|e| AppError::TerminalOperationError(e.to_string()))
                    .and_then(|id| {
                        let pane_id = id.trim();
                        if pane_id.is_empty() {
                            Err(AppError::TerminalOperationError(format!(
                                "There is no pane {direction}"
                            )))
                        } else {
                            Ok(pane_id.to_string())
                        }
                    })
            })
    }

    fn close_pane(&self, pane_id: &str) -> Result<(), AppError> {
        Command::new("wezterm")
            .args(["cli", "kill-pane", "--pane-id", pane_id])
            .output()
            .map(|_| ())
            .map_err(|e| AppError::TerminalOperationError(e.to_string()))
    }

    fn display_logs_in_pane(&self, pane_id: &str) -> Result<(), AppError> {
        info!("Displaying logs in pane with id {}", pane_id);
        let error_file = format!("{}/{}", self.dot_dir, self.error_filename);
        let output_file = format!("{}/{}", self.dot_dir, self.output_filename);
        let arg = format!("tail -f -n 20 {error_file} {output_file} | bat --paging=never -l log");

        let echo_cmd = Command::new("echo")
            .arg(arg)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| AppError::TerminalOperationError(e.to_string()))?;

        Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", pane_id, "--no-paste"])
            .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
            .stdout(Stdio::inherit())
            .spawn()
            .map(|_| ())
            .map_err(|e| AppError::TerminalOperationError(e.to_string()))
    }

    fn pipe_text_to_pane(
        &self,
        args: Vec<String>,
        pane_id: String,
    ) -> Result<ExitStatus, AppError> {
        let project_task = Command::new("echo")
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| AppError::TerminalOperationError(e.to_string()))?;

        Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", &pane_id, "--no-paste"])
            .stdin(Stdio::from(project_task.stdout.unwrap()))
            .stdout(Stdio::inherit())
            .spawn()
            .and_then(|c| c.wait_with_output())
            .map(|output| output.status)
            .map_err(|e| AppError::TerminalOperationError(e.to_string()))
    }
}
