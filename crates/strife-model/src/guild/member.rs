use crate::{
  id::RoleId,
  misc::{ImageHash, Permissions, Timestamp},
  user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuildMember {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<User>,
  #[serde(rename = "nickname")]
  pub nick: Option<String>,
  pub avatar: Option<ImageHash>,
  pub roles: Vec<RoleId>,
  pub joined_at: Timestamp,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "premium_since")]
  pub boosted_at: Option<Timestamp>,
  #[serde(rename = "deaf")]
  pub deafen: bool,
  #[serde(rename = "mute")]
  pub muted: bool,
  // TODO: flags
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pending: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub permissions: Option<Permissions>,
  pub timed_out_until: Option<Timestamp>,
}
