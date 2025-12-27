use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{de::DeserializeOwned, Serialize};

pub fn encode_cursor<T: Serialize>(payload: &T) -> anyhow::Result<String> {
    let json = serde_json::to_vec(payload)?;
    Ok(URL_SAFE_NO_PAD.encode(json))
}

pub fn decode_cursor<T: DeserializeOwned>(cursor: &str) -> anyhow::Result<T> {
    let bytes = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|e| anyhow::anyhow!("Invalid cursor (base64 decode): {e}"))?;

    let payload = serde_json::from_slice::<T>(&bytes)
        .map_err(|e| anyhow::anyhow!("Invalid cursor (json decode): {e}"))?;

    Ok(payload)
}