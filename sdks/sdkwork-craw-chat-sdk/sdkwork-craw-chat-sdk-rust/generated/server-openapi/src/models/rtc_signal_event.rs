use serde::{Deserialize, Serialize};

use crate::models::{Sender};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RtcSignalEvent {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "rtcSessionId")]
    pub rtc_session_id: String,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "rtcMode")]
    pub rtc_mode: String,

    #[serde(rename = "signalType")]
    pub signal_type: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    pub payload: String,

    pub sender: Sender,

    #[serde(rename = "signalingStreamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signaling_stream_id: Option<String>,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
