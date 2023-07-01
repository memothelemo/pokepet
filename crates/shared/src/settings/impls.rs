// Implementations of super::Settings and the rest of structs
use super::{DatabaseConnection, DatabaseConnectionParts};
use doku::Document;

use serde::de::Error as SerdeError;
use serde::Deserialize;

// I avoid using #[serde(untagged)] because you don't know what
// is the actual cause of the error if one variant (supposed to be matched)
// fails to deserialize
impl<'de> Deserialize<'de> for DatabaseConnection {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct DatabaseConnectionVisitor;

    #[derive(Debug, Deserialize)]
    #[serde(field_identifier, rename_all = "snake_case")]
    enum Field {
      User,
      Password,
      Host,
      Port,
      Database,
      Uri,
    }

    impl<'de> serde::de::Visitor<'de> for DatabaseConnectionVisitor {
      type Value = DatabaseConnection;

      fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("database connection (uri or parts)")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: serde::de::MapAccess<'de>,
      {
        let mut user = None;
        let mut password = None;
        let mut host = None;
        let mut port = None;
        let mut database = None;

        let mut has_parts_like_field = false;
        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Uri => {
              if has_parts_like_field {
                return Err(SerdeError::custom(
                  "'uri' is found but it is deserializing other connection parts",
                ));
              }
              return Ok(DatabaseConnection::Uri(map.next_value()?));
            }
            Field::User => {
              if user.is_some() {
                return Err(SerdeError::duplicate_field("user"));
              }
              user = Some(map.next_value()?);
            }
            Field::Password => {
              if password.is_some() {
                return Err(SerdeError::duplicate_field("password"));
              }
              password = Some(map.next_value()?);
            }
            Field::Host => {
              if host.is_some() {
                return Err(SerdeError::duplicate_field("host"));
              }
              host = Some(map.next_value()?);
            }
            Field::Port => {
              if port.is_some() {
                return Err(SerdeError::duplicate_field("port"));
              }
              port = Some(map.next_value()?);
            }
            Field::Database => {
              if database.is_some() {
                return Err(SerdeError::duplicate_field("database"));
              }
              database = Some(map.next_value()?);
            }
          }
          has_parts_like_field = true;
        }

        let mut parts = DatabaseConnectionParts::default();
        if let Some(value) = user {
          parts.user = value;
        }
        if let Some(value) = password {
          parts.password = value;
        }
        if let Some(value) = host {
          parts.host = value;
        }
        if let Some(value) = port {
          parts.port = value;
        }
        if let Some(value) = database {
          parts.database = value;
        }

        Ok(DatabaseConnection::Parts(parts))
      }
    }

    deserializer.deserialize_map(DatabaseConnectionVisitor)
  }
}

impl std::fmt::Debug for DatabaseConnection {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Parts(n) => std::fmt::Debug::fmt(&n, f),
      Self::Uri(..) => write!(f, "Uri(***)"),
    }
  }
}

// Not a bulletproof solution for implementing untagged variants with doku toml
impl Document for DatabaseConnection {
  fn ty() -> doku::Type {
    use doku::{Field, Fields, Type, TypeKind};

    let mut fields = match DatabaseConnectionParts::ty().kind {
      TypeKind::Struct {
        fields: Fields::Named { fields },
        ..
      } => fields,
      _ => unreachable!(),
    };

    fields.iter_mut().next().unwrap().1.ty.comment = Some("or");
    fields.insert(
      0,
      (
        "uri",
        Field {
          ty: Type {
            comment: None,
            example: Some(doku::Example::Simple("postgres://postgres@localhost")),
            metas: Default::default(),
            tag: None,
            serializable: true,
            deserializable: true,
            kind: <String as Document>::ty().kind,
          },
          flattened: false,
          aliases: &[],
        },
      ),
    );

    // Getting fields from Uri and Parts variant
    Type::from(TypeKind::Struct {
      fields: Fields::Named { fields },
      transparent: false,
    })
  }
}
