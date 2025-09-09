use std::process::ExitStatus;
use anyhow::{Context, Result};

use crate::adapters::cli_adapter::CliAdapter;
use crate::adapters::config_adapter::ConfigAdapter;
use crate::adapters::file_adapter::FileAdapter;
use crate::adapters::task_execution_adapter::TaskExecutionAdapter;
use crate::adapters::terminal_adapter::TerminalAdapter;
use crate::constants::{CONFIG_FILE, DOTDIR, ERROR_FILENAME, OUTPUT_FILENAME};
use crate::domain::behaviours::TaskExecutionService;

pub struct Application;

impl Application {
    pub fn run() -> Result<ExitStatus> {
        // Create adapters
        let file_adapter = FileAdapter::new(
            DOTDIR.to_string(),
            OUTPUT_FILENAME.to_string(),
            ERROR_FILENAME.to_string(),
        );

        let config_adapter = ConfigAdapter::new(
            file_adapter.clone(),
            DOTDIR.to_string(),
            CONFIG_FILE.to_string(),
        );

        let task_execution_adapter = TaskExecutionAdapter::new(file_adapter.clone());

        let terminal_adapter = TerminalAdapter::new(
            DOTDIR.to_string(),
            OUTPUT_FILENAME.to_string(),
            ERROR_FILENAME.to_string(),
        );

        // Create services
        let task_execution_service =
            TaskExecutionService::new(task_execution_adapter, terminal_adapter);

        // Create primary adapter
        let cli_adapter = CliAdapter::new(config_adapter, task_execution_service);

        // Run application
        cli_adapter.run().context("Failed to run application")
    }
}

// Need to implement Clone for FileAdapter for the config_adapter to work
impl Clone for FileAdapter {
    fn clone(&self) -> Self {
        Self::new(
            self.dot_dir.clone(),
            self.output_filename.clone(),
            self.error_filename.clone(),
        )
    }
}
