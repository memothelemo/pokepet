use crate::id::GuildId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UnavailableGuild {
  pub id: GuildId,
  #[serde(default)]
  pub unavailable: bool,
}
