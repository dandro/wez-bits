use std::{
    fs::File,
    process::{Command, ExitStatus},
};

use clap::{Parser, ValueEnum};
use log::info;
use serde::Deserialize;

#[derive(Clone, ValueEnum, Debug)]
enum ProjectileIntegration {
    Build,
    Format,
    Run,
    Test,
}

/// Helix Projectile - Project Scoped Interactivity
#[derive(Parser)]
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

#[derive(Debug)]
pub enum AppErr {
    ProjectConfig(String),
    FeatureNotConfigured(String),
    CommandFailed(String),
    OutputFile(String),
}

impl ToString for AppErr {
    fn to_string(&self) -> String {
        format!("{:?}", &self)
    }
}

fn get_output_file() -> Result<File, AppErr> {
    let filename = format!(".hx/output.log");
    info!("Creating output file: {}", filename);
    File::create(filename).map_err(|e| AppErr::OutputFile(e.to_string()))
}

fn exec(cmd: ProjectileCommand) -> Result<ExitStatus, AppErr> {
    let output = get_output_file()?;
    info!("Executing command: {} {:?}", cmd.program, cmd.args);
    Command::new(cmd.program)
        .args(cmd.args)
        .stderr(output)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|e| AppErr::CommandFailed(e.to_string()))
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
    let project_config = load_config()?;
    let helix_projectile = HelixProjectile::parse();
    let cmd = get_cmd(helix_projectile, project_config)?;

    exec(cmd)
}
