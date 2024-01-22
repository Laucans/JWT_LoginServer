use base64::prelude::*;
use serde::Deserialize;

pub fn base64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer).unwrap();
    BASE64_STANDARD
        .decode(s.as_bytes())
        .map_err(serde::de::Error::custom)
}
