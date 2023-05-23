use super::TeamMemberState;
use crate::{id::TeamId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TeamMember {
  #[serde(rename = "membership_state")]
  pub state: TeamMemberState,
  pub team_id: TeamId,
  pub user: User,
}

// Error from state field in TeamMember when implementing
// Hash trait using #[derive] macro.
impl std::hash::Hash for TeamMember {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.state.hash(state);
    self.team_id.hash(state);
    self.user.hash(state);
  }
}
