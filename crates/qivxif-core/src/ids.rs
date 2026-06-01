use crate::{CoreError, CoreResult};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

const RANDOM_ID_BYTES: usize = 32;
const RANDOM_ID_HEX_LEN: usize = RANDOM_ID_BYTES * 2;

macro_rules! id_type {
    ($name:ident, $prefix:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub const PREFIX: &'static str = $prefix;

            pub fn generate() -> Self {
                Self(format!("{}_{}", Self::PREFIX, random_hex_32()))
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
id_type!(EventId, "evt");
id_type!(OperationId, "op");
id_type!(CommitGroupId, "cg");
id_type!(BlobHash, "blb");
id_type!(ChunkHash, "chk");
id_type!(TextDocId, "txt");
id_type!(CursorId, "cur");
id_type!(RequestId, "req");

fn validate_id(value: &str, prefix: &str) -> CoreResult<()> {
    if value.matches('_').count() != 1 {
        return Err(CoreError::InvalidId);
    }
    let Some(body) = value.strip_prefix(&format!("{prefix}_")) else {
        return Err(CoreError::InvalidId);
    };
    if body.len() != RANDOM_ID_HEX_LEN
        || !body
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() && !ch.is_uppercase())
    {
        return Err(CoreError::InvalidId);
    }
    Ok(())
}

fn random_hex_32() -> String {
    let mut bytes = [0u8; RANDOM_ID_BYTES];
    OsRng.fill_bytes(&mut bytes);
    encode_lower_hex(&bytes)
}

fn encode_lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn id_round_trip() {
        let id = NodeId::generate();
        assert_eq!(id.to_string().parse::<NodeId>().unwrap(), id);
    }

    #[test]
    fn generated_body_has_sixty_four_hex_chars() {
        let id = EventId::generate().to_string();
        let (_, body) = id.split_once('_').unwrap();
        assert_eq!(body.len(), 64);
        assert!(body.chars().all(|ch| ch.is_ascii_hexdigit()));
        assert!(!body.chars().any(|ch| ch.is_uppercase()));
    }

    #[test]
    fn rejects_wrong_prefix() {
        assert!(
            "usr_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .parse::<NodeId>()
                .is_err()
        );
    }

    #[test]
    fn rejects_malformed_body() {
        assert!("nod_0123".parse::<NodeId>().is_err());
        assert!(
            "nod_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeF"
                .parse::<NodeId>()
                .is_err()
        );
        assert!(
            "nod_0123456789abcdef0123456789abcdef"
                .parse::<NodeId>()
                .is_err()
        );
        assert!(
            "nod_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef_extra"
                .parse::<NodeId>()
                .is_err()
        );
    }

    #[test]
    fn generated_ids_are_unique_in_sample() {
        let ids: BTreeSet<_> = (0..1000).map(|_| NodeId::generate()).collect();
        assert_eq!(ids.len(), 1000);
    }

    #[test]
    fn generated_id_has_no_time_or_uuid_shape() {
        let id = EventId::generate().to_string();
        let (_, body) = id.split_once('_').unwrap();
        assert_eq!(id.matches('_').count(), 1);
        assert_eq!(body.len(), 64);
        assert!(!body.contains('-'));
    }
}
