use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

use crate::domain::behaviours::TaskExecutionService;
use crate::ports::{ConfigPort, TaskExecutionPort, TerminalPort};

/// Application CLI command structure
#[derive(Parser)]
#[command(name = "Wez Bits")]
#[command(version = "0.7.0")]
#[command(about = crate::constants::BANNER, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: CliSubCmd,
}

#[derive(Debug, Subcommand)]
enum CliSubCmd {
    /// Run a project scoped task
    TaskRunner {
        /// Task name in config file
        name: String,

        /// When true, a wezterm pane will be opened and be left opened; otherwise, it will be closed when the task is completed.
        #[arg(short, long)]
        interactive: bool,
    },

    /// Interact with wez bits configuration
    Config {
        #[command(subcommand)]
        cmd: ConfigSubCmd,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigSubCmd {
    Create {},
    View {},
}

pub struct CliAdapter<C: ConfigPort, T: TaskExecutionPort, P: TerminalPort> {
    config_manager: C,
    task_service: TaskExecutionService<T, P>,
}

impl<C: ConfigPort, T: TaskExecutionPort, P: TerminalPort> CliAdapter<C, T, P> {
    pub fn new(config_manager: C, task_service: TaskExecutionService<T, P>) -> Self {
        Self {
            config_manager,
            task_service,
        }
    }

    pub fn run(&self) -> Result<ExitStatus> {
        let cli = Cli::parse();
        self.handle_command(cli)
    }

    fn handle_command(&self, cli: Cli) -> Result<ExitStatus> {
        info!("Matching application command");
        match cli.cmd {
            CliSubCmd::TaskRunner { name, interactive } => {
                info!("Command: TaskRunner");
                info!("Find command ({:?}) in config file", name);
                let tasks_config = self.config_manager.load_config()?;
                let task = self
                    .task_service
                    .find_task(&name, &tasks_config, interactive)?;
                self.task_service.execute_task(task)
            }
            CliSubCmd::Config { cmd } => {
                info!("Command: Config");
                match cmd {
                    ConfigSubCmd::Create {} => {
                        info!("Sub Command: Create");
                        self.config_manager.create_default_config()?;
                        Ok(ExitStatus::from_raw(0))
                    }
                    ConfigSubCmd::View {} => {
                        info!("Sub Command: View");
                        let config_str = self.config_manager.view_config()?;
                        println!("{}", config_str);
                        Ok(ExitStatus::from_raw(0))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Command;
    use crate::ports::mock::{MockConfigPort, MockTaskExecutionPort, MockTerminalPort};
    use std::collections::HashMap;
    use std::os::unix::process::ExitStatusExt;
    
    // Helper to create a mock config port
    fn setup_mock_config_port() -> MockConfigPort {
        let mut mock_config = MockConfigPort::new();
        
        // Setup default behavior
        let mut task_config = HashMap::new();
        task_config.insert("test".to_string(), Command {
            program: "echo".to_string(),
            args: vec!["test".to_string()]
        });
        
        mock_config.expect_load_config()
            .returning(move || Ok(task_config.clone()));
            
        mock_config.expect_create_default_config()
            .returning(|| Ok(()));
            
        mock_config.expect_view_config()
            .returning(|| Ok("[test] echo test\n".to_string()));
            
        mock_config
    }
    
    // Helper to create a mock task execution port
    fn setup_mock_task_executor() -> MockTaskExecutionPort {
        let mut mock_executor = MockTaskExecutionPort::new();
        
        mock_executor.expect_execute_command()
            .returning(|_| Ok(ExitStatus::from_raw(0)));
            
        mock_executor
    }
    
    // Helper to create a mock terminal port
    fn setup_mock_terminal() -> MockTerminalPort {
        let mut mock_terminal = MockTerminalPort::new();
        
        mock_terminal.expect_open_pane()
            .returning(|_, _| Ok("test-pane-id".to_string()));
            
        mock_terminal.expect_close_pane()
            .returning(|_| Ok(()));
            
        mock_terminal.expect_display_logs_in_pane()
            .returning(|_| Ok(()));
            
        mock_terminal.expect_pipe_text_to_pane()
            .returning(|_, _| Ok(ExitStatus::from_raw(0)));
            
        mock_terminal
    }
    
    #[test]
    fn test_cli_adapter_initialization() {
        let mock_config = setup_mock_config_port();
        let mock_executor = setup_mock_task_executor();
        let mock_terminal = setup_mock_terminal();
        
        let task_service = TaskExecutionService::new(mock_executor, mock_terminal);
        // Verify adapter can be created
        CliAdapter::new(mock_config, task_service);
        
        assert!(true);
    }
    
    #[test]
    fn test_handle_config_create_command() {
        let mut mock_config = MockConfigPort::new();
        mock_config.expect_create_default_config()
            .times(1)
            .returning(|| Ok(()));
            
        let mock_executor = setup_mock_task_executor();
        let mock_terminal = setup_mock_terminal();
        
        let task_service = TaskExecutionService::new(mock_executor, mock_terminal);
        let adapter = CliAdapter::new(mock_config, task_service);
        
        // Create a Config Create command
        let cli = Cli {
            cmd: CliSubCmd::Config {
                cmd: ConfigSubCmd::Create {}
            }
        };
        
        // Handle the command
        let result = adapter.handle_command(cli);
        
        // Check that execution succeeded
        assert!(result.is_ok());
        assert_eq!(result.unwrap().code(), Some(0));
    }
    
    #[test]
    fn test_handle_config_view_command() {
        let mut mock_config = MockConfigPort::new();
        mock_config.expect_view_config()
            .times(1)
            .returning(|| Ok("[test] echo test\n".to_string()));
            
        let mock_executor = setup_mock_task_executor();
        let mock_terminal = setup_mock_terminal();
        
        let task_service = TaskExecutionService::new(mock_executor, mock_terminal);
        let adapter = CliAdapter::new(mock_config, task_service);
        
        // Create a Config View command
        let cli = Cli {
            cmd: CliSubCmd::Config {
                cmd: ConfigSubCmd::View {}
            }
        };
        
        // Handle the command
        let result = adapter.handle_command(cli);
        
        // Check that execution succeeded
        assert!(result.is_ok());
        assert_eq!(result.unwrap().code(), Some(0));
    }
    
    #[test]
    fn test_handle_task_runner_command() {
        // Setup mock config that returns a task config with a "build" task
        let mut mock_config = MockConfigPort::new();
        let mut task_config = HashMap::new();
        task_config.insert("build".to_string(), Command {
            program: "npm".to_string(),
            args: vec!["run".to_string(), "build".to_string()]
        });
        
        mock_config.expect_load_config()
            .times(1)
            .returning(move || Ok(task_config.clone()));
        
        // Setup mock executor that expects to run the build command
        let mut mock_executor = MockTaskExecutionPort::new();
        mock_executor.expect_execute_command()
            .withf(|cmd| {
                cmd.program == "npm" && cmd.args == vec!["run", "build"]
            })
            .times(1)
            .returning(|_| Ok(ExitStatus::from_raw(0)));
        
        // Setup mock terminal for task execution
        let mut mock_terminal = MockTerminalPort::new();
        mock_terminal.expect_open_pane()
            .times(1)
            .returning(|_, _| Ok("test-pane-id".to_string()));
            
        mock_terminal.expect_display_logs_in_pane()
            .times(1)
            .returning(|_| Ok(()));
            
        mock_terminal.expect_close_pane()
            .times(1)
            .returning(|_| Ok(()));
        
        let task_service = TaskExecutionService::new(mock_executor, mock_terminal);
        let adapter = CliAdapter::new(mock_config, task_service);
        
        // Create a TaskRunner command for the "build" task
        let cli = Cli {
            cmd: CliSubCmd::TaskRunner {
                name: "build".to_string(),
                interactive: false
            }
        };
        
        // Handle the command
        let result = adapter.handle_command(cli);
        
        // Check that execution succeeded
        assert!(result.is_ok());
        assert_eq!(result.unwrap().code(), Some(0));
    }
    
    #[test]
    fn test_task_runner_with_nonexistent_task() {
        // Setup mock config that returns an empty task config
        let mut mock_config = MockConfigPort::new();
        mock_config.expect_load_config()
            .times(1)
            .returning(|| Ok(HashMap::new()));
        
        let mock_executor = setup_mock_task_executor();
        let mock_terminal = setup_mock_terminal();
        
        let task_service = TaskExecutionService::new(mock_executor, mock_terminal);
        let adapter = CliAdapter::new(mock_config, task_service);
        
        // Create a TaskRunner command for a non-existent task
        let cli = Cli {
            cmd: CliSubCmd::TaskRunner {
                name: "nonexistent".to_string(),
                interactive: false
            }
        };
        
        // Handle the command
        let result = adapter.handle_command(cli);
        
        // Check that execution failed
        assert!(result.is_err());
        
        // Check error message indicates task not found
        let err = result.unwrap_err();
        let err_string = err.to_string();
        assert!(err_string.contains("nonexistent"));
    }
}
