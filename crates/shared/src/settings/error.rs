use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to parse settings")]
pub struct SettingsParseError;
