use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct SeedDeserializer<T: DeserializeOwned>(PhantomData<T>);

impl<T: DeserializeOwned> SeedDeserializer<T> {
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<'de, T: DeserializeOwned> serde::de::DeserializeSeed<'de> for SeedDeserializer<T> {
  type Value = T;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    T::deserialize(deserializer)
  }
}
