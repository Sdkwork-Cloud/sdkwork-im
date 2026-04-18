use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeEvent {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "realtimeSeq")]
    pub realtime_seq: i64,

    #[serde(rename = "scopeType")]
    pub scope_type: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "eventType")]
    pub event_type: String,

    #[serde(rename = "deliveryClass")]
    pub delivery_class: String,

    pub payload: String,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
