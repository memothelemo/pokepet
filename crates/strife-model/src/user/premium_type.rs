use crate::internal::macros::enum_int;

enum_int! {
  // TODO: rename something descriptive
  pub enum UserPremiumType {
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
    assert_tokens(&UserPremiumType::None, &[Token::U64(0)]);
    assert_tokens(&UserPremiumType::Classic, &[Token::U64(1)]);
    assert_tokens(&UserPremiumType::Nitro, &[Token::U64(2)]);
    assert_tokens(&UserPremiumType::Basic, &[Token::U64(3)]);
  }
}
