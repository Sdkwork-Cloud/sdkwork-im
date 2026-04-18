use serde::{Deserialize, Serialize};

use crate::models::{ConversationAgentHandoffView, SummarySenderView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationSummaryView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageCount")]
    pub message_count: i64,

    #[serde(rename = "lastMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,

    #[serde(rename = "lastMessageSeq")]
    pub last_message_seq: i64,

    #[serde(rename = "lastSenderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sender_id: Option<String>,

    #[serde(rename = "lastSenderKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sender_kind: Option<String>,

    #[serde(rename = "lastSender")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sender: Option<SummarySenderView>,

    #[serde(rename = "lastSummary")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_summary: Option<String>,

    #[serde(rename = "lastMessageAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<String>,

    #[serde(rename = "agentHandoff")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_handoff: Option<ConversationAgentHandoffView>,
}
