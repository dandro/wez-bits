use std::process::{Command, Stdio};

use log::info;

use crate::domain::AppErr;

#[derive(Debug)]
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

pub fn open_pane(direction: Direction) -> Result<String, AppErr> {
    info!("Get or open wezterm panel: {}", direction.to_string());
    let args = match direction {
        Direction::Right | Direction::Left => {
            vec!["cli", "split-pane", "--horizaontal", "--percent", "30"]
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
    let echo_cmd = Command::new("echo")
        .arg("tail -f -n 20 .hx/output.log .hx/errors.log")
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
