use crate::id::{ChannelId, GuildId};
use crate::user::User;

use serde::{Deserialize, Serialize};

mod settings;
pub use settings::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuildWidgetChannel {
  pub id: ChannelId,
  pub name: String,
  pub position: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuildWidget {
  pub id: GuildId,
  pub name: String,
  pub instant_invite: Option<String>,
  pub channels: Vec<GuildWidgetChannel>,
  pub members: Vec<User>,
  pub presence_count: u64,
}
