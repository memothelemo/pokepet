use crate::id::UserId;
use crate::misc::{Color, ImageHash, Locale};

use serde::{Deserialize, Serialize};

mod connection;
mod discriminator;
mod flags;
mod premium_type;

pub use connection::*;
pub use discriminator::*;
pub use flags::*;
pub use premium_type::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct User {
  pub id: UserId,
  #[serde(rename = "username")]
  pub name: String,
  pub discriminator: Discriminator,
  pub avatar: Option<ImageHash>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bot: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mfa_enabled: Option<bool>,
  pub banner: Option<ImageHash>,
  pub accent_color: Option<Color>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locale: Option<Locale>,
  pub verified: Option<bool>,
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flags: Option<UserFlags>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub premium_type: Option<UserPremiumType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub public_flags: Option<UserFlags>,
}
