// Why discord
macro_rules! size_array {
  { pub struct $Name:ident($VisitorName:literal); } => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct $Name {
      index: u64,
      total: std::num::NonZeroU64,
    }

    impl $Name {
      // TODO: make will panic documentation
      pub const fn new(index: u64, total: u64) -> Self {
        assert!(index <= total, "number must not be greater than total");
        if let Some(total) = std::num::NonZeroU64::new(total) {
          Self { index, total }
        } else {
          panic!("total must be at least 1");
        }
      }

      pub const fn new_checked(index: u64, total: u64) -> Option<Self> {
        if index > total {
          return None;
        }

        if let Some(total) = std::num::NonZeroU64::new(total) {
          Some(Self { index, total })
        } else {
          None
        }
      }
    }

    crate::internal::macros::size_array!(impl base for $Name & $VisitorName);
  };
  { pub struct $Name:ident(array_like, $VisitorName:literal); } => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct $Name {
      index: u64,
      total: std::num::NonZeroU64,
    }

    impl $Name {
      // TODO: make will panic documentation
      pub const fn new(index: u64, total: u64) -> Self {
        assert!(index < total, "number must be less than total");
        if let Some(total) = std::num::NonZeroU64::new(total) {
          Self { index, total }
        } else {
          panic!("total must be at least 1");
        }
      }

      pub const fn new_checked(index: u64, total: u64) -> Option<Self> {
        if index >= total {
          return None;
        }

        if let Some(total) = std::num::NonZeroU64::new(total) {
          Some(Self { index, total })
        } else {
          None
        }
      }
    }

    $crate::internal::macros::size_array!(impl base for $Name & $VisitorName);
  };
  (impl base for $Name:ident & $VisitorName:literal) => {
    impl std::fmt::Display for $Name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
          .entry(&self.index)
          .entry(&self.total)
          .finish()
      }
    }

    impl From<$Name> for [u64; 2] {
      fn from(value: $Name) -> Self {
        [value.index(), value.total()]
      }
    }

    impl PartialOrd for $Name {
      fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.total != other.total {
          return None;
        }
        Some(self.total.cmp(&other.total))
      }
    }

    impl $Name {
      pub const fn index(self) -> u64 {
        self.index
      }

      pub const fn total(self) -> u64 {
        self.total.get()
      }
    }

    impl<'de> serde::de::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
          type Value = $Name;

          fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str($VisitorName)
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

            $Name::new_checked(id, total)
              .ok_or_else(|| serde::de::Error::custom("total must be at least 1 or id must not be greater than the size"))
          }
        }

        deserializer.deserialize_seq(Visitor)
      }
    }

    impl serde::Serialize for $Name {
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
  };
}
pub(crate) use size_array;

