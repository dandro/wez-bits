use std::{
    fs::File,
    process::{Command, ExitStatus},
    thread,
    time::Duration,
};

use clap::{Parser, Subcommand};
use log::info;

use crate::{
    config::{self, load_tasks_config},
    constants::{BANNER, DOTDIR, ERROR_FILENAME, OUTPUT_FILENAME},
    domain::{AppErr, AppTask, CommandSettings, TasksConfig},
    wezterm::{close_pane, display_logs_in_pane, open_pane, pipe_stdout_to_pane, Direction},
};

/// Wez Bits - Project Scoped Interactivity
#[derive(Parser)]
#[command(name = "Wez Bits")]
#[command(version = "0.7.0")]
#[command(about = BANNER, long_about = None)]
struct App {
    #[command(subcommand)]
    cmd: AppSubCmd,
}

#[derive(Debug, Subcommand)]
enum AppSubCmd {
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

enum FilePurpose {
    Stdout,
    Stderr,
}

fn get_output_file(purpose: FilePurpose) -> Result<File, AppErr> {
    let name = match purpose {
        FilePurpose::Stdout => OUTPUT_FILENAME,
        FilePurpose::Stderr => ERROR_FILENAME,
    };
    let filename = format!("{DOTDIR}/{name}");
    info!("Creating output file: {}", filename);
    File::create(filename).map_err(|e| AppErr::OutputFile(e.to_string()))
}

fn exec(wez_bits_cmd: AppTask) -> Result<ExitStatus, AppErr> {
    let exit_status = if wez_bits_cmd.settings.interactive {
        interactive_cmd(wez_bits_cmd)?
    } else {
        non_interactive_cmd(wez_bits_cmd)?
    };

    Ok(exit_status)
}

fn interactive_cmd(wez_bits_cmd: AppTask) -> Result<ExitStatus, AppErr> {
    info!(
        "Executing interactive command: {} {:?}",
        wez_bits_cmd.cmd.program, wez_bits_cmd.cmd.args
    );

    let pane_id = open_pane(Direction::Right, 30)?;

    pipe_stdout_to_pane(
        [
            &[wez_bits_cmd.cmd.program],
            wez_bits_cmd.cmd.args.as_slice(),
        ]
        .concat(),
        pane_id,
    )
}

fn non_interactive_cmd(wez_bits_cmd: AppTask) -> Result<ExitStatus, AppErr> {
    let output = get_output_file(FilePurpose::Stdout)?;
    let error = get_output_file(FilePurpose::Stderr)?;
    let mini_buffer_id = open_pane(Direction::Down, 30)?;
    display_logs_in_pane(&mini_buffer_id)?;
    info!(
        "Executing command: {} {:?}",
        wez_bits_cmd.cmd.program, wez_bits_cmd.cmd.args
    );
    let exit_status = Command::new(wez_bits_cmd.cmd.program)
        .args(wez_bits_cmd.cmd.args)
        .stderr(error)
        .stdout(output)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|e| AppErr::CommandFailed(e.to_string()))?;

    if exit_status.success() {
        thread::sleep(Duration::from_secs(5));
        let _ = close_pane(&mini_buffer_id);
    }

    Ok(exit_status)
}

fn handle_command(wez_bits: App) -> Result<ExitStatus, AppErr> {
    info!("Matching application command");
    match wez_bits.cmd {
        AppSubCmd::TaskRunner { name, interactive } => {
            info!("Command: TaskRunner");
            info!("Find command ({:?}) in config file", name);
            let tasks_config = load_tasks_config()?;
            let cmd = handle_task_runner(name, interactive, tasks_config)?;
            exec(cmd)
        }
        AppSubCmd::Config { cmd } => {
            info!("Command: Config");
            match cmd {
                ConfigSubCmd::Create {} => {
                    info!("Sub Command: Create");
                    config::create()
                }
                ConfigSubCmd::View {} => {
                    info!("Sub Command: View");
                    config::view()
                }
            }
        }
    }
}

fn handle_task_runner(
    name: String,
    interactive: bool,
    tasks_config: TasksConfig,
) -> Result<AppTask, AppErr> {
    match tasks_config.get(&name) {
        Some(cmd) => Ok(AppTask {
            cmd: cmd.clone(),
            settings: CommandSettings { interactive },
        }),
        None => Err(AppErr::FeatureNotConfigured(format!(
            "Project {} feature not configured",
            &name
        ))),
    }
}

pub fn run_app() -> Result<ExitStatus, AppErr> {
    let app = App::parse();
    handle_command(app)
}
