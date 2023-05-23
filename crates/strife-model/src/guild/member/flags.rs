use crate::internal::macros::bitflags;

bitflags! {
  pub struct GuildMemberFlags: u8 {
    const DID_REJOIN = 1 << 0;
    const COMPLETED_ONBOARDING = 1 << 1;
    const BYPASSES_VERIFICATION = 1 << 2;
    const STARTED_ONBOARDING = 1 << 3;
  }
}

impl GuildMemberFlags {
  pub const EDITABLE_FLAGS: Self = Self::from_bits_truncate(Self::BYPASSES_VERIFICATION.bits());

  pub const fn can_edit(&self) -> bool {
    self.intersects(Self::EDITABLE_FLAGS)
  }
}
