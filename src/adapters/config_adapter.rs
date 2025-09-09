use anyhow::{Context, Result};
use log::info;

use crate::domain::models::TaskConfig;
use crate::errors::ConfigError;
use crate::ports::{ConfigPort, FileSystemPort};

const DEFAULT_CONFIG: &str = r#"# WezBits Configuration

# Common tasks
[build]
program = "npm"
args = ["run", "build"]

[format]
program = ""
args = []

[run]
program = ""
args = []

[test]
program = ""
args = []

[check]
program = ""
args = []

# Non-interactive registers
[q]
program = ""
args = []

[w]
program = ""
args = []

[e]
program = ""
args = []

[y]
program = ""
args = []

# Interactive registers
[Q]
program = ""
args = []

[W]
program = ""
args = []

[E]
program = ""
args = []

[Y]
program = ""
args = []
"#;

pub struct ConfigAdapter<F: FileSystemPort> {
    file_system: F,
    dot_dir: String,
    config_file: String,
}

impl<F: FileSystemPort> ConfigAdapter<F> {
    pub fn new(file_system: F, dot_dir: String, config_file: String) -> Self {
        Self {
            file_system,
            dot_dir,
            config_file,
        }
    }
}

impl<F: FileSystemPort> ConfigPort for ConfigAdapter<F> {
    fn load_config(&self) -> Result<TaskConfig> {
        let path = format!("{}/{}", self.dot_dir, self.config_file);
        info!("Load and parse configuration: {}", path);
        
        let content = self.file_system.read_from_file(&path)?;
        
        toml::from_str::<TaskConfig>(&content)
            .with_context(|| ConfigError::Parse(format!("Failed to parse TOML config file: {}", path)))
    }

    fn create_default_config(&self) -> Result<()> {
        info!("Creating {} directory", self.dot_dir);
        self.file_system.create_directory(&self.dot_dir)
            .with_context(|| ConfigError::Create(format!("Failed to create directory: {}", self.dot_dir)))?;

        let path = format!("{}/{}", self.dot_dir, self.config_file);
        info!("Creating {}", path);
        
        info!("Writing default configuration");
        self.file_system.write_to_file(&path, DEFAULT_CONFIG)
            .with_context(|| ConfigError::Create(format!("Failed to write config file: {}", path)))?;
        
        info!("Successfully created config at {}/{}", self.dot_dir, self.config_file);
        Ok(())
    }

    fn view_config(&self) -> Result<String> {
        info!("Viewing config");
        let config = self.load_config()
            .with_context(|| ConfigError::Load("Failed to load configuration for viewing".to_string()))?;

        let output = config
            .iter()
            .map(|(key, value)| format!("[{}] {} {}\n", key, value.program, value.args.join(" ")))
            .collect::<String>();

        Ok(output)
    }
}