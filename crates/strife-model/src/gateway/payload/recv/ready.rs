use crate::{gateway::ShardId, oauth::PartialApplication, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Ready {
  #[serde(rename = "v")]
  pub version: u64,
  pub user: User,
  // TODO: guilds: Vec<UnavailableGuild>,
  pub session_id: String,
  pub resume_gateway_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shard: Option<ShardId>,
  pub application: PartialApplication,
}
