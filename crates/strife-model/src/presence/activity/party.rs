use crate::internal::macros::size_array;
use serde::{Deserialize, Serialize};

size_array! {
  pub struct ActivityPartySize("activity party size");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivityParty {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub size: Option<ActivityPartySize>,
}
