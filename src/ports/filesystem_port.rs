use anyhow::Result;

/// Port for file system operations
#[cfg_attr(test, mockall::automock)]
pub trait FileSystemPort {
    /// Create a directory
    fn create_directory(&self, path: &str) -> Result<()>;

    /// Write content to a file
    fn write_to_file(&self, path: &str, content: &str) -> Result<()>;

    /// Read content from a file
    fn read_from_file(&self, path: &str) -> Result<String>;
}
