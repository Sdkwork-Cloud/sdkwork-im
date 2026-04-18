use serde::{Deserialize, Serialize};

use crate::models::{PortalUserView, PortalWorkspaceView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalLoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,

    #[serde(rename = "expiresAt")]
    pub expires_at: i64,

    pub user: PortalUserView,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<PortalWorkspaceView>,
}
