use std::process::ExitStatus;
use anyhow::Result;

use crate::domain::models::Command;

/// Port for executing commands
#[cfg_attr(test, mockall::automock)]
pub trait TaskExecutionPort {
    /// Execute a command and return its exit status
    fn execute_command(&self, command: Command) -> Result<ExitStatus>;
}