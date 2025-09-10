use std::fs::{create_dir, File};
use std::io::Write;

use anyhow::{Context, Result};
use log::info;

use crate::domain::models::{FileSystemError, OutputType};
use crate::ports::FileSystemPort;

pub struct FileAdapter {
    pub dot_dir: String,
    pub output_filename: String,
    pub error_filename: String,
}

impl FileAdapter {
    pub fn new(dot_dir: String, output_filename: String, error_filename: String) -> Self {
        Self {
            dot_dir,
            output_filename,
            error_filename,
        }
    }
}

impl FileSystemPort for FileAdapter {
    fn create_directory(&self, path: &str) -> Result<()> {
        info!("Creating directory: {}", path);
        create_dir(path).with_context(|| FileSystemError::CreateDirectory(path.to_string()))?;
        Ok(())
    }

    fn write_to_file(&self, path: &str, content: &str) -> Result<()> {
        info!("Writing to file: {}", path);
        let mut file =
            File::create(path).with_context(|| FileSystemError::Write(path.to_string()))?;

        file.write_all(content.as_bytes())
            .with_context(|| FileSystemError::Write(path.to_string()))?;

        Ok(())
    }

    fn read_from_file(&self, path: &str) -> Result<String> {
        info!("Reading from file: {}", path);
        std::fs::read_to_string(path).with_context(|| FileSystemError::Read(path.to_string()))
    }

    fn get_output_file(&self, output_type: OutputType) -> Result<File> {
        let name = match output_type {
            OutputType::Stdout => &self.output_filename,
            OutputType::Stderr => &self.error_filename,
        };
        let filename = format!("{}/{}", self.dot_dir, name);
        info!("Creating output file: {}", filename);

        File::create(&filename).with_context(|| FileSystemError::CreateOutputFile(filename.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_create_directory() {
        let temp_dir = tempdir().unwrap();
        let new_dir_path = temp_dir.path().join("test_dir");
        let new_dir_str = new_dir_path.to_str().unwrap();

        let adapter = FileAdapter::new(
            ".wez".to_string(),
            "output.log".to_string(),
            "error.log".to_string(),
        );

        // Create directory
        let result = adapter.create_directory(new_dir_str);
        assert!(result.is_ok());

        // Check that the directory exists
        assert!(new_dir_path.exists());
        assert!(new_dir_path.is_dir());
    }

    #[test]
    fn test_write_to_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        let file_str = file_path.to_str().unwrap();
        let content = "Hello, world!";

        let adapter = FileAdapter::new(
            ".wez".to_string(),
            "output.log".to_string(),
            "error.log".to_string(),
        );

        // Write to file
        let result = adapter.write_to_file(file_str, content);
        assert!(result.is_ok());

        // Read back and check content
        let mut file = File::open(file_path).unwrap();
        let mut read_content = String::new();
        file.read_to_string(&mut read_content).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_read_from_file() {
        let content = "Hello, world!";
        let mut temp_file = NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut temp_file, content.as_bytes()).unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        let adapter = FileAdapter::new(
            ".wez".to_string(),
            "output.log".to_string(),
            "error.log".to_string(),
        );

        // Read from file
        let result = adapter.read_from_file(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_get_output_file() {
        let temp_dir = tempdir().unwrap();
        let dot_dir = temp_dir.path().to_str().unwrap().to_string();

        // Create the directory structure
        std::fs::create_dir_all(&dot_dir).unwrap();

        let adapter = FileAdapter::new(
            dot_dir.clone(),
            "output.log".to_string(),
            "error.log".to_string(),
        );

        // Get stdout file
        let stdout_result = adapter.get_output_file(OutputType::Stdout);
        assert!(stdout_result.is_ok());

        // Check that the file exists
        let stdout_path = format!("{}/output.log", dot_dir);
        assert!(std::path::Path::new(&stdout_path).exists());

        // Get stderr file
        let stderr_result = adapter.get_output_file(OutputType::Stderr);
        assert!(stderr_result.is_ok());

        // Check that the file exists
        let stderr_path = format!("{}/error.log", dot_dir);
        assert!(std::path::Path::new(&stderr_path).exists());
    }

    #[test]
    fn test_error_handling() {
        let adapter = FileAdapter::new(
            ".wez".to_string(),
            "output.log".to_string(),
            "error.log".to_string(),
        );

        // Try to read a non-existent file
        let result = adapter.read_from_file("non_existent_file.txt");
        assert!(result.is_err());
    }
}
