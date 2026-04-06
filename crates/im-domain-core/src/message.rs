use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::media::MediaResource;

pub type MessageAttributes = BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Standard,
    System,
    Signal,
}

impl MessageType {
    pub fn as_wire_value(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::System => "system",
            Self::Signal => "signal",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub id: String,
    pub kind: String,
    pub member_id: Option<String>,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: MessageAttributes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub sender: Sender,
    pub message_type: MessageType,
    pub delivery_mode: String,
    pub client_msg_id: Option<String>,
    pub stream_session_id: Option<String>,
    pub rtc_session_id: Option<String>,
    pub body: MessageBody,
    pub attributes: MessageAttributes,
    pub metadata: MessageAttributes,
    pub occurred_at: String,
    pub committed_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEdited {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub body: MessageBody,
    pub editor: Sender,
    pub edited_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRecalled {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub recalled_by: Sender,
    pub recalled_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageBody {
    pub summary: Option<String>,
    pub parts: Vec<ContentPart>,
    pub render_hints: MessageAttributes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPart {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPart {
    pub schema_ref: String,
    pub encoding: String,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaPart {
    pub media_asset_id: String,
    pub resource: Option<MediaResource>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalPart {
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamRefPart {
    pub stream_id: String,
    pub stream_type: String,
    pub state: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ContentPart {
    Text(TextPart),
    Data(DataPart),
    Media(MediaPart),
    Signal(SignalPart),
    StreamRef(StreamRefPart),
}

impl ContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(TextPart { text: text.into() })
    }

    pub fn media(part: MediaPart) -> Self {
        Self::Media(part)
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Data(_) => "data",
            Self::Media(_) => "media",
            Self::Signal(_) => "signal",
            Self::StreamRef(_) => "stream_ref",
        }
    }

    pub fn as_media(&self) -> Option<&MediaPart> {
        match self {
            Self::Media(part) => Some(part),
            _ => None,
        }
    }
}
