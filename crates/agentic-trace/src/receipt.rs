use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub event_id: Uuid,
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
    pub parent_event_id: Option<Uuid>,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_id: Uuid,
    pub event_id: Uuid,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    ToolCall, ModelInference, ArtifactWrite, Approval, PolicyDecision, Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,
    pub public_key: String,
    pub value: String,
}
