use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum VerificationLevel(u8): strict {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
  }
}
