use anyhow::Result;
use std::process::ExitStatus;

use crate::domain::models::{Direction, TaskClose};

/// Port for terminal operations
#[cfg_attr(test, mockall::automock)]
pub trait TerminalPort {
    /// Open a new pane in the terminal
    fn open_pane(&self, direction: Direction, size: i32) -> Result<String>;

    /// Pipe text to a pane
    fn pipe_text_to_pane(
        &self,
        args: Vec<String>,
        pane_id: &str,
        close: TaskClose,
    ) -> Result<ExitStatus>;
}
