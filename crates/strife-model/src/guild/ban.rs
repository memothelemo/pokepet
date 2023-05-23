use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GuildBan {
  pub reason: Option<String>,
  pub user: User,
}
