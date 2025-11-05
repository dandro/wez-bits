use anyhow::{Context, Result};
use std::process::ExitStatus;

use crate::adapters::cli_adapter::CliAdapter;
use crate::adapters::config_adapter::ConfigAdapter;
use crate::adapters::file_adapter::FileAdapter;
use crate::adapters::terminal_adapter::TerminalAdapter;
use crate::constants::{CONFIG_FILE, DOTDIR};
use crate::domain::behaviours::TaskExecutionService;

pub struct Application;

impl Application {
    pub fn run() -> Result<ExitStatus> {
        // Create adapters
        let file_adapter = FileAdapter::new();

        let config_adapter = ConfigAdapter::new(
            file_adapter.clone(),
            DOTDIR.to_string(),
            CONFIG_FILE.to_string(),
        );

        let terminal_adapter = TerminalAdapter::new();

        let task_execution_service = TaskExecutionService::new(terminal_adapter);

        let cli_adapter = CliAdapter::new(config_adapter, task_execution_service);

        cli_adapter.run().context("Failed to run application")
    }
}
