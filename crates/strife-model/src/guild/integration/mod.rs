use crate::oauth::{Application, OAuthScope};
use crate::user::User;
use crate::{id::IntegrationId, misc::Timestamp};

use serde::{Deserialize, Serialize};

mod account;
mod application;
mod expire_behavior;

pub use account::*;
pub use application::*;
pub use expire_behavior::*;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Integration {
  pub id: IntegrationId,
  pub name: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub syncing: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role_id: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enable_emoticons: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expire_behavior: Option<IntegrationExpireBehavior>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expire_grace_period: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<User>,
  pub account: IntegrationAccount,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub synced_at: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscriber_count: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub revoked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub application: Option<Application>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scopes: Option<Vec<OAuthScope>>,
}
