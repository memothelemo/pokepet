use crate::internal::macros::enum_int;

enum_int! {
  pub enum ConnectionVisibility: strict {
    None = 0,
    Everyone = 1,
  }
}

#[cfg(test)]
mod tests {
  use super::ConnectionVisibility;
  use serde_test::{assert_tokens, Token};

  #[test]
  fn serde() {
    assert_tokens(&ConnectionVisibility::None, &[Token::U64(0)]);
    assert_tokens(&ConnectionVisibility::Everyone, &[Token::U64(1)]);
  }
}
