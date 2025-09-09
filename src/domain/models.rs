use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    ConfigurationError(String),
    FeatureNotConfigured(String),
    CommandExecutionError(String),
    FileOperationError(String),
    TerminalOperationError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

/// Represents a command with program name and arguments
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

/// Settings for task execution
#[derive(Debug, Clone)]
pub struct TaskSettings {
    pub interactive: bool,
}

/// A task to be executed
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

/// Collection of commands mapped by name
pub type TaskConfig = HashMap<String, Command>;

/// Direction for splitting panes
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

/// Types of files for task output
#[derive(Debug)]
pub enum OutputType {
    Stdout,
    Stderr,
}
