use anyhow::Result;
use crate::domain::models::TaskConfig;

/// Port for configuration management
#[cfg_attr(test, mockall::automock)]
pub trait ConfigPort {
    /// Load task configuration
    fn load_config(&self) -> Result<TaskConfig>;
    
    /// Create default configuration
    fn create_default_config(&self) -> Result<()>;
    
    /// View current configuration
    fn view_config(&self) -> Result<String>;
}