use std::fs::{create_dir, File};
use std::io::Write;

use anyhow::{Context, Result};
use log::info;

use crate::domain::models::OutputType;
use crate::errors::FileSystemError;
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
        let mut file = File::create(path)
            .with_context(|| FileSystemError::Write(path.to_string()))?;
        
        file.write_all(content.as_bytes())
            .with_context(|| FileSystemError::Write(path.to_string()))?;
        
        Ok(())
    }

    fn read_from_file(&self, path: &str) -> Result<String> {
        info!("Reading from file: {}", path);
        std::fs::read_to_string(path)
            .with_context(|| FileSystemError::Read(path.to_string()))
    }

    fn get_output_file(&self, output_type: OutputType) -> Result<File> {
        let name = match output_type {
            OutputType::Stdout => &self.output_filename,
            OutputType::Stderr => &self.error_filename,
        };
        let filename = format!("{}/{}", self.dot_dir, name);
        info!("Creating output file: {}", filename);
        
        File::create(&filename)
            .with_context(|| FileSystemError::CreateOutputFile(filename.clone()))
    }
}