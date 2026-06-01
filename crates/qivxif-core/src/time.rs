use crate::{CoreError, CoreResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServerTime(OffsetDateTime);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClientTime(OffsetDateTime);

impl ServerTime {
    pub fn now() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    pub fn parse(value: &str) -> CoreResult<Self> {
        OffsetDateTime::parse(value, &Rfc3339)
            .map(Self)
            .map_err(|_| CoreError::InvalidTime)
    }
}

impl ClientTime {
    pub fn now() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl std::fmt::Display for ServerTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.format(&Rfc3339).map_err(|_| std::fmt::Error)?)
    }
}

impl FromStr for ServerTime {
    type Err = CoreError;

    fn from_str(value: &str) -> CoreResult<Self> {
        Self::parse(value)
    }
}

impl Serialize for ServerTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ServerTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for ClientTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.0.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for ClientTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        OffsetDateTime::parse(&value, &Rfc3339)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}
