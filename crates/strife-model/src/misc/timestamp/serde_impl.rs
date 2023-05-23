use super::Timestamp;
use serde::{Serialize, Serializer};
use time::format_description::well_known::Rfc3339;

#[inline]
fn serialize_inner<S: Serializer>(value: &Timestamp, serializer: S) -> Result<S::Ok, S::Error> {
  value
    .0
    .format(&Rfc3339)
    .map_err(serde::ser::Error::custom)?
    .serialize(serializer)
}

pub mod secs {
  use crate::misc::Timestamp;
  use serde::{Deserializer, Serializer};

  struct Visitor;

  impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Timestamp;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str("Unix timestamp")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Timestamp::from_secs(v as i64).map_err(|v| {
        crate::internal::macros::log_warn!("{} is invalid Unix timestamp!", v);
        serde::de::Error::custom(v)
      })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Timestamp::parse(v).map_err(serde::de::Error::custom)
    }
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Timestamp, D::Error> {
    deserializer.deserialize_any(Visitor)
  }

  pub fn serialize<S: Serializer>(value: &Timestamp, serializer: S) -> Result<S::Ok, S::Error> {
    super::serialize_inner(value, serializer)
  }
}

pub mod millis {
  use crate::misc::Timestamp;
  use serde::{Deserializer, Serializer};

  struct Visitor;

  impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Timestamp;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str("Unix timestamp in milliseconds")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Timestamp::from_millis(v as i64).map_err(|v| {
        crate::internal::macros::log_warn!("{} is invalid Unix timestamp!", v);
        serde::de::Error::custom(v)
      })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Timestamp::parse(v).map_err(serde::de::Error::custom)
    }
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Timestamp, D::Error> {
    deserializer.deserialize_any(Visitor)
  }

  pub fn serialize<S: Serializer>(value: &Timestamp, serializer: S) -> Result<S::Ok, S::Error> {
    super::serialize_inner(value, serializer)
  }
}

pub mod millis_opt {
  use crate::misc::Timestamp;
  use serde::{Deserializer, Serializer};

  struct Visitor;

  impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Option<Timestamp>;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str("Unix timestamp in milliseconds")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Timestamp::from_millis(v as i64)
        .map_err(|v| {
          crate::internal::macros::log_warn!("{} is invalid Unix timestamp!", v);
          serde::de::Error::custom(v)
        })
        .map(Some)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Timestamp::parse(v)
        .map_err(serde::de::Error::custom)
        .map(Some)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(None)
    }
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<Option<Timestamp>, D::Error> {
    deserializer.deserialize_any(Visitor)
  }

  pub fn serialize<S: Serializer>(
    value: &Option<Timestamp>,
    serializer: S,
  ) -> Result<S::Ok, S::Error> {
    match value {
      Some(n) => super::serialize_inner(n, serializer),
      None => serializer.serialize_none(),
    }
  }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = Timestamp;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RFC 3339 timestamp")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Timestamp::parse(v).map_err(serde::de::Error::custom)
      }
    }

    deserializer.deserialize_any(Visitor)
  }
}

impl serde::Serialize for Timestamp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serialize_inner(self, serializer)
  }
}
