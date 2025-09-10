use std::process::ExitStatus;
use anyhow::Result;

use crate::domain::models::Direction;

/// Port for terminal operations
#[cfg_attr(test, mockall::automock)]
pub trait TerminalPort {
    /// Open a new pane in the terminal
    fn open_pane(&self, direction: Direction, size: i32) -> Result<String>;
    
    /// Close a pane
    fn close_pane(&self, pane_id: &str) -> Result<()>;
    
    /// Display logs in a pane
    fn display_logs_in_pane(&self, pane_id: &str) -> Result<()>;
    
    /// Pipe text to a pane
    fn pipe_text_to_pane(&self, args: Vec<String>, pane_id: String) -> Result<ExitStatus>;
}