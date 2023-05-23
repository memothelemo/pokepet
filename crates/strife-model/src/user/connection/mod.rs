use serde::{Deserialize, Serialize};

mod service;
mod visibility;

pub use service::*;
pub use visibility::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Connection {
  pub id: String,
  pub name: String,
  #[serde(rename = "type")]
  pub service: ConnectionService,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub revoked: Option<bool>,
  // TODO: integrations?: Vec<ServerIntegration>,
  pub verified: bool,
  pub friend_sync: bool,
  pub show_activity: bool,
  pub two_way_link: bool,
  pub visibility: ConnectionVisibility,
}
