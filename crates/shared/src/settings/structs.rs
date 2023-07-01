use doku::Document;
use secrecy::SecretString;
use serde::Deserialize;
use smart_default::SmartDefault;
use std::num::{NonZeroU32, NonZeroU64};

#[derive(Debug, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct Settings {
  pub bot: BotConfig,
  pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct BotConfig {
  #[doku(as = "String")]
  #[default("unset".to_string().into())]
  pub token: SecretString,
}

#[derive(Debug, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct DatabaseConfig {
  /// Method to connect to the database, either by
  /// refering to the URI or parts with user, password,
  /// and other fields.
  #[serde(flatten, default)]
  pub connect_info: DatabaseConnection,
  #[doku(as = "u32", example = "5")]
  #[default(NonZeroU32::new(5).unwrap())]
  pub pool_size: NonZeroU32,
  #[doku(as = "u64", example = "5")]
  #[default(NonZeroU64::new(5).unwrap())]
  pub pool_timeout_secs: NonZeroU64,

  /// It enforces the program to connect to the database
  /// with SSL/TLS encryption.
  ///
  /// *If the connecting to the database with SSL/TLS
  /// encryption is not supported, then it will go back
  /// to the regular protocol.*
  ///
  /// **NOTE: If you use URI as your method to connect
  /// to the database and set something with enforce_tls field,
  /// your preferred SSL/TLS method in URI will be overriden.**
  pub enforce_tls: Option<bool>,
}

#[derive(SmartDefault)]
pub enum DatabaseConnection {
  Uri(SecretString),
  #[default]
  Parts(DatabaseConnectionParts),
}

#[derive(Debug, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct DatabaseConnectionParts {
  #[default("postgres")]
  #[doku(example = "postgres")]
  pub user: String,
  #[default("postgres".to_string().into())]
  #[doku(as = "String")]
  pub password: SecretString,
  #[default("localhost")]
  #[doku(example = "localhost")]
  pub host: String,
  #[default(5432)]
  #[doku(example = "5432")]
  pub port: u16,
  #[default("pokepet")]
  #[doku(example = "pokepet")]
  pub database: String,
}
