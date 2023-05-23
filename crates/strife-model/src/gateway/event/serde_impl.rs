use super::GatewayEvent;
use serde::{de::IgnoredAny, Deserialize};

use crate::gateway::{EventDeserializer, EventType, GatewayOpcode};
use crate::internal::deser::SeedDeserializer;

struct Visitor;

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
  Op,
  D,
  S,
  T,
}

impl<'de> serde::de::Visitor<'de> for Visitor {
  type Value = GatewayEvent;

  fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("gateway event")
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::MapAccess<'de>,
  {
    let mut op: Option<GatewayOpcode> = None;
    let mut s: Option<u64> = None;
    let mut t: Option<EventType> = None;

    while let Some(field) = map.next_key::<Field>()? {
      match field {
        Field::D => {
          let op =
            op.ok_or_else(|| serde::de::Error::custom("'d' field recieved but not opcode"))?;

          match op {
            GatewayOpcode::Dispatch => {
              let s = s.ok_or_else(|| serde::de::Error::missing_field("s"))?;
              let t = t.ok_or_else(|| serde::de::Error::missing_field("t"))?;

              let data = map.next_value_seed(EventDeserializer(t))?;
              return Ok(GatewayEvent::Dispatch(s, Box::new(data)));
            }
            GatewayOpcode::InvalidSession => {
              return Ok(GatewayEvent::InvalidSession(
                map.next_value_seed(SeedDeserializer::new())?,
              ))
            }
            GatewayOpcode::Hello => {
              return Ok(GatewayEvent::Hello(
                map.next_value_seed(SeedDeserializer::new())?,
              ));
            }
            _ => {
              map.next_value_seed(SeedDeserializer::<IgnoredAny>::new())?;
            }
          }
        }
        Field::Op => {
          if op.is_some() {
            return Err(serde::de::Error::duplicate_field("op"));
          }
          op = Some(map.next_value()?);
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

    let op = op.ok_or_else(|| serde::de::Error::missing_field("op"))?;
    match op {
      GatewayOpcode::Reconnect => Ok(GatewayEvent::Reconnect),
      GatewayOpcode::Heartbeat => Ok(GatewayEvent::Heartbeat),
      GatewayOpcode::HeartbeatAck => Ok(GatewayEvent::HeartbeatAck),
      GatewayOpcode::Dispatch | GatewayOpcode::InvalidSession | GatewayOpcode::Hello => {
        Err(serde::de::Error::missing_field("d"))
      }
      _ => Err(serde::de::Error::custom(format!(
        "cannot deserialize event for {op:?}"
      ))),
    }
  }
}

impl<'de> serde::Deserialize<'de> for GatewayEvent {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_map(Visitor)
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
