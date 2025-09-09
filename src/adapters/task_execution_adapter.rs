use std::process::{Command, ExitStatus};
use anyhow::{Context, Result};

use crate::domain::models::{Command as AppCommand, OutputType};
use crate::errors::TaskExecutionError;
use crate::ports::{FileSystemPort, TaskExecutionPort};

pub struct TaskExecutionAdapter<F: FileSystemPort> {
    file_system: F,
}

impl<F: FileSystemPort> TaskExecutionAdapter<F> {
    pub fn new(file_system: F) -> Self {
        Self { file_system }
    }
}

impl<F: FileSystemPort> TaskExecutionPort for TaskExecutionAdapter<F> {
    fn execute_command(&self, command: AppCommand) -> Result<ExitStatus> {
        let output = self.file_system.get_output_file(OutputType::Stdout)
            .with_context(|| TaskExecutionError::Execute(format!("Failed to create stdout file for command: {}", command.program)))?;
            
        let error = self.file_system.get_output_file(OutputType::Stderr)
            .with_context(|| TaskExecutionError::Execute(format!("Failed to create stderr file for command: {}", command.program)))?;

        Command::new(&command.program)
            .args(&command.args)
            .stderr(error)
            .stdout(output)
            .spawn()
            .and_then(|mut child| child.wait())
            .with_context(|| TaskExecutionError::Execute(format!("Failed to execute command: {} {:?}", command.program, command.args)))
    }
}