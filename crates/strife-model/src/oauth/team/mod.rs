use crate::id::{TeamId, UserId};
use crate::misc::ImageHash;

use serde::{Deserialize, Serialize};

mod member;
mod member_state;

pub use member::*;
pub use member_state::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Team {
  pub id: TeamId,
  pub name: String,
  pub owner_user_id: UserId,
  pub icon: Option<ImageHash>,
  pub members: Vec<TeamMember>,
}
