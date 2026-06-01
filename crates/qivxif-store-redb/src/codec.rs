use crate::StoreResult;
use serde::{Serialize, de::DeserializeOwned};

pub fn encode<T: Serialize>(value: &T) -> StoreResult<Vec<u8>> {
    Ok(bincode::serialize(value)?)
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> StoreResult<T> {
    Ok(bincode::deserialize(bytes)?)
}
