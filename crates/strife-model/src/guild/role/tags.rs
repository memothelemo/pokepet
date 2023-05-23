use crate::id::{IntegrationId, RoleSubscriptionSkuId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct RoleTags {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bot_id: Option<UserId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub integration_id: Option<IntegrationId>,
  #[serde(default, with = "null_value")]
  pub premium_subscriber: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_listing_id: Option<RoleSubscriptionSkuId>,
  #[serde(default, with = "null_value")]
  pub available_for_purchase: bool,
  #[serde(rename = "guild_connections")]
  #[serde(default, with = "null_value")]
  pub is_linked_role: bool,
}

mod null_value {
  use serde::{Deserializer, Serializer};

  pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_option(Visitor)
  }

  pub fn serialize<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_none()
  }

  struct Visitor;

  impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = bool;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str("null value")
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
      Ok(true)
    }
  }
}
