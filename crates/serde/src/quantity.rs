//! Maili quantity serialization and deserialization helpers.

use alloc::string::ToString;
use serde::{self, de, Deserialize, Deserializer};
use serde_json::Value;

/// Deserializes a u128 from a string or a number.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u128, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_u128().ok_or(de::Error::custom("Invalid number"))?,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

/// Serializes a u128 to a string.
pub fn serialize<S: serde::Serializer>(value: &u128, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
