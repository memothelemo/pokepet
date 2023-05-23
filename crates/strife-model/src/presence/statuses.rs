use crate::internal::macros::enum_string;
use serde::{Deserialize, Serialize};

enum_string! {
  pub enum PresenceStatus: strict {
    Online = "online",
    DoNotDisturb = "dnd",
    AFK = "idle",
    Invisible = "invisible",
    Offline = "offline",
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ClientPresenceStatus {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub desktop: Option<PresenceStatus>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mobile: Option<PresenceStatus>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub web: Option<PresenceStatus>,
}
