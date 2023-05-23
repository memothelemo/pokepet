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

macro_rules! enum_string {
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident {
      $(
        $( #[$VariantMeta:meta] )*
        $Variant:ident = $Discriminator:expr,
      )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    $vis enum $Name {
      $(
        $( #[$VariantMeta] )*
        $Variant,
      )*
      Unknown(String),
    }

    impl ::std::fmt::Display for $Name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $( Self::$Variant => f.write_str($Discriminator), )*
          Self::Unknown(n) => n.fmt(f),
        }
      }
    }

    impl<'de> ::serde::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
          type Value = $Name;

          fn expecting(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, concat!(stringify!($Name), " string type"))
          }

          fn visit_str<E>(self, value: &str) -> ::core::result::Result<Self::Value, E>
          where
            E: ::serde::de::Error,
          {
            match value {
              $( $Discriminator => Ok($Name::$Variant), )*
              _ => {
                crate::internal::macros::log_warn!(
                  concat!("unknown ", stringify!($Name), " variant: {}"),
                  value,
                );
                Ok($Name::Unknown(value.to_string()))
              }
            }
          }
        }

        deserializer.deserialize_str(Visitor)
      }
    }

    impl ::serde::Serialize for $Name {
      fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        serializer.collect_str(self)
      }
    }
  )*};
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident : strict {
      $(
        $( #[$VariantMeta:meta] )*
        $Variant:ident = $Discriminator:expr,
      )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    $vis enum $Name {
      $(
        $( #[$VariantMeta] )*
        $Variant,
      )*
    }

    impl ::std::fmt::Display for $Name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $( Self::$Variant => f.write_str($Discriminator), )*
        }
      }
    }

    impl<'de> ::serde::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
          type Value = $Name;

          fn expecting(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, concat!(stringify!($Name), " string type"))
          }

          fn visit_str<E>(self, value: &str) -> ::core::result::Result<Self::Value, E>
          where
            E: ::serde::de::Error,
          {
            match value {
              $( $Discriminator => Ok($Name::$Variant), )*
              _ => {
                crate::internal::macros::log_warn!(
                  concat!("unknown ", stringify!($Name), " variant: {}"),
                  value,
                );
                Err(::serde::de::Error::custom(format!(
                  concat!("unknown ", stringify!($Name), " variant: {}"),
                  value
                )))
              }
            }
          }
        }

        deserializer.deserialize_str(Visitor)
      }
    }

    impl ::serde::Serialize for $Name {
      fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        serializer.collect_str(self)
      }
    }
  )*};
}
pub(crate) use enum_string;

macro_rules! enum_repr {
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident($Repr:ty): strict {
      $(
        $( #[$VariantMeta:meta] )*
        $Variant:ident $( = $Value:expr )?,
      )*
    }
  )*} => {$(
    $( #[$EnumMeta] )*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr($Repr)]
    #[non_exhaustive]
    $vis enum $Name {
      $(
        $( #[$VariantMeta] )*
        $Variant $( = $Value )?,
      )*
    }

    impl<'de> ::serde::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        #[allow(non_camel_case_types)]
        struct discriminators;
        #[allow(non_upper_case_globals)]
        impl discriminators {
          $( const $Variant: $Repr = $Name::$Variant as $Repr; )*
        }

        let value = <$Repr as ::serde::Deserialize>::deserialize(deserializer)?;
        match value {
          $( discriminators::$Variant => Ok(Self::$Variant), )*
          _ => {
            $crate::internal::macros::log_warn!(
              concat!("unknown ", stringify!($Name), " variant: {}"),
              value
            );
            ::core::result::Result::Err(serde::de::Error::custom(format!(
              concat!("unknown ", stringify!($Name), " variant: {}"),
              value
            )))
          }
        }
      }
    }

    impl ::serde::Serialize for $Name {
      fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        (*self as $Repr).serialize(serializer)
      }
    }
  )*};
  {$(
    $( #[$EnumMeta:meta] )*
    $vis:vis enum $Name:ident($Repr:ty) {
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
        $Variant,
      )*
      Unknown($Repr),
    }

    impl<'de> ::serde::Deserialize<'de> for $Name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        #[allow(non_camel_case_types)]
        #[repr($Repr)]
        enum repr_enum {
          $( $Variant $( = $Value )?, )*
        }

        #[allow(non_camel_case_types)]
        struct discriminators;

        #[allow(non_upper_case_globals)]
        impl discriminators {
          $( const $Variant: $Repr = repr_enum::$Variant as $Repr; )*
        }

        let value = <$Repr as serde::Deserialize>::deserialize(deserializer)?;
        match value {
          $( discriminators::$Variant => Ok(Self::$Variant), )*
          _ => {
            $crate::internal::macros::log_warn!(
              concat!("unknown ", stringify!($Name), " variant: {}"),
              value
            );
            Ok(Self::Unknown(value))
          },
        }
      }
    }

    impl serde::Serialize for $Name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
      {
        #[allow(non_camel_case_types)]
        #[repr($Repr)]
        enum discriminators {
          $( $Variant $( = $Value )?, )*
        }

        match self {
          $( Self::$Variant => discriminators::$Variant as $Repr, )*
          Self::Unknown(n) => *n,
        }
        .serialize(serializer)
      }
    }
  )*};
}
pub(crate) use enum_repr;

#[cfg(not(feature = "tracing"))]
macro_rules! log_warn {
  ( $( $token:tt )* ) => {};
}
#[cfg(feature = "tracing")]
macro_rules! log_warn {
  ( $( $token:tt )* ) => {};
}
pub(crate) use log_warn;
