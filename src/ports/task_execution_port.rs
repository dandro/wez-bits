use std::process::ExitStatus;

use crate::domain::models::{AppError, Command};

/// Port for executing commands
pub trait TaskExecutionPort {
    /// Execute a command and return its exit status
    fn execute_command(&self, command: Command) -> Result<ExitStatus, AppError>;
}