use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum BoostLevel(u8) {
    None = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
  }
}