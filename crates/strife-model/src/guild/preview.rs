use super::GuildFeature;
use crate::{id::GuildId, misc::ImageHash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuildPreview {
  pub id: GuildId,
  pub name: String,
  pub icon: Option<ImageHash>,
  pub splash: Option<ImageHash>,
  pub discovery_splash: Option<ImageHash>,
  // TODO: emojis
  pub features: Vec<GuildFeature>,
  #[serde(rename = "approximate_member_count")]
  pub approx_members: Option<u64>,
  #[serde(rename = "approximate_presence_count")]
  pub approx_presences: Option<u64>,
  pub description: Option<String>,
  // TODO: stickers,
}
