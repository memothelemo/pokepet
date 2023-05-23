use std::{fmt::Debug, marker::PhantomData};

use crate::gateway::GatewayOpcode;

use super::payload::recv::{Hello, InvalidSession};
use serde::{de::DeserializeOwned, Deserialize};

mod dispatch;
pub use dispatch::*;

struct ValueDeserializer<T: DeserializeOwned> {
  _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> ValueDeserializer<T> {
  pub fn new() -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<'de, T: DeserializeOwned> serde::de::DeserializeSeed<'de> for ValueDeserializer<T> {
  type Value = T;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    T::deserialize(deserializer)
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GatewayEvent {
  Dispatch(u64, Box<Event>),
  Hello(Hello),
  InvalidSession(InvalidSession),
  Reconnect,
  Heartbeat,
  HeartbeatAck,
}

impl std::fmt::Debug for GatewayEvent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Dispatch(s, e) => {
        write!(f, "Dispatch({s}, ")?;
        e.fmt(f)?;
        write!(f, ")")
      }
      Self::Hello(n) => n.fmt(f),
      Self::InvalidSession(n) => n.fmt(f),
      Self::Reconnect => f.write_str("Reconnect"),
      Self::Heartbeat => f.write_str("Heartbeat"),
      Self::HeartbeatAck => f.write_str("HeartbeatAck"),
    }
  }
}

impl GatewayEvent {
  pub(crate) fn opcode(&self) -> GatewayOpcode {
    match self {
      Self::Dispatch(..) => GatewayOpcode::Dispatch,
      Self::Hello(..) => GatewayOpcode::Hello,
      Self::InvalidSession(..) => GatewayOpcode::InvalidSession,
      Self::Reconnect => GatewayOpcode::Reconnect,
      Self::Heartbeat => GatewayOpcode::Heartbeat,
      Self::HeartbeatAck => GatewayOpcode::HeartbeatACK,
    }
  }
}

struct GatewayEventVisitor;

impl<'de> serde::de::Visitor<'de> for GatewayEventVisitor {
  type Value = GatewayEvent;

  fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("gateway event")
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::MapAccess<'de>,
  {
    #[derive(Debug, Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
      Op,
      D,
      S,
      T,
    }

    let mut op: Option<GatewayOpcode> = None;
    let mut s: Option<u64> = None;
    let mut t: Option<EventType> = None;

    // event type must go first before sequence, then op, then data, that's how discord
    // sorts their gateway payload, otherwise we have to find a way there...
    while let Some(field) = map.next_key::<Field>()? {
      match field {
        Field::Op => {
          if op.is_some() {
            return Err(serde::de::Error::duplicate_field("op"));
          }
          op = Some(map.next_value()?);
        }
        Field::D => {
          let op =
            op.ok_or_else(|| serde::de::Error::custom("'d' field recieved but not opcode"))?;

          return match op {
            GatewayOpcode::Dispatch => {
              let s = s.ok_or_else(|| serde::de::Error::missing_field("s"))?;
              let t = t.ok_or_else(|| serde::de::Error::missing_field("t"))?;

              let data = map.next_value_seed(EventDeserializer(t))?;
              Ok(GatewayEvent::Dispatch(s, Box::new(data)))
            }
            GatewayOpcode::Heartbeat => {
              map.next_value()?;
              Ok(GatewayEvent::Heartbeat)
            }
            GatewayOpcode::Reconnect => {
              map.next_value()?;
              Ok(GatewayEvent::Reconnect)
            }
            GatewayOpcode::InvalidSession => Ok(GatewayEvent::InvalidSession(
              map.next_value_seed(ValueDeserializer::new())?,
            )),
            GatewayOpcode::Hello => Ok(GatewayEvent::Hello(
              map.next_value_seed(ValueDeserializer::new())?,
            )),
            GatewayOpcode::HeartbeatACK => {
              map.next_value()?;
              Ok(GatewayEvent::HeartbeatAck)
            }
            GatewayOpcode::Unknown => {
              Err(serde::de::Error::custom("unknown gateway opcode recieved"))
            }
            _ => {
              crate::internal::macros::log_warn!("(GatewayEvent) unknown opcode: {:?}", op);
              Err(serde::de::Error::custom(format!(
                "cannot deserialize event for {op:?}"
              )))
            }
          };
        }
        Field::S => {
          if s.is_some() {
            return Err(serde::de::Error::duplicate_field("t"));
          }
          s = map.next_value()?;
        }
        Field::T => {
          if t.is_some() {
            return Err(serde::de::Error::duplicate_field("t"));
          }
          t = map.next_value()?;
        }
      }
    }

    todo!()
  }
}

impl<'de> serde::Deserialize<'de> for GatewayEvent {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_map(GatewayEventVisitor)
  }
}

impl serde::Serialize for GatewayEvent {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeMap;

    let mut map = serializer.serialize_map(None)?;
    match self {
      Self::Dispatch(s, d) => {
        map.serialize_entry("t", &d.kind())?;
        map.serialize_entry("s", &s)?;
        map.serialize_entry("op", &self.opcode())?;
        map.serialize_entry("d", &d)?;
        return map.end();
      }
      _ => {}
    }
    map.serialize_entry("op", &self.opcode())?;
    match self {
      Self::Dispatch(..) => unreachable!(),
      Self::Hello(d) => {
        map.serialize_entry("d", &d)?;
      }
      Self::InvalidSession(d) => {
        map.serialize_entry("d", &d)?;
      }
      _ => {
        map.serialize_entry("d", &None::<()>)?;
      }
    }
    map.end()
  }
}
