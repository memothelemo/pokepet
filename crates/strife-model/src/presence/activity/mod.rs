use crate::{id::ApplicationId, misc::Timestamp};
use serde::{Deserialize, Serialize};

mod assets;
mod button;
mod emoji;
mod flags;
mod kind;
mod party;
mod secrets;
mod timestamps;

pub use assets::*;
pub use button::*;
pub use emoji::*;
pub use flags::*;
pub use kind::*;
pub use party::*;
pub use secrets::*;
pub use timestamps::*;

// TODO: make builder something
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Activity {
  pub name: String,
  #[serde(rename = "type")]
  pub kind: ActivityType,
  pub url: Option<String>,
  #[serde(with = "crate::misc::timestamp::serde_impl::millis")]
  pub created_at: Timestamp,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamps: Option<ActivityTimestamps>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub application_id: Option<ApplicationId>,
  pub details: Option<String>,
  #[serde(rename = "state")]
  pub status: Option<String>,
  // TODO: emoji
  #[serde(skip_serializing_if = "Option::is_none")]
  pub party: Option<ActivityParty>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub assets: Option<ActivityAsset>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub secrets: Option<ActivitySecrets>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub instance: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flags: Option<ActivityFlags>,
  #[serde(default)]
  pub buttons: Vec<ActivityButton>,
}
