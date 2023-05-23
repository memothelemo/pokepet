use crate::internal::macros::bitflags;

bitflags! {
  pub struct UserFlags: u32 {
    const DISCORD_EMPLOYEE = 1 << 0;
    const PARTNERED_SERVER_OWNER = 1 << 1;
    const HYPESQUAD_EVENTS = 1 << 2;
    const BUG_HUNTER_LEVEL_1 = 1 << 3;
    const HYPESQUAD_BRAVERY = 1 << 6;
    const HYPESQUAD_BRILLIANCE = 1 << 7;
    const HYPESQUAD_BALANCE = 1 << 8;
    const EARLY_PREMIUM_SUPPORTER = 1 << 9;
    const TEAM_PSEUDO_USER = 1 << 10;
    const BUG_HUNTER_LEVEL_2 = 1 << 14;
    const VERIFIED_BOT = 1 << 16;
    const EARLY_VERIFIED_BOT_DEVELOPER = 1 << 17;
    const CERTIFIED_MODERATOR = 1 << 18;
    const BOT_USE_ONLY_HTTP_INTERACTIONS = 1 << 19;
    const ACTIVE_DEVELOPER = 1 << 22;
  }
}

impl UserFlags {
  pub const fn is_discord_partner(&self) -> bool {
    self.intersects(Self::PARTNERED_SERVER_OWNER)
  }

  pub const fn is_verified_bot(&self) -> bool {
    self.intersects(Self::VERIFIED_BOT)
  }

  pub const fn is_discord_employee(&self) -> bool {
    self.intersects(Self::DISCORD_EMPLOYEE)
  }
}

impl UserFlags {
  pub const BUG_HUNTER: Self =
    Self::from_bits_truncate(Self::BUG_HUNTER_LEVEL_1.bits() | Self::BUG_HUNTER_LEVEL_2.bits());

  pub const fn is_bug_hunter(&self) -> bool {
    self.intersects(Self::BUG_HUNTER)
  }
}

impl UserFlags {
  pub const HYPESQUAD_MEMBER: Self = Self::from_bits_truncate(
    Self::HYPESQUAD_EVENTS.bits()
      | Self::HYPESQUAD_BRAVERY.bits()
      | Self::HYPESQUAD_BRILLIANCE.bits()
      | Self::HYPESQUAD_BALANCE.bits(),
  );

  pub const fn is_hypesquad_member(&self) -> bool {
    self.intersects(Self::HYPESQUAD_MEMBER)
  }
}

#[cfg(test)]
mod tests {
  use super::UserFlags;
  use serde_test::{assert_de_tokens, assert_tokens, Token};

  #[test]
  fn serde() {
    let valid = UserFlags::EARLY_PREMIUM_SUPPORTER | UserFlags::PARTNERED_SERVER_OWNER;
    assert_tokens(&valid, &[Token::U32(valid.bits())]);

    // Invalid flags
    assert_de_tokens(&UserFlags::empty(), &[Token::U32(1 << 30)]);

    // Invalid flags with valid flags
    assert_de_tokens(
      &UserFlags::VERIFIED_BOT,
      &[Token::U32((1 << 30) | UserFlags::VERIFIED_BOT.bits())],
    );
  }
}
