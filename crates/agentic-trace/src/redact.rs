use serde_json::Value;
use sha2::{Sha256, Digest};

pub fn redact_field(value: &Value, reason: &str) -> Value {
    let hash = Sha256::digest(serde_json::to_string(value).unwrap().as_bytes());
    serde_json::json!({"redacted": true, "original_hash": format!("sha256:{:x}", hash), "reason": reason})
}
