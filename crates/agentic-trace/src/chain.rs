use crate::canonicalize::canonicalize;
use crate::receipt::TraceEvent;
use sha2::{Digest, Sha256};

pub fn compute_hash(event: &TraceEvent, prev_hash: &str) -> String {
    let canonical = canonicalize(&serde_json::to_value(event).unwrap());
    let input = format!("{}{}{}", canonical, prev_hash, event.timestamp.to_rfc3339());
    format!("sha256:{:x}", Sha256::digest(input.as_bytes()))
}
