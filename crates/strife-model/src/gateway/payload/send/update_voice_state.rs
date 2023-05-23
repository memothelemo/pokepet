use crate::id::{ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdateBotVoiceState {
  pub guild_id: GuildId,
  pub channel_id: Option<ChannelId>,
  #[serde(rename = "self_mute")]
  pub mute: bool,
  #[serde(rename = "self_deaf")]
  pub deafen: bool,
}
