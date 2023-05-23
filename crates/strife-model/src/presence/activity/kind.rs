use crate::internal::macros::enum_int;

enum_int! {
  pub enum ActivityType: strict {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
  }
}
