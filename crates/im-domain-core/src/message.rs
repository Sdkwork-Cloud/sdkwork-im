use std::collections::{BTreeMap, BTreeSet, HashMap};

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
pub struct MessageReactionAdded {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub reaction_key: String,
    pub reacted_by: Sender,
    pub reacted_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageReactionRemoved {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub reaction_key: String,
    pub removed_by: Sender,
    pub removed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePinned {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub pinned_by: Sender,
    pub pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageUnpinned {
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub unpinned_by: Sender,
    pub unpinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredMessagePin {
    pub pinned_by: Sender,
    pub pinned_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredMessage {
    pub message: Message,
    pub recalled: bool,
    pub reactions: BTreeMap<String, BTreeSet<String>>,
    pub pin: Option<StoredMessagePin>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConversationMessageLog {
    high_watermark: u64,
    messages: HashMap<String, StoredMessage>,
}

impl ConversationMessageLog {
    pub fn high_watermark(&self) -> u64 {
        self.high_watermark
    }

    pub fn next_message_seq(&mut self) -> u64 {
        self.high_watermark += 1;
        self.high_watermark
    }

    pub fn unread_count_since(&self, read_seq: u64) -> u64 {
        self.high_watermark.saturating_sub(read_seq)
    }

    pub fn message(&self, message_id: &str) -> Option<&StoredMessage> {
        self.messages.get(message_id)
    }

    pub fn messages_in_order(&self) -> Vec<StoredMessage> {
        let mut items: Vec<_> = self.messages.values().cloned().collect();
        items.sort_by_key(|stored| stored.message.message_seq);
        items
    }

    pub fn store_posted(&mut self, message: Message) {
        self.high_watermark = self.high_watermark.max(message.message_seq);
        self.messages.insert(
            message.message_id.clone(),
            StoredMessage {
                message,
                recalled: false,
                reactions: BTreeMap::new(),
                pin: None,
            },
        );
    }

    pub fn apply_edited(&mut self, edited: &MessageEdited) -> Option<&StoredMessage> {
        let stored = self.messages.get_mut(edited.message_id.as_str())?;
        stored.message.body = edited.body.clone();
        stored.message.committed_at = Some(edited.edited_at.clone());
        Some(stored)
    }

    pub fn apply_recalled(&mut self, recalled: &MessageRecalled) -> Option<&StoredMessage> {
        let stored = self.messages.get_mut(recalled.message_id.as_str())?;
        stored.recalled = true;
        stored.message.body.summary = Some("[recalled]".into());
        stored.message.committed_at = Some(recalled.recalled_at.clone());
        Some(stored)
    }

    pub fn apply_reaction_added(&mut self, added: &MessageReactionAdded) -> Option<bool> {
        let stored = self.messages.get_mut(added.message_id.as_str())?;
        let actor_ids = stored
            .reactions
            .entry(added.reaction_key.clone())
            .or_insert_with(BTreeSet::new);
        Some(actor_ids.insert(added.reacted_by.id.clone()))
    }

    pub fn apply_reaction_removed(&mut self, removed: &MessageReactionRemoved) -> Option<bool> {
        let stored = self.messages.get_mut(removed.message_id.as_str())?;
        let Some(actor_ids) = stored.reactions.get_mut(removed.reaction_key.as_str()) else {
            return Some(false);
        };
        let changed = actor_ids.remove(removed.removed_by.id.as_str());
        if actor_ids.is_empty() {
            stored.reactions.remove(removed.reaction_key.as_str());
        }
        Some(changed)
    }

    pub fn apply_pinned(&mut self, pinned: &MessagePinned) -> Option<bool> {
        let stored = self.messages.get_mut(pinned.message_id.as_str())?;
        if stored.pin.is_some() {
            return Some(false);
        }
        stored.pin = Some(StoredMessagePin {
            pinned_by: pinned.pinned_by.clone(),
            pinned_at: pinned.pinned_at.clone(),
        });
        Some(true)
    }

    pub fn apply_unpinned(&mut self, unpinned: &MessageUnpinned) -> Option<bool> {
        let stored = self.messages.get_mut(unpinned.message_id.as_str())?;
        Some(stored.pin.take().is_some())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageLocatorIndex {
    conversation_ids: HashMap<String, String>,
}

impl MessageLocatorIndex {
    pub fn register(&mut self, tenant_id: &str, message_id: &str, conversation_id: &str) {
        self.conversation_ids.insert(
            message_locator_key(tenant_id, message_id),
            conversation_id.to_owned(),
        );
    }

    pub fn register_message(&mut self, message: &Message) {
        self.register(
            message.tenant_id.as_str(),
            message.message_id.as_str(),
            message.conversation_id.as_str(),
        );
    }

    pub fn conversation_id(&self, tenant_id: &str, message_id: &str) -> Option<&str> {
        self.conversation_ids
            .get(message_locator_key(tenant_id, message_id).as_str())
            .map(String::as_str)
    }
}

fn message_locator_key(tenant_id: &str, message_id: &str) -> String {
    format!("{tenant_id}:{message_id}")
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
// Keeping the content variants inline preserves the public message contract and
// serde shape across services; boxing only media would add cross-crate API churn
// for a layout optimization that is not release-blocking here.
#[allow(clippy::large_enum_variant)]
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
