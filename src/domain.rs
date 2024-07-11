#[derive(Debug)]
pub enum AppErr {
    ProjectConfig(String),
    FeatureNotConfigured(String),
    CommandFailed(String),
    OutputFile(String),
}

impl ToString for AppErr {
    fn to_string(&self) -> String {
        format!("{:?}", &self)
    }
}
