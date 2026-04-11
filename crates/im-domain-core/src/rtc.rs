use crate::message::Sender;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtcSessionState {
    Started,
    Accepted,
    Rejected,
    Ended,
}

impl RtcSessionState {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Ended => "ended",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSession {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
    pub initiator_id: String,
    pub provider_plugin_id: Option<String>,
    pub provider_session_id: Option<String>,
    pub access_endpoint: Option<String>,
    pub provider_region: Option<String>,
    pub state: RtcSessionState,
    pub signaling_stream_id: Option<String>,
    pub artifact_message_id: Option<String>,
    pub started_at: String,
    pub ended_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSignalEvent {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
    pub sender: Sender,
    pub signaling_stream_id: Option<String>,
    pub occurred_at: String,
}
