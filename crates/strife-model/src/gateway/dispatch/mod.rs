use super::payload::recv::*;
use serde::{de::IgnoredAny, Deserialize, Serialize};

mod kind;
pub use kind::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum Event {
  PresenceUpdate(PresenceUpdate),
  Ready(Ready),
  Resumed,
  TypingStart(TypingStart),
}

impl Event {
  pub const fn kind(&self) -> EventType {
    match self {
      Self::PresenceUpdate(..) => EventType::PresenceUpdate,
      Self::Ready(..) => EventType::Ready,
      Self::Resumed => EventType::Resumed,
      Self::TypingStart(..) => EventType::TypingStart,
    }
  }
}

pub(crate) struct EventDeserializer(pub EventType);

impl<'de> serde::de::DeserializeSeed<'de> for EventDeserializer {
  type Value = Event;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(match self.0 {
      EventType::PresenceUpdate => {
        Event::PresenceUpdate(PresenceUpdate::deserialize(deserializer)?)
      }
      EventType::Ready => Event::Ready(Ready::deserialize(deserializer)?),
      EventType::Resumed => {
        deserializer.deserialize_ignored_any(IgnoredAny)?;
        Event::Resumed
      }
      EventType::TypingStart => Event::TypingStart(TypingStart::deserialize(deserializer)?),
      EventType::Unknown(n) => {
        return Err(serde::de::Error::custom(format!("unknown event for {n:?}")))
      }
    })
  }
}
