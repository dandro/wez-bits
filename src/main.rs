use std::process::Command;

use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(Clone, ValueEnum)]
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

enum AppErr {
    ProjectConfig(String),
    FeatureNotConfigured(String),
    CommandFailed(String),
}

impl ToString for AppErr {
    fn to_string(&self) -> String {
        let msg = match &self {
            AppErr::ProjectConfig(msg) => format!("[kind] ProjectConfig [msg]: {} ", msg),
            AppErr::FeatureNotConfigured(msg) => {
                format!("[kind] FeatureNotConfigured [msg]: {} ", msg)
            }
            AppErr::CommandFailed(msg) => format!("[kind] CommandFailed [msg]: {} ", msg),
        };
        format!("[Helix Projectile Error] {} ", msg)
    }
}

fn main() {
    match run_app() {
        Err(err) => println!("{}", &err.to_string()),
        Ok(_) => (),
    }
}

fn run_app() -> Result<(), AppErr> {
    let project_config = load_config()?;
    let helix_projectile = HelixProjectile::parse();
    let cmd = get_cmd(helix_projectile, project_config)?;

    exec(cmd)
}

fn exec(cmd: ProjectileCommand) -> Result<(), AppErr> {
    Command::new(cmd.program)
        .args(cmd.args)
        .output()
        .map_err(|e| AppErr::CommandFailed(e.to_string()))
        .inspect(|output| println!("status: {}", output.status))
        .map(|_| ())
}

fn get_cmd(
    helix_projectile: HelixProjectile,
    project_config: ProjectileConfig,
) -> Result<ProjectileCommand, AppErr> {
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
    std::fs::read_to_string(".hx/hx-projectile.json")
        .map_err(|e| AppErr::ProjectConfig(e.to_string()))
        .and_then(|config| {
            serde_json::from_str::<ProjectileConfig>(&config)
                .map_err(|e| AppErr::ProjectConfig(e.to_string()))
        })
}
