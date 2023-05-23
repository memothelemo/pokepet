use crate::misc::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivityTimestamps {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(with = "crate::misc::timestamp::serde_impl::millis_opt")]
  #[serde(default)]
  pub start: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(with = "crate::misc::timestamp::serde_impl::millis_opt")]
  #[serde(default)]
  pub end: Option<Timestamp>,
}
