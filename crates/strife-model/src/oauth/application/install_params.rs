use crate::{misc::Permissions, oauth::OAuthScope};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct InstallParams {
  pub scopes: Vec<OAuthScope>,
  pub permissions: Permissions,
}
