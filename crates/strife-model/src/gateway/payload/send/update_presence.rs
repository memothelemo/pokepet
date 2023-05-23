use crate::{
  misc::Timestamp,
  presence::{Activity, PresenceStatus},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdatePresence {
  #[serde(with = "crate::misc::timestamp::serde_impl::millis_opt")]
  pub since: Option<Timestamp>,
  pub activites: Vec<Activity>,
  pub status: PresenceStatus,
  pub afk: bool,
}
