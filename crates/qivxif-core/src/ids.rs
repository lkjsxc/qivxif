use crate::{CoreError, CoreResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};
use uuid::Uuid;

macro_rules! id_type {
    ($name:ident, $prefix:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub const PREFIX: &'static str = $prefix;

            pub fn generate() -> Self {
                Self(format!(
                    "{}_{:032x}",
                    Self::PREFIX,
                    Uuid::new_v4().as_u128()
                ))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl FromStr for $name {
            type Err = CoreError;

            fn from_str(value: &str) -> CoreResult<Self> {
                validate_id(value, Self::PREFIX)?;
                Ok(Self(value.to_owned()))
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

id_type!(UserId, "usr");
id_type!(ActorId, "act");
id_type!(SessionId, "ses");
id_type!(NodeId, "nod");
id_type!(EdgeId, "edg");
id_type!(OperationId, "op");
id_type!(CommitGroupId, "cg");
id_type!(BlobHash, "blb");
id_type!(ChunkHash, "chk");
id_type!(TextDocId, "txt");
id_type!(CursorId, "cur");
id_type!(RequestId, "req");

fn validate_id(value: &str, prefix: &str) -> CoreResult<()> {
    let Some(body) = value.strip_prefix(&format!("{prefix}_")) else {
        return Err(CoreError::InvalidId);
    };
    if body.len() != 32
        || !body
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() && !ch.is_uppercase())
    {
        return Err(CoreError::InvalidId);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_round_trip() {
        let id = NodeId::generate();
        assert_eq!(id.to_string().parse::<NodeId>().unwrap(), id);
    }

    #[test]
    fn rejects_wrong_prefix() {
        assert!(
            "usr_0123456789abcdef0123456789abcdef"
                .parse::<NodeId>()
                .is_err()
        );
    }

    #[test]
    fn rejects_malformed_body() {
        assert!("nod_0123".parse::<NodeId>().is_err());
        assert!(
            "nod_0123456789abcdef0123456789abcdeF"
                .parse::<NodeId>()
                .is_err()
        );
    }
}
