use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use log::info;

use crate::domain::behaviours::TaskExecutionService;
use crate::domain::models::{Direction, TaskClose};
use crate::ports::{ConfigPort, TerminalPort};

/// Application CLI command structure
#[derive(Parser)]
#[command(name = "Wez Bits")]
#[command(version = "0.10.0")]
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

        /// Configure when should a task pane close.
        #[arg(short, long, default_value = "on-success")]
        close: TaskCloseOption,

        /// Direction to open the panel
        #[arg(short, long, default_value = "right")]
        direction: TaskDirectionOption,

        /// Values the command can use when executed
        #[arg(short, long, value_parser=parsers::kv_pairs)]
        param: Vec<(String, String)>,
    },

    /// Interact with wez bits configuration
    Config {
        #[command(subcommand)]
        cmd: ConfigSubCmd,
    },
}

#[derive(ValueEnum, Debug, Clone)]
enum TaskCloseOption {
    Always,
    OnSuccess,
    Never,
}

impl TaskCloseOption {
    fn to_task_close(&self) -> TaskClose {
        match self {
            TaskCloseOption::Always => TaskClose::Always,
            TaskCloseOption::OnSuccess => TaskClose::OnSuccess,
            TaskCloseOption::Never => TaskClose::Never,
        }
    }
}

#[derive(ValueEnum, Debug, Clone)]
enum TaskDirectionOption {
    Right,
    Down,
}

impl TaskDirectionOption {
    fn to_task_direction(&self) -> Direction {
        match self {
            TaskDirectionOption::Right => Direction::Right,
            TaskDirectionOption::Down => Direction::Down,
        }
    }
}

#[derive(Debug, Subcommand)]
enum ConfigSubCmd {
    Create {},
    View {},
}

pub struct CliAdapter<C: ConfigPort, P: TerminalPort> {
    config_manager: C,
    task_service: TaskExecutionService<P>,
}

impl<C: ConfigPort, P: TerminalPort> CliAdapter<C, P> {
    pub fn new(config_manager: C, task_service: TaskExecutionService<P>) -> Self {
        Self {
            config_manager,
            task_service,
        }
    }

    pub fn run(&self) -> Result<ExitStatus> {
        let cli = Cli::parse();

        info!("Matching application command");
        match cli.cmd {
            CliSubCmd::TaskRunner {
                name,
                close,
                direction,
                param: params,
            } => self.handle_task_runner_command(name, close, direction, params),
            CliSubCmd::Config { cmd } => self.handle_config_command(cmd),
        }
    }

    fn handle_config_command(
        &self,
        cmd: ConfigSubCmd,
    ) -> std::result::Result<ExitStatus, anyhow::Error> {
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

    fn handle_task_runner_command(
        &self,
        name: String,
        close: TaskCloseOption,
        direction: TaskDirectionOption,
        params: Vec<(String, String)>,
    ) -> std::result::Result<ExitStatus, anyhow::Error> {
        info!("Command: TaskRunner");
        info!("Find command ({:?}) in config file", name);
        let tasks_config = self.config_manager.load_config()?;
        let task = self.task_service.find_task(
            &name,
            &tasks_config,
            close.to_task_close(),
            direction.to_task_direction(),
        )?;

        info!("Injecting params");
        let injected_task = self.task_service.apply_param_injections(
            task,
            params.into_iter().collect::<HashMap<String, String>>(),
        )?;

        info!("Executing task.");
        self.task_service.execute_task(injected_task)
    }
}

mod parsers {
    use anyhow::{anyhow, Result};
    use log::info;

    pub fn kv_pairs(s: &str) -> Result<(String, String)> {
        info!("Parsing kv_pair: {s}");
        let parts: Vec<&str> = s.splitn(2, '=').collect();
        if parts.len() == 2 {
            Ok((parts[0].to_string(), parts[1].to_string()))
        } else {
            Err(anyhow!(
                "Invalid key-value pair: {}. Expected format `key=value`",
                s
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Command;
    use crate::ports::mock::{MockConfigPort, MockTerminalPort};
    use std::collections::HashMap;
    use std::os::unix::process::ExitStatusExt;

    // Helper to create a mock config port
    fn setup_mock_config_port() -> MockConfigPort {
        let mut mock_config = MockConfigPort::new();

        // Setup default behavior
        let mut task_config = HashMap::new();
        task_config.insert(
            "test".to_string(),
            Command {
                program: "echo".to_string(),
                args: vec!["test".to_string()],
            },
        );

        mock_config
            .expect_load_config()
            .returning(move || Ok(task_config.clone()));

        mock_config
            .expect_create_default_config()
            .returning(|| Ok(()));

        mock_config
            .expect_view_config()
            .returning(|| Ok("[test] echo test\n".to_string()));

        mock_config
    }

    // Helper to create a mock terminal port
    fn setup_mock_terminal() -> MockTerminalPort {
        let mut mock_terminal = MockTerminalPort::new();

        mock_terminal
            .expect_open_pane()
            .returning(|_, _| Ok("test-pane-id".to_string()));

        mock_terminal
            .expect_pipe_text_to_pane()
            .returning(|_, _, _| Ok(ExitStatus::from_raw(0)));

        mock_terminal
    }

    #[test]
    fn test_cli_adapter_initialization() {
        let mock_config = setup_mock_config_port();
        let mock_terminal = setup_mock_terminal();

        let task_service = TaskExecutionService::new(mock_terminal);
        // Verify adapter can be created
        CliAdapter::new(mock_config, task_service);

        assert!(true);
    }
}
