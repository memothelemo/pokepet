use crate::{
  gateway::payload::recv::{PresenceUpdate, Ready, TypingStart},
  internal::macros::enum_string,
};
use serde::{Deserialize, Serialize};

enum_string! {
  pub enum EventType {
    PresenceUpdate = "PRESENCE_UPDATE",
    Ready = "READY",
    Resumed = "RESUMED",
    TypingStart = "TYPING_START",
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum Event {
  Resumed,
  Ready(Ready),
  PresenceUpdate(PresenceUpdate),
  TypingStart(TypingStart),
}

impl Event {
  pub fn kind(&self) -> EventType {
    match self {
      Self::Resumed => EventType::Resumed,
      Self::Ready(..) => EventType::Ready,
      Self::PresenceUpdate(..) => EventType::PresenceUpdate,
      Self::TypingStart(..) => EventType::TypingStart,
    }
  }
}

pub(super) struct EventDeserializer(pub EventType);

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
      EventType::Resumed => Event::Resumed,
      EventType::TypingStart => Event::TypingStart(TypingStart::deserialize(deserializer)?),
      EventType::Unknown(n) => {
        crate::internal::macros::log_warn!("(EventDeserializer) unknown event type: {}", n);
        return Err(serde::de::Error::custom(format!(
          "unknown event type while deserializing 't' and 'd': {n}"
        )));
      }
    })
  }
}
