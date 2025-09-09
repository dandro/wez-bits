use crate::domain::models::{AppError, OutputType};

/// Port for file system operations
pub trait FileSystemPort {
    /// Create a directory
    fn create_directory(&self, path: &str) -> Result<(), AppError>;
    
    /// Write content to a file
    fn write_to_file(&self, path: &str, content: &str) -> Result<(), AppError>;
    
    /// Read content from a file
    fn read_from_file(&self, path: &str) -> Result<String, AppError>;
    
    /// Create an output file for task execution
    fn get_output_file(&self, output_type: OutputType) -> Result<std::fs::File, AppError>;
}