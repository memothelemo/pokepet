use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum GuildNSFWLevel(u8) {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
  }
}
