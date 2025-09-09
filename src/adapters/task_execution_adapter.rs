use std::process::{Command, ExitStatus};

use crate::domain::models::{AppError, Command as AppCommand, OutputType};
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
    fn execute_command(&self, command: AppCommand) -> Result<ExitStatus, AppError> {
        let output = self.file_system.get_output_file(OutputType::Stdout)?;
        let error = self.file_system.get_output_file(OutputType::Stderr)?;

        Command::new(command.program)
            .args(command.args)
            .stderr(error)
            .stdout(output)
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| AppError::CommandExecutionError(e.to_string()))
    }
}