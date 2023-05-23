use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum ExplicitContentFilter(u8) {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
  }
}
