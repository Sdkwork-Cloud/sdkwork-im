use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceSyncFeedEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "deviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    #[serde(rename = "syncSeq")]
    pub sync_seq: i64,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "originEventType")]
    pub origin_event_type: String,

    #[serde(rename = "actorId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "messageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    #[serde(rename = "messageSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_seq: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    #[serde(rename = "readSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_seq: Option<i64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
