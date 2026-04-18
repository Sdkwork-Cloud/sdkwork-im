use serde::{Deserialize, Serialize};

use crate::models::{ChangeAgentHandoffStatusView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AgentHandoffStateView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    pub status: String,

    pub source: ChangeAgentHandoffStatusView,

    pub target: ChangeAgentHandoffStatusView,

    #[serde(rename = "handoffSessionId")]
    pub handoff_session_id: String,

    #[serde(rename = "handoffReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff_reason: Option<String>,

    #[serde(rename = "acceptedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,

    #[serde(rename = "acceptedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_by: Option<ChangeAgentHandoffStatusView>,

    #[serde(rename = "resolvedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,

    #[serde(rename = "resolvedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<ChangeAgentHandoffStatusView>,

    #[serde(rename = "closedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,

    #[serde(rename = "closedBy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<ChangeAgentHandoffStatusView>,
}
