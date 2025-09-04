use std::{
    fs::File,
    process::{Command, ExitStatus},
    thread,
    time::Duration,
};

use clap::{Parser, Subcommand, ValueEnum};
use log::info;

use crate::{
    config::{self, load_tasks_config},
    constants::{DOTDIR, ERROR_FILENAME, OUTPUT_FILENAME},
    domain::{AppErr, CommandSettings, ProjectileTask, TasksConfig},
    wezterm::{close_pane, display_logs_in_pane, open_pane, pipe_stdout_to_pane, Direction},
};

#[derive(Clone, ValueEnum, Debug)]
enum ProjectileIntegration {
    Build,
    Format,
    Run,
    Test,
}

/// Helix Projectile - Project Scoped Interactivity
#[derive(Parser)]
#[command(name = "Helix Projectile")]
#[command(version = "0.6.0-rc21")]
#[command(about = "
 ██░ ██ ▓█████  ██▓     ██▓▒██   ██▒                                            
▓██░ ██▒▓█   ▀ ▓██▒    ▓██▒▒▒ █ █ ▒░                                            
▒██▀▀██░▒███   ▒██░    ▒██▒░░  █   ░                                            
░▓█ ░██ ▒▓█  ▄ ▒██░    ░██░ ░ █ █ ▒                                             
░▓█▒░██▓░▒████▒░██████▒░██░▒██▒ ▒██▒                                            
 ▒ ░░▒░▒░░ ▒░ ░░ ▒░▓  ░░▓  ▒▒ ░ ░▓ ░                                            
 ▒ ░▒░ ░ ░ ░  ░░ ░ ▒  ░ ▒ ░░░   ░▒ ░                                            
 ░  ░░ ░   ░     ░ ░    ▒ ░ ░    ░                                              
 ░  ░  ░   ░  ░    ░  ░ ░   ░    ░                                              
                                                                                
 ██▓███   ██▀███   ▒█████   ▄▄▄██▀▀▀▓█████  ▄████▄  ▄▄▄█████▓ ██▓ ██▓    ▓█████ 
▓██░  ██▒▓██ ▒ ██▒▒██▒  ██▒   ▒██   ▓█   ▀ ▒██▀ ▀█  ▓  ██▒ ▓▒▓██▒▓██▒    ▓█   ▀ 
▓██░ ██▓▒▓██ ░▄█ ▒▒██░  ██▒   ░██   ▒███   ▒▓█    ▄ ▒ ▓██░ ▒░▒██▒▒██░    ▒███   
▒██▄█▓▒ ▒▒██▀▀█▄  ▒██   ██░▓██▄██▓  ▒▓█  ▄ ▒▓▓▄ ▄██▒░ ▓██▓ ░ ░██░▒██░    ▒▓█  ▄ 
▒██▒ ░  ░░██▓ ▒██▒░ ████▓▒░ ▓███▒   ░▒████▒▒ ▓███▀ ░  ▒██▒ ░ ░██░░██████▒░▒████▒
▒▓▒░ ░  ░░ ▒▓ ░▒▓░░ ▒░▒░▒░  ▒▓▒▒░   ░░ ▒░ ░░ ░▒ ▒  ░  ▒ ░░   ░▓  ░ ▒░▓  ░░░ ▒░ ░
░▒ ░       ░▒ ░ ▒░  ░ ▒ ▒░  ▒ ░▒░    ░ ░  ░  ░  ▒       ░     ▒ ░░ ░ ▒  ░ ░ ░  ░
░░         ░░   ░ ░ ░ ░ ▒   ░ ░ ░      ░   ░          ░       ▒ ░  ░ ░      ░   
            ░         ░ ░   ░   ░      ░  ░░ ░                ░      ░  ░   ░  ░
                                           ░                                    

Project Utils for Helix", long_about = None)]
struct HelixProjectile {
    #[command(subcommand)]
    cmd: ProjectileSubCmd,
}

#[derive(Debug, Subcommand)]
enum ProjectileSubCmd {
    /// Run a project scoped task
    TaskRunner {
        /// Task name in config file
        name: String,

        /// When true, a wezterm pane will be opened and be left opened; otherwise, it will be closed when the task is completed.
        #[arg(short, long)]
        interactive: bool,
    },

    /// Interact with helix projectile configuration
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

fn exec(projectile_cmd: ProjectileTask) -> Result<ExitStatus, AppErr> {
    let exit_status = if projectile_cmd.settings.interactive {
        interactive_cmd(projectile_cmd)?
    } else {
        non_interactive_cmd(projectile_cmd)?
    };

    Ok(exit_status)
}

fn interactive_cmd(projectile_cmd: ProjectileTask) -> Result<ExitStatus, AppErr> {
    info!(
        "Executing interactive command: {} {:?}",
        projectile_cmd.cmd.program, projectile_cmd.cmd.args
    );

    let pane_id = open_pane(Direction::Right, 30)?;

    pipe_stdout_to_pane(
        [
            &[projectile_cmd.cmd.program],
            projectile_cmd.cmd.args.as_slice(),
        ]
        .concat(),
        pane_id,
    )
}

fn non_interactive_cmd(projectile_cmd: ProjectileTask) -> Result<ExitStatus, AppErr> {
    let output = get_output_file(FilePurpose::Stdout)?;
    let error = get_output_file(FilePurpose::Stderr)?;
    let mini_buffer_id = open_pane(Direction::Down, 30)?;
    display_logs_in_pane(&mini_buffer_id)?;
    info!(
        "Executing command: {} {:?}",
        projectile_cmd.cmd.program, projectile_cmd.cmd.args
    );
    let exit_status = Command::new(projectile_cmd.cmd.program)
        .args(projectile_cmd.cmd.args)
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

fn handle_command(helix_projectile: HelixProjectile) -> Result<ExitStatus, AppErr> {
    info!("Matching projectile command");
    match helix_projectile.cmd {
        ProjectileSubCmd::TaskRunner { name, interactive } => {
            info!("Command: TaskRunner");
            info!("Find command ({:?}) in config file", name);
            let tasks_config = load_tasks_config()?;
            let cmd = handle_task_runner(name, interactive, tasks_config)?;
            exec(cmd)
        }
        ProjectileSubCmd::Config { cmd } => {
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
) -> Result<ProjectileTask, AppErr> {
    match tasks_config.get(&name) {
        Some(cmd) => Ok(ProjectileTask {
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
    let helix_projectile = HelixProjectile::parse();
    handle_command(helix_projectile)
}
