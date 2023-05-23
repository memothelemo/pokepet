use crate::id::RoleId;
use crate::misc::{Color, Permissions};

use serde::{Deserialize, Serialize};

mod tags;
pub use tags::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Role {
  pub id: RoleId,
  pub name: String,
  pub color: Color,
  #[serde(rename = "hoist")]
  pub pinned: bool,
  pub unicode_emoji: Option<String>,
  pub position: Option<u64>,
  pub permissions: Permissions,
  pub managed: bool,
  pub mentionable: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tags: Option<RoleTags>,
}
