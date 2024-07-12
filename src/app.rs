use std::{
    fs::File,
    process::{Command, ExitStatus},
    thread,
    time::Duration,
};

use clap::{Parser, ValueEnum};
use log::info;
use serde::Deserialize;

use crate::{
    domain::AppErr,
    wezterm::{close_pane, display_logs_in_pane, open_pane, Direction},
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
#[command(version = "0.2.0")]
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

Project Scoped Interactivity", long_about = None)]
struct HelixProjectile {
    /// Integration Feature
    #[arg(value_enum, short, long)]
    integration: ProjectileIntegration,
}

#[derive(Deserialize)]
struct ProjectileCommand {
    program: String,
    args: Vec<String>,
}

#[derive(Deserialize)]
struct ProjectileConfig {
    build: Option<ProjectileCommand>,
    format: Option<ProjectileCommand>,
    run: Option<ProjectileCommand>,
    test: Option<ProjectileCommand>,
}

enum FilePurpose {
    Stdout,
    Stderr,
}

fn get_output_file(purpose: FilePurpose) -> Result<File, AppErr> {
    let name = match purpose {
        FilePurpose::Stdout => "output",
        FilePurpose::Stderr => "errors",
    };
    let filename = format!(".hx/{}.log", name);
    info!("Creating output file: {}", filename);
    File::create(filename).map_err(|e| AppErr::OutputFile(e.to_string()))
}

fn exec(cmd: ProjectileCommand) -> Result<ExitStatus, AppErr> {
    let output = get_output_file(FilePurpose::Stdout)?;
    let error = get_output_file(FilePurpose::Stderr)?;

    let mini_buffer_id = open_pane(Direction::Down)?;
    let _ = display_logs_in_pane(&mini_buffer_id)?;

    info!("Executing command: {} {:?}", cmd.program, cmd.args);
    let exit_status = Command::new(cmd.program)
        .args(cmd.args)
        .stderr(error)
        .stdout(output)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|e| AppErr::CommandFailed(e.to_string()))?;

    thread::sleep(Duration::from_secs(3));
    let _ = close_pane(&mini_buffer_id);

    Ok(exit_status)
}

fn get_cmd(
    helix_projectile: HelixProjectile,
    project_config: ProjectileConfig,
) -> Result<ProjectileCommand, AppErr> {
    info!(
        "Find command ({:?}) in config file",
        helix_projectile.integration
    );
    match helix_projectile.integration {
        ProjectileIntegration::Build => project_config.build.ok_or(err_with("build")),
        ProjectileIntegration::Format => project_config.format.ok_or(err_with("format")),
        ProjectileIntegration::Run => project_config.run.ok_or(err_with("run")),
        ProjectileIntegration::Test => project_config.test.ok_or(err_with("test")),
    }
}

fn err_with(feature: &str) -> AppErr {
    AppErr::FeatureNotConfigured(format!("Project {} feature not configured", feature))
}

fn load_config() -> Result<ProjectileConfig, AppErr> {
    let path = ".hx/hx-projectile.json";
    info!("Load and parse configuration: {}", path);
    std::fs::read_to_string(path)
        .map_err(|e| AppErr::ProjectConfig(e.to_string()))
        .and_then(|config| {
            serde_json::from_str::<ProjectileConfig>(&config)
                .map_err(|e| AppErr::ProjectConfig(e.to_string()))
        })
}

pub fn run_app() -> Result<ExitStatus, AppErr> {
    let helix_projectile = HelixProjectile::parse();
    let project_config = load_config()?;
    let cmd = get_cmd(helix_projectile, project_config)?;

    exec(cmd)
}
