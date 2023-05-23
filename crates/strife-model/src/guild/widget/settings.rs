use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuildWidgetSettings {
  pub enabled: bool,
  pub channel_id: ChannelId,
}
