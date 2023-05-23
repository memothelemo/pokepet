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
