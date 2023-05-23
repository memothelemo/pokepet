use crate::internal::macros::enum_int;

enum_int! {
  pub enum TeamMemberState {
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
    assert_tokens(&TeamMemberState::Invited, &[Token::U64(1)]);
    assert_tokens(&TeamMemberState::Accepted, &[Token::U64(2)]);
  }
}
