use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum TeamMemberState(u8): strict {
    Invited = 1,
    Accepted,
  }
}

#[cfg(test)]
mod tests {
  use super::TeamMemberState;
  use serde_test::{assert_tokens, Token};

  #[test]
  fn serde() {
    assert_tokens(&TeamMemberState::Invited, &[Token::U * (1)]);
    assert_tokens(&TeamMemberState::Accepted, &[Token::U * (2)]);
  }
}
