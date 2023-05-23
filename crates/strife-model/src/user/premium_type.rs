use crate::internal::macros::enum_repr;

enum_repr! {
  // TODO: rename something descriptive
  pub enum UserPremiumType(u8) {
    None = 0,
    Classic,
    Nitro,
    Basic,
  }
}

#[cfg(test)]
mod tests {
  use super::UserPremiumType;
  use serde_test::{assert_tokens, Token};

  #[test]
  fn serde() {
    assert_tokens(&UserPremiumType::None, &[Token::U8(0)]);
    assert_tokens(&UserPremiumType::Classic, &[Token::U8(1)]);
    assert_tokens(&UserPremiumType::Nitro, &[Token::U8(2)]);
    assert_tokens(&UserPremiumType::Basic, &[Token::U8(3)]);
  }
}
