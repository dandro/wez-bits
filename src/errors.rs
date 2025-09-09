use thiserror::Error;

/// Domain-specific errors in the application
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Task '{0}' not configured")]
    FeatureNotConfigured(String),

    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("File operation failed: {0}")]
    FileOperation(String),

    #[error("Terminal operation failed: {0}")]
    TerminalOperation(String),
}

/// File system related errors
#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("Failed to read file: {0}")]
    Read(String),

    #[error("Failed to write file: {0}")]
    Write(String),

    #[error("Failed to create directory: {0}")]
    CreateDirectory(String),

    #[error("Failed to create output file: {0}")]
    CreateOutputFile(String),
}

/// Configuration related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    Load(String),

    #[error("Failed to parse configuration: {0}")]
    Parse(String),

    #[error("Failed to create default configuration: {0}")]
    Create(String),
}

/// Terminal operation errors
#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("Failed to open pane: {0}")]
    OpenPane(String),

    #[error("Failed to close pane: {0}")]
    ClosePane(String),

    #[error("Failed to display logs: {0}")]
    DisplayLogs(String),

    #[error("Failed to pipe text to pane: {0}")]
    PipeText(String),
}

/// Task execution errors
#[derive(Error, Debug)]
pub enum TaskExecutionError {
    #[error("Failed to execute command: {0}")]
    Execute(String),
}

// From implementations for error conversions
impl From<FileSystemError> for DomainError {
    fn from(err: FileSystemError) -> Self {
        DomainError::FileOperation(err.to_string())
    }
}

impl From<ConfigError> for DomainError {
    fn from(err: ConfigError) -> Self {
        DomainError::Configuration(err.to_string())
    }
}

impl From<TerminalError> for DomainError {
    fn from(err: TerminalError) -> Self {
        DomainError::TerminalOperation(err.to_string())
    }
}

impl From<TaskExecutionError> for DomainError {
    fn from(err: TaskExecutionError) -> Self {
        DomainError::CommandExecution(err.to_string())
    }
}

impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        FileSystemError::Read(err.to_string())
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Parse(err.to_string())
    }
}