use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum IntegrationExpireBehavior(u8) {
    RemoveRole = 0,
    Kick,
  }
}
