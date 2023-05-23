use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum ActivityType(u8): strict {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
  }
}
