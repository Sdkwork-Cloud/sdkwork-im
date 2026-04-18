use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeAckState {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "ackedThroughSeq")]
    pub acked_through_seq: i64,

    #[serde(rename = "trimmedThroughSeq")]
    pub trimmed_through_seq: i64,

    #[serde(rename = "retainedEventCount")]
    pub retained_event_count: i64,

    #[serde(rename = "ackedAt")]
    pub acked_at: String,
}
