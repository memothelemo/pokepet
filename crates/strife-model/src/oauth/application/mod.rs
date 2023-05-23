use crate::id::{ApplicationId, GameSkuId, GuildId};
use crate::misc::ImageHash;
use crate::user::User;

use super::Team;
use serde::{Deserialize, Serialize};

mod flags;
mod install_params;

pub use flags::*;
pub use install_params::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PartialApplication {
  pub id: ApplicationId,
  pub flags: ApplicationFlags,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Application {
  pub id: ApplicationId,
  pub name: String,
  pub icon: Option<ImageHash>,
  pub description: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rpc_origins: Option<Vec<String>>,
  pub bot_public: bool,
  pub bot_require_code_grant: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub terms_of_service_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub privacy_policy_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub owner: Option<User>,
  pub verify_key: String,
  pub team: Option<Team>,
  pub guild_id: Option<GuildId>,
  pub primary_sku_id: Option<GameSkuId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cover_image: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flags: Option<ApplicationFlags>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub install_params: Option<InstallParams>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub custom_install_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role_connections_verification_url: Option<String>,
}
