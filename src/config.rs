use std::{
    fs::{create_dir, File},
    io::Write,
    os::unix::process::ExitStatusExt,
    process::ExitStatus,
};

use log::info;

use crate::domain::AppErr;
use crate::{
    constants::{CONFIG_FILE, DOTDIR},
    domain::TasksConfig,
};

pub fn load_tasks_config() -> Result<TasksConfig, AppErr> {
    let path = format!("{DOTDIR}/{CONFIG_FILE}");
    info!("Load and parse configuration: {}", path);
    std::fs::read_to_string(path)
        .map_err(|e| AppErr::ProjectConfig(e.to_string()))
        .and_then(|config| {
            serde_json::from_str::<TasksConfig>(&config)
                .map_err(|e| AppErr::ProjectConfig(e.to_string()))
        })
}

const CONTENTS: &str = r#"
{
  "build": {
    "program": "npm",
    "args": ["run", "build"]
  },
  "format": {
    "program": "",
    "args": []
  },
  "run": {
    "program": "",
    "args": []
  },
  "test": {
    "program": "",
    "args": []
  },
  "check": {
    "program": "",
    "args": []
  },
  // non-interactive registers
  "q": {
    "program": "",
    "args": []
  },
  "w": {
    "program": "",
    "args": []
  },
  "e": {
    "program": "",
    "args": []
  },
  "y": {
    "program": "",
    "args": []
  },
  // interactive registers
  "Q": {
    "program": "",
    "args": []
  },
  "W": {
    "program": "",
    "args": []
  },
  "E": {
    "program": "",
    "args": []
  },
  "Y": {
    "program": "",
    "args": []
  }
}
"#;

pub fn create() -> Result<ExitStatus, AppErr> {
    info!("Creating {DOTDIR} directory");
    create_dir(DOTDIR).map_err(|e| AppErr::CreateConfig(e.to_string()))?;

    let path = format!("{DOTDIR}/{CONFIG_FILE}");
    info!("Creating {path}");
    let mut file = File::create(path).map_err(|e| AppErr::CreateConfig(e.to_string()))?;

    info!("Writing contents of the file");
    file.write(CONTENTS.as_bytes())
        .map_err(|e| AppErr::CreateConfig(e.to_string()))?;

    info!("Successfully created config at {DOTDIR}/{CONFIG_FILE}");
    Ok(ExitStatus::from_raw(0))
}

pub fn view() -> Result<ExitStatus, AppErr> {
    info!("Viewing config");
    let config = load_tasks_config()?;

    let commands = config
        .iter()
        .map(|(key, value)| format!("[{key}] {} {}\n", value.program, value.args.join(" ")))
        .collect::<String>();
    println!("{commands}");
    Ok(ExitStatus::from_raw(0))
}
