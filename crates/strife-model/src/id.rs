use crate::misc::Timestamp;
use serde::{Deserialize, Serialize};

macro_rules! impl_ids {
  { $( $( #[$meta:meta] )* $vis:vis struct $name:ident; )* } => {$(
    $( #[$meta] )*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    #[serde(transparent)]
    $vis struct $name(#[serde(with = "snowflake")] pub u64);

    $( #[$meta] )*
    impl $name {
      #[inline]
      #[must_use]
      pub fn timestamp(&self) -> Timestamp {
        Timestamp::from_snowflake(self.0)
      }

      #[inline]
      #[must_use]
      #[allow(unused)]
      pub const fn get(&self) -> u64 {
        self.0
      }
    }

    $( #[$meta] )*
    impl<'a> From<&'a $name> for $name {
      fn from(value: &'a $name) -> Self {
        Self(value.0)
      }
    }

    $( #[$meta] )*
    impl From<u64> for $name {
      fn from(value: u64) -> Self {
        Self(value)
      }
    }

    $( #[$meta] )*
    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
      }
    }

    $( #[$meta] )*
    impl From<$name> for u64 {
      fn from(value: $name) -> Self {
        value.0
      }
    }
  )*};
}

impl_ids! {
  pub struct AttachmentId;
  pub struct ChannelId;
  pub struct ForumTagId;
  pub struct MessageId;

  pub struct EmojiId;
  pub struct CoverStickerId;
  pub struct StickerId;
  pub struct StickerPackId;
  pub struct StickerPackSkuId;
  pub struct StickerPackBannerAssetId;

  pub struct GuildId;
  pub struct RoleId;
  pub struct IntegrationId;
  pub struct IntegrationApplicationId;
  pub struct RoleSubscriptionSkuId;
  pub struct WebhookId;

  pub struct ApplicationId;
  pub struct GameSkuId;
  pub struct TeamId;
  pub struct UserId;

  #[cfg(test)]
  struct TestId;
}

mod snowflake {
  use serde::de::Error;
  use serde::de::Visitor as SerdeVisitor;
  use serde::Deserializer;
  use serde::Serializer;

  struct Visitor;

  impl<'de> SerdeVisitor<'de> for Visitor {
    type Value = u64;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "discord snowflake (integer or string)")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      if v.is_negative() {
        Err(Error::custom("negative ids are not allowed"))
      } else {
        self.visit_u64(v as u64)
      }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: Error,
    {
      v.parse().map_err(Error::custom)
    }
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    deserializer.deserialize_any(Visitor)
  }

  pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(value)
  }
}

#[cfg(test)]
mod tests {
  use super::TestId;
  use serde_test::{assert_de_tokens, assert_tokens, Token};
  use time::Month;

  #[test]
  fn serde() {
    let id = TestId(10);
    assert_tokens(&id, &[Token::Str("10")]);
    assert_de_tokens(&id, &[Token::I64(10)]);
  }

  #[test]
  fn timestamp() {
    let timestamp = TestId(613425648685547541).timestamp();
    assert_eq!(timestamp.year(), 2019);
    assert_eq!(timestamp.month(), Month::August);
    assert_eq!(timestamp.day(), 20);
    assert_eq!(timestamp.hour(), 17);
    assert_eq!(timestamp.minute(), 34);
    assert_eq!(timestamp.second(), 31);
  }
}
