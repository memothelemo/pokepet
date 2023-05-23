use crate::guild::GuildMember;
use crate::id::{ChannelId, GuildId, UserId};
use crate::misc::Timestamp;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct VoiceState {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub guild_id: Option<GuildId>,
  pub channel_id: Option<ChannelId>,
  pub user_id: UserId,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub member: Option<GuildMember>,
  pub session_id: String,
  #[serde(rename = "deaf")]
  pub deafen: bool,
  #[serde(rename = "mute")]
  pub muted: bool,
  #[serde(rename = "self_deaf")]
  pub self_deafen: bool,
  #[serde(rename = "self_mute")]
  pub self_muted: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "self_stream")]
  pub self_streaming: Option<bool>,
  #[serde(rename = "self_video")]
  pub self_video_enabled: bool,
  #[serde(rename = "suppress")]
  pub suppressed: bool,
  pub request_to_speak_timestamp: Option<Timestamp>,
}
