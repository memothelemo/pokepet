use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidSession {
  pub can_resume: bool,
}

impl<'de> Deserialize<'de> for InvalidSession {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(Self {
      can_resume: bool::deserialize(deserializer)?,
    })
  }
}

impl Serialize for InvalidSession {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.can_resume.serialize(serializer)
  }
}
