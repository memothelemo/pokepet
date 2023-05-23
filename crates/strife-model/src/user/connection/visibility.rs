use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum ConnectionVisibility(u8): strict {
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
    assert_tokens(&ConnectionVisibility::None, &[Token::U8(0)]);
    assert_tokens(&ConnectionVisibility::Everyone, &[Token::U8(1)]);
  }
}
