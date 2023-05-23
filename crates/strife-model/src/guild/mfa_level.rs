use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum MFALevel(u8): strict {
    None = 0,
    Elevated = 1,
  }
}
