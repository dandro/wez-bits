use log::info;

use crate::domain::models::{AppError, TaskConfig};
use crate::ports::{ConfigPort, FileSystemPort};

const DEFAULT_CONFIG: &str = r#"
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
    fn load_config(&self) -> Result<TaskConfig, AppError> {
        let path = format!("{}/{}", self.dot_dir, self.config_file);
        info!("Load and parse configuration: {}", path);
        
        self.file_system
            .read_from_file(&path)
            .and_then(|config_str| {
                serde_json::from_str::<TaskConfig>(&config_str)
                    .map_err(|e| AppError::ConfigurationError(e.to_string()))
            })
    }

    fn create_default_config(&self) -> Result<(), AppError> {
        info!("Creating {} directory", self.dot_dir);
        self.file_system.create_directory(&self.dot_dir)?;

        let path = format!("{}/{}", self.dot_dir, self.config_file);
        info!("Creating {}", path);
        
        info!("Writing default configuration");
        self.file_system.write_to_file(&path, DEFAULT_CONFIG)?;
        
        info!("Successfully created config at {}/{}", self.dot_dir, self.config_file);
        Ok(())
    }

    fn view_config(&self) -> Result<String, AppError> {
        info!("Viewing config");
        let config = self.load_config()?;

        let output = config
            .iter()
            .map(|(key, value)| format!("[{}] {} {}\n", key, value.program, value.args.join(" ")))
            .collect::<String>();

        Ok(output)
    }
}