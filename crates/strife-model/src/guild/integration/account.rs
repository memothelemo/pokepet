use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct IntegrationAccount {
  pub id: String,
  pub name: String,
}
