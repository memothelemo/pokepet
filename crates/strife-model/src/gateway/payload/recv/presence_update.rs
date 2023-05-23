use crate::{
  id::GuildId,
  presence::{Activity, ClientPresenceStatus, PresenceStatus},
  user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PresenceUpdate {
  pub user: User,
  pub guild_id: GuildId,
  pub status: PresenceStatus,
  pub activities: Vec<Activity>,
  pub client_status: ClientPresenceStatus,
}
