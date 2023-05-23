use crate::id::EmojiId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivityEmoji {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<EmojiId>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub animated: Option<bool>,
}
