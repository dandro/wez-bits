use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppErr {
    ProjectConfig(String),
    FeatureNotConfigured(String),
    CommandFailed(String),
    OutputFile(String),
    CreateConfig(String),
}

impl Display for AppErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Deserialize, Clone)]
pub struct ConfigCommand {
    pub program: String,
    pub args: Vec<String>,
}

pub struct CommandSettings {
    pub interactive: bool,
}

pub struct ProjectileTask {
    pub cmd: ConfigCommand,
    pub settings: CommandSettings,
}
