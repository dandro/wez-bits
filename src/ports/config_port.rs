use crate::domain::models::{AppError, TaskConfig};

/// Port for configuration management
pub trait ConfigPort {
    /// Load task configuration
    fn load_config(&self) -> Result<TaskConfig, AppError>;
    
    /// Create default configuration
    fn create_default_config(&self) -> Result<(), AppError>;
    
    /// View current configuration
    fn view_config(&self) -> Result<String, AppError>;
}