macro_rules! bitflags {
  (
    $( #[$outer:meta] )*
    $vis:vis struct $BitFlags:ident: $T:ty {
      $(
        $( #[ $inner:ident $( $args:tt )* ] )*
        const $Flag:ident = $value:expr;
      )*
    }
  ) => {
    ::bitflags::bitflags! {
      $( #[$outer] )*
      #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
      $vis struct $BitFlags: $T {
        $(
          $( #[ $inner $( $args )* ] )*
          const $Flag = $value;
        )*
      }
    }

    impl<'de> ::serde::Deserialize<'de> for $BitFlags {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        <$T>::deserialize(deserializer).map(Self::from_bits_truncate)
      }
    }

    impl ::serde::Serialize for $BitFlags {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        self.bits().serialize(serializer)
      }
    }
  };
}
pub(crate) use bitflags;

macro_rules! enum_int {
  (impl base for $Name:ident) => {
    impl $Name {
      pub const fn value(&self) -> u64 {
        *self as u64
      }
    }

    impl<'de> serde::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
          type Value = $Name;

          fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(concat!(stringify!($Name), " type (integer)"))
          }

          fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
          where
            E: serde::de::Error,
          {
            $Name::__internal_enum_int(value).ok_or_else(|| {
              crate::internal::macros::log_warn!(
                concat!("unknown ", stringify!($Name), " variant: {}"),
                value
              );
              serde::de::Error::custom(format!(
                concat!("unknown ", stringify!($Name), " variant: {}"),
                value
              ))
            })
          }
        }

        deserializer.deserialize_u64(Visitor)
      }
    }

    impl serde::Serialize for $Name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        serializer.serialize_u64(self.value())
      }
    }
  };
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident {
      $(
        $( #[$VariantMeta:meta] )*
        $Variant:ident $( = $Value:expr )?,
      )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    $vis enum $Name {
      $(
        $( #[$VariantMeta] )*
        $Variant $( = $Value )?,
      )*
      Unknown = !0,
    }

    impl $Name {
      #[doc(hidden)]
      fn __internal_enum_int(value: u64) -> Option<Self> {
        match value {
          $( _ if value == $Name::$Variant.value() => Some(Self::$Variant), )*
          _ => Some(Self::Unknown),
        }
      }
    }

    $crate::internal::macros::enum_int!(impl base for $Name);
  )*};
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident: strict {
      $( $Variant:ident $( = $Value:expr )?, )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    $vis enum $Name {
      $( $Variant $( = $Value )?, )*
    }

    impl $Name {
      #[doc(hidden)]
      fn __internal_enum_int(value: u64) -> Option<Self> {
        match value {
          $( _ if value == $Name::$Variant.value() => Some(Self::$Variant), )*
          _ => None,
        }
      }
    }

    $crate::internal::macros::enum_int!(impl base for $Name);
  )*};
}
pub(crate) use enum_int;

macro_rules! enum_string {
  (impl base for $Name:ident) => {
    impl<'de> serde::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
          type Value = $Name;

          fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(concat!(stringify!($Name), " enum"))
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: serde::de::Error,
          {
            $Name::__internal_enum_string(value).ok_or_else(|| {
              crate::internal::macros::log_warn!(
                concat!("unknown ", stringify!($Name), " variant: {:?}"),
                value
              );
              serde::de::Error::custom(format!(
                concat!("unknown ", stringify!($Name), " variant: {:?}"),
                value
              ))
            })
          }
        }

        deserializer.deserialize_str(Visitor)
      }
    }

    impl serde::Serialize for $Name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        serializer.collect_str(self)
      }
    }
  };
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident {
      $(
        $( #[$VariantMeta:meta] )*
        $Variant:ident = $Value:expr,
      )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum $Name {
      $(
        $( #[$VariantMeta] )*
        $Variant,
      )*
      Unknown(String),
    }

    impl $Name {
      #[doc(hidden)]
      fn __internal_enum_string(value: &str) -> Option<Self> {
        match value {
          $( $Value => Some(Self::$Variant), )*
          _ => Some(Self::Unknown(value.to_string())),
        }
      }
    }

    impl std::fmt::Display for $Name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $( Self::$Variant => f.write_str($Value), )*
          Self::Unknown(n) => n.fmt(f),
        }
      }
    }

    $crate::internal::macros::enum_string!(impl base for $Name);
  )*};
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident : strict {
      $(
        $( #[$VariantMeta:meta] )*
        $Variant:ident = $Value:expr,
      )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum $Name {
      $(
        $( #[$VariantMeta] )*
        $Variant,
      )*
    }

    impl $Name {
      #[doc(hidden)]
      fn __internal_enum_string(value: &str) -> Option<Self> {
        match value {
          $( $Value => Some(Self::$Variant), )*
          _ => None,
        }
      }
    }

    impl std::fmt::Display for $Name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $( Self::$Variant => f.write_str($Value), )*
        }
      }
    }

    $crate::internal::macros::enum_string!(impl base for $Name);
  )*};
}
pub(crate) use enum_string;

#[cfg(not(feature = "tracing"))]
macro_rules! log_warn {
  ( $( $token:tt )* ) => {};
}
#[cfg(feature = "tracing")]
macro_rules! log_warn {
  ( $( $token:tt )* ) => {};
}
pub(crate) use log_warn;
