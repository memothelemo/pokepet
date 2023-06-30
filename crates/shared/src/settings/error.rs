use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
  #[error("failed to load settings")]
  Parse,
  #[error("{0:?} is not set!")]
  NotSet(&'static str),
}
