use crate::{
  misc::Timestamp,
  presence::{Activity, PresenceStatus},
};
use serde::{Deserialize, Serialize};

// To make it distingushable from other one that when a user
// updated their presence which is crate::gateway::payload::recv::UpdatedUserPresence
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdateBotPresence {
  #[serde(with = "crate::misc::timestamp::serde_impl::millis_opt")]
  pub since: Option<Timestamp>,
  pub activites: Vec<Activity>,
  pub status: PresenceStatus,
  pub afk: bool,
}
