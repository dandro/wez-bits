use std::{
    fs::{create_dir, File},
    io::Write,
    os::unix::process::ExitStatusExt,
    process::ExitStatus,
};

use log::info;

use crate::constants::{CONFIG_FILE, DOTDIR};
use crate::domain::AppErr;

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
    let _ = create_dir(DOTDIR).map_err(|e| AppErr::CreateConfig(e.to_string()))?;

    let path = format!("{DOTDIR}/{CONFIG_FILE}");
    info!("Creating {path}");
    let mut file = File::create(path).map_err(|e| AppErr::CreateConfig(e.to_string()))?;

    info!("Writing contents of the file");
    file.write(CONTENTS.as_bytes())
        .map_err(|e| AppErr::CreateConfig(e.to_string()))?;

    info!("Successfully created config at {DOTDIR}/{CONFIG_FILE}");
    Ok(ExitStatus::from_raw(0))
}
