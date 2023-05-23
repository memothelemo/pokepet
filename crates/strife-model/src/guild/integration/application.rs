use crate::id::IntegrationApplicationId;
use crate::misc::ImageHash;
use crate::user::User;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct IntegrationApplication {
  pub id: IntegrationApplicationId,
  pub name: String,
  pub icon: Option<ImageHash>,
  pub description: String,
  pub bot: Option<User>,
}
