use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShardId {
  index: u64,
  total: NonZeroU64,
}

impl std::fmt::Display for ShardId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_list()
      .entry(&self.index)
      .entry(&self.total)
      .finish()
  }
}

impl From<ShardId> for [u64; 2] {
  fn from(value: ShardId) -> Self {
    [value.index(), value.total()]
  }
}

impl PartialOrd for ShardId {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    if self.total != other.total {
      return None;
    }
    Some(self.total.cmp(&other.total))
  }
}

impl ShardId {
  pub const DEFAULT: Self = Self::new(0, 1);

  // TODO: make will panic documentation
  pub const fn new(index: u64, total: u64) -> Self {
    assert!(index < total, "number must be less than total");
    if let Some(total) = NonZeroU64::new(total) {
      Self { index, total }
    } else {
      panic!("total must be at least 1");
    }
  }

  pub const fn new_checked(index: u64, total: u64) -> Option<Self> {
    if index >= total {
      return None;
    }

    if let Some(total) = NonZeroU64::new(total) {
      Some(Self { index, total })
    } else {
      None
    }
  }
}

impl ShardId {
  pub const fn index(self) -> u64 {
    self.index
  }

  pub const fn total(self) -> u64 {
    self.total.get()
  }
}

impl<'de> serde::de::Deserialize<'de> for ShardId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = ShardId;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("shard id")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
      where
        A: serde::de::SeqAccess<'de>,
      {
        let id: u64 = seq
          .next_element()?
          .ok_or_else(|| serde::de::Error::custom("missing first entry (id)"))?;

        let total: u64 = seq
          .next_element()?
          .ok_or_else(|| serde::de::Error::custom("missing second entry (total)"))?;

        if id >= total {
          return Err(serde::de::Error::custom("id must be less than total"));
        }

        ShardId::new_checked(id, total)
          .ok_or_else(|| serde::de::Error::custom("total must be at least 1"))
      }
    }

    deserializer.deserialize_seq(Visitor)
  }
}

impl serde::Serialize for ShardId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeSeq;

    let mut seq = serializer.serialize_seq(Some(2))?;
    seq.serialize_element(&self.index)?;
    seq.serialize_element(&self.total.get())?;

    seq.end()
  }
}
