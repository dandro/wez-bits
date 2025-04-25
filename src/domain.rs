use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppErr {
    ProjectConfig(String),
    FeatureNotConfigured(String),
    CommandFailed(String),
    OutputFile(String),
}

impl Display for AppErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
