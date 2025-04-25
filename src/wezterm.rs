use std::process::{Command, ExitStatus, Stdio};

use log::info;

use crate::{
    constants::{DOTDIR, ERROR_FILENAME, OUTPUT_FILENAME},
    domain::AppErr,
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match &self {
            Direction::Right => String::from("right"),
            Direction::Left => String::from("left"),
            Direction::Up => String::from("up"),
            Direction::Down => String::from("down"),
        }
    }
}

pub fn open_pane(direction: Direction, size: i32) -> Result<String, AppErr> {
    info!("Get or open wezterm panel: {}", direction.to_string());
    let pane_size = size.to_string();
    let args = match direction {
        Direction::Right | Direction::Left => {
            vec!["cli", "split-pane", "--horizontal", "--percent", &pane_size]
        }
        Direction::Up | Direction::Down => vec!["cli", "split-pane", "--percent", "15"],
    };
    Command::new("wezterm")
        .args(args)
        .output()
        .map_err(|e| AppErr::CommandFailed(e.to_string()))
        .and_then(|o| {
            String::from_utf8(o.stdout)
                .map_err(|e| AppErr::CommandFailed(e.to_string()))
                .and_then(|id| {
                    let pane_id = id.trim();
                    if pane_id.is_empty() {
                        Err(AppErr::CommandFailed(format!(
                            "There is not pane {:?}",
                            direction
                        )))
                    } else {
                        Ok(pane_id.to_string())
                    }
                })
        })
}

pub fn display_logs_in_pane(pane_id: &str) -> Result<(), AppErr> {
    info!("Displaying logs in pane with id {}", pane_id);
    let error_file = format!("{}/{}", DOTDIR, ERROR_FILENAME);
    let output_file = format!("{}/{}", DOTDIR, OUTPUT_FILENAME);
    let arg = format!(
        "tail -f -n 20 {} {} | bat --paging=never -l log",
        error_file, output_file
    );
    let echo_cmd = Command::new("echo")
        .arg(arg)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| AppErr::CommandFailed(e.to_string()))?;

    Command::new("wezterm")
        .args(&["cli", "send-text", "--pane-id", pane_id, "--no-paste"])
        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
        .stdout(Stdio::inherit())
        .spawn()
        .map(|_| ())
        .map_err(|e| AppErr::CommandFailed(e.to_string()))
}

pub fn close_pane(pane_id: &str) -> Result<(), AppErr> {
    Command::new("wezterm")
        .args(&["cli", "kill-pane", "--pane-id", pane_id])
        .output()
        .map(|_| ())
        .map_err(|e| AppErr::CommandFailed(e.to_string()))
}

pub fn pipe_stdout_to_pane(args: Vec<String>, pane_id: String) -> Result<ExitStatus, AppErr> {
    let project_task = Command::new("echo")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| AppErr::CommandFailed(e.to_string()))?;

    Command::new("wezterm")
        .args(&["cli", "send-text", "--pane-id", &pane_id, "--no-paste"])
        .stdin(Stdio::from(project_task.stdout.unwrap()))
        .stdout(Stdio::inherit())
        .spawn()
        .and_then(|c| c.wait_with_output())
        .map(|output| output.status)
        .map_err(|e| AppErr::CommandFailed(e.to_string()))
}
