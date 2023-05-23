use crate::{
  guild::GuildMember,
  id::{ChannelId, GuildId, UserId},
  misc::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TypingStart {
  pub channel_id: ChannelId,
  pub guild_id: Option<GuildId>,
  pub user_id: UserId,
  #[serde(with = "crate::misc::timestamp::serde_impl::secs")]
  pub timestamp: Timestamp,
  pub member: GuildMember,
}
