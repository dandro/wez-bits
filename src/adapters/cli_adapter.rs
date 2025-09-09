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
