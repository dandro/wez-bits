use anyhow::{Context, Result};
use log::info;

use crate::domain::models::{ConfigError, TaskConfig};
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

        toml::from_str::<TaskConfig>(&content).with_context(|| {
            ConfigError::Parse(format!("Failed to parse TOML config file: {}", path))
        })
    }

    fn create_default_config(&self) -> Result<()> {
        info!("Creating {} directory", self.dot_dir);
        self.file_system
            .create_directory(&self.dot_dir)
            .with_context(|| {
                ConfigError::Create(format!("Failed to create directory: {}", self.dot_dir))
            })?;

        let path = format!("{}/{}", self.dot_dir, self.config_file);
        info!("Creating {}", path);

        info!("Writing default configuration");
        self.file_system
            .write_to_file(&path, DEFAULT_CONFIG)
            .with_context(|| {
                ConfigError::Create(format!("Failed to write config file: {}", path))
            })?;

        info!(
            "Successfully created config at {}/{}",
            self.dot_dir, self.config_file
        );
        Ok(())
    }

    fn view_config(&self) -> Result<String> {
        info!("Viewing config");
        let config = self.load_config().with_context(|| {
            ConfigError::Load("Failed to load configuration for viewing".to_string())
        })?;

        let output = config
            .iter()
            .map(|(key, value)| format!("[{}] {} {}\n", key, value.program, value.args.join(" ")))
            .collect::<String>();

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::mock::MockFileSystemPort;

    fn create_valid_toml_config() -> String {
        r#"
[build]
program = "npm"
args = ["run", "build"]

[test]
program = "npm"
args = ["run", "test"]
"#
        .to_string()
    }

    #[test]
    fn test_load_config_success() {
        let mut mock_fs = MockFileSystemPort::new();
        let config_path = ".wez/config.toml";

        // Setup expectations
        mock_fs
            .expect_read_from_file()
            .with(mockall::predicate::eq(config_path))
            .times(1)
            .returning(|_| Ok(create_valid_toml_config()));

        let adapter = ConfigAdapter::new(mock_fs, ".wez".to_string(), "config.toml".to_string());

        // Load config
        let result = adapter.load_config();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.len(), 2);

        let build_cmd = config.get("build").unwrap();
        assert_eq!(build_cmd.program, "npm");
        assert_eq!(build_cmd.args, vec!["run", "build"]);

        let test_cmd = config.get("test").unwrap();
        assert_eq!(test_cmd.program, "npm");
        assert_eq!(test_cmd.args, vec!["run", "test"]);
    }

    #[test]
    fn test_load_config_file_error() {
        let mut mock_fs = MockFileSystemPort::new();
        let config_path = ".wez/config.toml";

        // Setup expectations - simulate file not found
        mock_fs
            .expect_read_from_file()
            .with(mockall::predicate::eq(config_path))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("File not found")));

        let adapter = ConfigAdapter::new(mock_fs, ".wez".to_string(), "config.toml".to_string());

        // Load config
        let result = adapter.load_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_parse_error() {
        let mut mock_fs = MockFileSystemPort::new();
        let config_path = ".wez/config.toml";

        // Setup expectations - return invalid TOML
        mock_fs
            .expect_read_from_file()
            .with(mockall::predicate::eq(config_path))
            .times(1)
            .returning(|_| Ok("This is not valid TOML".to_string()));

        let adapter = ConfigAdapter::new(mock_fs, ".wez".to_string(), "config.toml".to_string());

        // Load config
        let result = adapter.load_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_default_config() {
        let mut mock_fs = MockFileSystemPort::new();
        let dot_dir = ".wez";
        let config_path = ".wez/config.toml";

        // Setup expectations
        mock_fs
            .expect_create_directory()
            .with(mockall::predicate::eq(dot_dir))
            .times(1)
            .returning(|_| Ok(()));

        mock_fs
            .expect_write_to_file()
            .with(
                mockall::predicate::eq(config_path),
                mockall::predicate::eq(DEFAULT_CONFIG),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let adapter = ConfigAdapter::new(mock_fs, dot_dir.to_string(), "config.toml".to_string());

        // Create default config
        let result = adapter.create_default_config();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_default_config_directory_error() {
        let mut mock_fs = MockFileSystemPort::new();
        let dot_dir = ".wez";

        // Setup expectations - simulate directory creation error
        mock_fs
            .expect_create_directory()
            .with(mockall::predicate::eq(dot_dir))
            .times(1)
            .returning(|_| Err(anyhow::anyhow!("Directory creation failed")));

        let adapter = ConfigAdapter::new(mock_fs, dot_dir.to_string(), "config.toml".to_string());

        // Create default config
        let result = adapter.create_default_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_view_config() {
        let mut mock_fs = MockFileSystemPort::new();
        let config_path = ".wez/config.toml";

        // Setup expectations
        mock_fs
            .expect_read_from_file()
            .with(mockall::predicate::eq(config_path))
            .times(1)
            .returning(|_| Ok(create_valid_toml_config()));

        let adapter = ConfigAdapter::new(mock_fs, ".wez".to_string(), "config.toml".to_string());

        // View config
        let result = adapter.view_config();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("[build] npm run build"));
        assert!(output.contains("[test] npm run test"));
    }
}
