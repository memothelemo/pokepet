use std::num::NonZeroU64;

use doku::Document;
use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(Debug, Clone, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct Settings {
  pub bot: BotConfig,
  pub caching: Option<bool>,
  pub database: DatabaseConfig,
}

macro_rules! const_nonzero {
  ($tt:ident, $expr:expr) => {
    match $tt::new($expr) {
      Some(n) => n,
      None => panic!("default value is 0"),
    }
  };
}

const POOL_SIZE: NonZeroU64 = const_nonzero!(NonZeroU64, 5);
const POOL_TIMEOUT_SECS: NonZeroU64 = const_nonzero!(NonZeroU64, 5);

#[derive(Debug, Clone, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct BotConfig {
  #[doku(
    as = "String",
    example = "eDWu9NHJiQHg9uFYGH3aSMc2hA.u8mZhPv.eqK4vM2ZPwBJ9KBoxuwLAVWaf9QbfyDbVcz3DW"
  )]
  #[default("unset")]
  pub token: String,
}

#[derive(Debug, Clone, Deserialize, Document, SmartDefault)]
#[serde(default)]
pub struct DatabaseConfig {
  #[doku(as = "String", example = "redis://localhost:6379")]
  #[default("unset")]
  pub cache: String,
  #[doku(as = "String", example = "postgres://postgres@localhost")]
  #[default("unset")]
  pub uri: String,
  #[doku(as = "u64", example = "5")]
  #[default(POOL_SIZE)]
  pub pool_size: NonZeroU64,
  #[doku(as = "u64", example = "5")]
  #[default(POOL_TIMEOUT_SECS)]
  pub pool_timeout_secs: NonZeroU64,
}
