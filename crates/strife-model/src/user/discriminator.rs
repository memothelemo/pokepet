use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Discriminator(u16);

const MAX_DIGITS: u32 = 4;
const MAX_MASK: u16 = 10_u16.pow(MAX_DIGITS) - 1;

impl Discriminator {
  // TODO: Make panic warning documentation
  pub const fn new(value: u16) -> Self {
    assert!(Self::is_valid(value), "number must be less than 4 digits",);
    Self(value)
  }

  pub const fn new_checked(value: u16) -> Option<Self> {
    if Self::is_valid(value) {
      Some(Self(value))
    } else {
      None
    }
  }

  pub const fn is_valid(value: u16) -> bool {
    value <= MAX_MASK
  }

  pub const fn value(self) -> u16 {
    self.0
  }
}

impl std::fmt::Display for Discriminator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}", self.0)
  }
}

impl<'de> Deserialize<'de> for Discriminator {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    use serde::de::Error as DeError;
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = Discriminator;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("user discriminator (integer or string)")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        self.visit_u16(v.parse::<u16>().map_err(DeError::custom)?)
      }

      fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
      where
        E: DeError,
      {
        Discriminator::new_checked(v).ok_or_else(|| {
          crate::internal::macros::log_warn!("invalid user discriminator value: {}", v);
          DeError::custom(format!("{:?} is too big for user discriminator", v))
        })
      }

      // serde_json probably called here
      fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        if v.is_negative() {
          crate::internal::macros::log_warn!(
            "negative number occurred when deserializing discriminator: {}",
            v
          );
          Err(DeError::custom("user discriminators must not be negative"))
        } else {
          self.visit_u64(v as u64)
        }
      }

      fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        if v >= u16::MAX as u64 {
          Err(DeError::custom(format!(
            "{:?} is too big for user discriminator",
            v
          )))
        } else {
          self.visit_u16(v as u16)
        }
      }
    }

    deserializer.deserialize_any(Visitor)
  }
}

impl Serialize for Discriminator {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl std::ops::Deref for Discriminator {
  type Target = u16;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::Discriminator;
  use serde_test::{assert_de_tokens, assert_tokens, Token};

  #[test]
  fn is_valid() {
    assert!(Discriminator::is_valid(0));
    assert!(Discriminator::is_valid(1));
    assert!(Discriminator::is_valid(12));
    assert!(Discriminator::is_valid(123));
    assert!(Discriminator::is_valid(1234));
    assert!(!Discriminator::is_valid(12345));
  }

  #[test]
  fn serde() {
    assert_tokens(&Discriminator::new(1000), &[Token::U16(1000)]);
    assert_de_tokens(&Discriminator::new(1000), &[Token::Str("1000")]);
  }

  #[test]
  fn new_checked() {
    assert!(Discriminator::new_checked(0).is_some());
    assert!(Discriminator::new_checked(1).is_some());
    assert!(Discriminator::new_checked(12).is_some());
    assert!(Discriminator::new_checked(123).is_some());
    assert!(Discriminator::new_checked(1234).is_some());
    assert!(Discriminator::new_checked(12345).is_none());
  }
}
