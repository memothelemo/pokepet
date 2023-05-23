use crate::gateway::{GatewayIntents, ShardId};
use serde::{Deserialize, Serialize};

fn default_large_threshold() -> u64 {
  50
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Identify {
  pub token: String,
  pub properties: IdentifyProps,
  #[serde(default)]
  pub compress: bool,
  #[serde(default = "default_large_threshold")]
  pub large_threshold: u64,
  pub shard: Option<ShardId>,
  pub intents: GatewayIntents,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct IdentifyProps {
  pub os: String,
  pub browser: String,
  pub device: String,
}
