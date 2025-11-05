pub mod config_port;
pub mod filesystem_port;
pub mod terminal_port;

// Re-export ports
pub use config_port::ConfigPort;
pub use filesystem_port::FileSystemPort;
pub use terminal_port::TerminalPort;

// Re-export mock implementations for testing
#[cfg(test)]
pub mod mock {
    pub use super::config_port::MockConfigPort;
    pub use super::filesystem_port::MockFileSystemPort;
    pub use super::terminal_port::MockTerminalPort;
}
