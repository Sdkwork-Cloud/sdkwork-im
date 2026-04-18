use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceSyncFeedEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "syncSeq")]
    pub sync_seq: i64,

    #[serde(rename = "originEventId")]
    pub origin_event_id: String,

    #[serde(rename = "originEventType")]
    pub origin_event_type: String,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "messageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    #[serde(rename = "messageSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_seq: Option<i64>,

    #[serde(rename = "memberId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,

    #[serde(rename = "readSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_seq: Option<i64>,

    #[serde(rename = "lastReadMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_message_id: Option<String>,

    #[serde(rename = "actorId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,

    #[serde(rename = "actorKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_kind: Option<String>,

    #[serde(rename = "actorDeviceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_device_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "payloadSchema")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_schema: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
