use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

/// Domain-specific errors in the application
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Task '{0}' not configured")]
    FeatureNotConfigured(String),

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

    #[error("Failed to pipe text to pane: {0}")]
    PipeText(String),
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

impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        FileSystemError::Read(err.to_string())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse(err.to_string())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TaskSettings {
    pub close: TaskClose,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub command: Command,
    pub settings: TaskSettings,
}

impl Task {
    pub fn new(command: Command, settings: TaskSettings) -> Self {
        Self { command, settings }
    }
}

pub type TaskConfig = HashMap<String, Command>;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Direction::Right => write!(f, "right"),
            Direction::Down => write!(f, "down"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaskClose {
    Always,
    OnSuccess,
    Never,
}
