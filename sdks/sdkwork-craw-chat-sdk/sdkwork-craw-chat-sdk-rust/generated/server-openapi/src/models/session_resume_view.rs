use serde::{Deserialize, Serialize};

use crate::models::{PresenceSnapshotView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SessionResumeView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "actorId")]
    pub actor_id: String,

    #[serde(rename = "actorKind")]
    pub actor_kind: String,

    #[serde(rename = "sessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "resumeRequired")]
    pub resume_required: bool,

    #[serde(rename = "resumeFromSyncSeq")]
    pub resume_from_sync_seq: i64,

    #[serde(rename = "latestSyncSeq")]
    pub latest_sync_seq: i64,

    #[serde(rename = "resumedAt")]
    pub resumed_at: String,

    pub presence: PresenceSnapshotView,
}
