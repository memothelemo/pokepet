use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivitySecrets {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub join: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub spectate: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "match")]
  pub specified_match: Option<String>,
}
