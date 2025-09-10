use anyhow::{Context, Result};
use std::process::{Command, ExitStatus};

use crate::domain::models::{Command as AppCommand, OutputType, TaskExecutionError};
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
        let output = self
            .file_system
            .get_output_file(OutputType::Stdout)
            .with_context(|| {
                TaskExecutionError::Execute(format!(
                    "Failed to create stdout file for command: {}",
                    command.program
                ))
            })?;

        let error = self
            .file_system
            .get_output_file(OutputType::Stderr)
            .with_context(|| {
                TaskExecutionError::Execute(format!(
                    "Failed to create stderr file for command: {}",
                    command.program
                ))
            })?;

        Command::new(&command.program)
            .args(&command.args)
            .stderr(error)
            .stdout(output)
            .spawn()
            .and_then(|mut child| child.wait())
            .with_context(|| {
                TaskExecutionError::Execute(format!(
                    "Failed to execute command: {} {:?}",
                    command.program, command.args
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::mock::MockFileSystemPort;
    use std::fs::File;
    use tempfile::tempdir;

    // Helper function to create a mock file system
    fn create_mock_fs() -> MockFileSystemPort {
        let mut mock_fs = MockFileSystemPort::new();

        // Setup mock for getting stdout file
        mock_fs
            .expect_get_output_file()
            .with(mockall::predicate::eq(OutputType::Stdout))
            .times(1)
            .returning(|_| {
                let dir = tempdir().unwrap();
                let file_path = dir.path().join("stdout.log");
                Ok(File::create(file_path).unwrap())
            });

        // Setup mock for getting stderr file
        mock_fs
            .expect_get_output_file()
            .with(mockall::predicate::eq(OutputType::Stderr))
            .times(1)
            .returning(|_| {
                let dir = tempdir().unwrap();
                let file_path = dir.path().join("stderr.log");
                Ok(File::create(file_path).unwrap())
            });

        mock_fs
    }

    #[test]
    fn test_adapter_initialization() {
        let mock_fs = MockFileSystemPort::new();
        // Verify adapter can be created
        TaskExecutionAdapter::new(mock_fs);
        assert!(true);
    }

    // Test executing a simple command like 'echo'
    #[test]
    fn test_execute_simple_command() {
        let mock_fs = create_mock_fs();
        let adapter = TaskExecutionAdapter::new(mock_fs);

        // Create a simple command (echo) that should succeed
        let command = AppCommand {
            program: "echo".to_string(),
            args: vec!["test".to_string()],
        };

        // Execute the command
        let result = adapter.execute_command(command);

        // Check that execution was successful
        assert!(result.is_ok());

        // Check exit status
        let status = result.unwrap();
        assert!(status.success());
    }

    #[test]
    fn test_execute_nonexistent_command() {
        let mock_fs = create_mock_fs();
        let adapter = TaskExecutionAdapter::new(mock_fs);

        // Create a command that doesn't exist
        let command = AppCommand {
            program: "nonexistent_command_12345".to_string(),
            args: vec![],
        };

        // Execute the command
        let result = adapter.execute_command(command);

        // Check that execution failed
        assert!(result.is_err());
    }

    #[test]
    fn test_file_system_error_handling() {
        let mut mock_fs = MockFileSystemPort::new();

        // Setup mock to return error for stdout
        mock_fs
            .expect_get_output_file()
            .with(mockall::predicate::eq(OutputType::Stdout))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Failed to create stdout file")));

        let adapter = TaskExecutionAdapter::new(mock_fs);

        // Create a simple command
        let command = AppCommand {
            program: "echo".to_string(),
            args: vec!["test".to_string()],
        };

        // Execute the command
        let result = adapter.execute_command(command);

        // Check that execution failed due to file system error
        assert!(result.is_err());

        // Check error message contains the expected text
        let err = result.unwrap_err();
        let err_string = err.to_string();
        assert!(err_string.contains("stdout"));
    }
}
