use thiserror::Error;
use time::error::Parse as ParseError;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const DISCORD_EPOCH_MS: i64 = 1420070400000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(OffsetDateTime);

#[derive(Debug, Error)]
pub enum TimestampError {
  #[error("failed to parse timestamp: {0}")]
  Parse(ParseError),
  #[error("invalid given unix timestamp")]
  InvalidUnixTimestamp,
}

impl Timestamp {
  #[must_use]
  pub fn now() -> Self {
    Self(OffsetDateTime::now_utc())
  }

  pub(crate) fn from_snowflake(id: u64) -> Self {
    let timestamp_ms = (id >> 22) as i64 + DISCORD_EPOCH_MS;
    let ns = time::Duration::milliseconds(timestamp_ms).whole_nanoseconds();
    match OffsetDateTime::from_unix_timestamp_nanos(ns) {
      Ok(n) => Self(n),
      Err(..) => unreachable!(),
    }
  }

  #[inline]
  pub const fn from_inner(dt: OffsetDateTime) -> Self {
    Self(dt)
  }

  pub fn from_unix_timestamp(secs: i64) -> Result<Self, TimestampError> {
    OffsetDateTime::from_unix_timestamp(secs)
      .map_err(|_| TimestampError::InvalidUnixTimestamp)
      .map(Self)
  }

  pub fn parse(input: &str) -> Result<Self, TimestampError> {
    OffsetDateTime::parse(input, &Rfc3339)
      .map_err(TimestampError::Parse)
      .map(Self)
  }
}

impl Timestamp {
  #[must_use]
  pub const fn unix_timestamp(&self) -> i64 {
    self.0.unix_timestamp()
  }
}

impl std::fmt::Display for Timestamp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
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

    deserializer.deserialize_str(Visitor)
  }
}

impl serde::Serialize for Timestamp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self
      .0
      .format(&Rfc3339)
      .map_err(serde::ser::Error::custom)?
      .serialize(serializer)
  }
}

impl From<OffsetDateTime> for Timestamp {
  fn from(value: OffsetDateTime) -> Self {
    Self(value)
  }
}

impl std::ops::Deref for Timestamp {
  type Target = OffsetDateTime;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::Timestamp;
  use serde_test::{assert_tokens, Token};
  use time::Month;

  const SAMPLE_TIMESTAMP: &str = "2023-05-22T05:39:02Z";

  #[test]
  fn parse() {
    let timestamp = Timestamp::parse(SAMPLE_TIMESTAMP).unwrap();
    assert_eq!(timestamp.year(), 2023);
    assert_eq!(timestamp.month(), Month::May);
    assert_eq!(timestamp.day(), 22);
    assert_eq!(timestamp.hour(), 05);
    assert_eq!(timestamp.minute(), 39);
    assert_eq!(timestamp.second(), 02);
  }

  #[test]
  fn serde() {
    let timestamp = Timestamp::parse(SAMPLE_TIMESTAMP).unwrap();
    assert_tokens(&timestamp, &[Token::Str(SAMPLE_TIMESTAMP)]);
  }
}
