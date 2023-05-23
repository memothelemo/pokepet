use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct VoiceRegion {
  pub id: String,
  pub name: String,
  pub optimal: bool,
  pub deprecated: bool,
  pub custom: bool,
}
