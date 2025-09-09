use std::process::ExitStatus;

use crate::domain::models::{AppError, Direction};

/// Port for terminal operations
pub trait TerminalPort {
    /// Open a new pane in the terminal
    fn open_pane(&self, direction: Direction, size: i32) -> Result<String, AppError>;
    
    /// Close a pane
    fn close_pane(&self, pane_id: &str) -> Result<(), AppError>;
    
    /// Display logs in a pane
    fn display_logs_in_pane(&self, pane_id: &str) -> Result<(), AppError>;
    
    /// Pipe text to a pane
    fn pipe_text_to_pane(&self, args: Vec<String>, pane_id: String) -> Result<ExitStatus, AppError>;
}