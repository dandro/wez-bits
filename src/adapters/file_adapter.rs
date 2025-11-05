use std::fs::{create_dir, File};
use std::io::Write;

use anyhow::{Context, Result};
use log::info;

use crate::domain::models::FileSystemError;
use crate::ports::FileSystemPort;

#[derive(Debug, Clone)]
pub struct FileAdapter {}

impl FileAdapter {
    pub fn new() -> Self {
        Self {}
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_create_directory() {
        let temp_dir = tempdir().unwrap();
        let new_dir_path = temp_dir.path().join("test_dir");
        let new_dir_str = new_dir_path.to_str().unwrap();

        let adapter = FileAdapter::new();

        let result = adapter.create_directory(new_dir_str);
        assert!(result.is_ok());

        assert!(new_dir_path.exists());
        assert!(new_dir_path.is_dir());
    }

    #[test]
    fn test_read_from_file() {
        let content = "Hello, world!";
        let mut temp_file = NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut temp_file, content.as_bytes()).unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        let adapter = FileAdapter::new();

        let result = adapter.read_from_file(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_error_handling() {
        let adapter = FileAdapter::new();

        let result = adapter.read_from_file("non_existent_file.txt");
        assert!(result.is_err());
    }
}
