use crate::internal::macros::enum_repr;

enum_repr! {
  pub enum MessageNotificationLevel(u8) {
    AllMessages = 0,
    OnlyMentions = 1,
  }
}