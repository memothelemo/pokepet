use crate::internal::macros::event_type;

event_type!(EventType, {
  PresenceUpdate,
  Ready,
  Resumed,
  TypingStart,
});
