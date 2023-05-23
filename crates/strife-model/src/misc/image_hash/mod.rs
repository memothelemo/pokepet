use serde::{Deserialize, Serialize};
use thiserror::Error;

mod nibbles;
pub use nibbles::*;

#[cfg(test)]
mod tests;

const ANIMATED_HEADER: &str = "a_";

const HASH_LEN: usize = 32;
const HALF_HASH_LEN: usize = HASH_LEN / 2;
const BITS_IN_HALF_BYTE: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageHash {
  animated: bool,
  bytes: [u8; HALF_HASH_LEN],
}

#[derive(Debug, Error)]
pub enum ImageHashError {
  #[error("invalid hash size, expected {HASH_LEN}")]
  InputSize,
  #[error("invalid hash format")]
  Format,
}

impl ImageHash {
  #[must_use]
  pub const fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  #[must_use]
  pub const fn animated(&self) -> bool {
    self.animated
  }

  pub const fn nibbles(&self) -> ImageHashNibbles {
    ImageHashNibbles::new(*self)
  }
}

impl ImageHash {
  pub fn parse<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ImageHashError> {
    let bytes = bytes.as_ref();
    let animated = bytes.starts_with(ANIMATED_HEADER.as_bytes());
    let bytes = parse_inner(if animated {
      &bytes[ANIMATED_HEADER.len()..]
    } else {
      bytes
    })?;
    Ok(Self { animated, bytes })
  }
}

impl std::fmt::Display for ImageHash {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.animated {
      f.write_str(ANIMATED_HEADER)?;
    }
    for nibble in self.nibbles() {
      write!(f, "{}", nibble as char)?;
    }
    Ok(())
  }
}

impl std::fmt::Debug for ImageHash {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ImageHash(animated = {}, bytes = ", self.animated)?;

    for nibble in self.nibbles() {
      write!(f, "{}", nibble as char)?;
    }

    write!(f, ")")
  }
}

impl std::str::FromStr for ImageHash {
  type Err = ImageHashError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}

impl TryFrom<&[u8]> for ImageHash {
  type Error = ImageHashError;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Self::parse(value)
  }
}

impl TryFrom<&str> for ImageHash {
  type Error = ImageHashError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::parse(value)
  }
}

impl<'de> Deserialize<'de> for ImageHash {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = ImageHash;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("image hash")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        ImageHash::parse(v).map_err(serde::de::Error::custom)
      }
    }

    deserializer.deserialize_str(Visitor)
  }
}

impl Serialize for ImageHash {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_str(self)
  }
}

const fn parse_hex_byte(byte: u8) -> Option<u8> {
  match byte {
    b'0'..=b'9' => Some(byte - b'0'),
    b'a'..=b'f' => Some(byte - (b'a' - 10)),
    _ => None,
  }
}

fn parse_inner(raw: &[u8]) -> Result<[u8; 16], ImageHashError> {
  let mut bytes = [0u8; HALF_HASH_LEN];
  for (i, chunk) in raw.chunks(2).enumerate() {
    assert!(chunk.len() == 2);

    let left = parse_hex_byte(chunk[0]).ok_or(ImageHashError::Format)?;
    let right = parse_hex_byte(chunk[1]).ok_or(ImageHashError::Format)?;

    bytes[i] = (left << BITS_IN_HALF_BYTE) | right;
  }
  Ok(bytes)
}
