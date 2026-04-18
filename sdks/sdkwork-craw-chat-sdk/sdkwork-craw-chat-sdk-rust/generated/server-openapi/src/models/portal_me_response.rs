use serde::{Deserialize, Serialize};

use crate::models::{PortalUserView, PortalWorkspaceView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalMeResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    pub user: PortalUserView,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<PortalWorkspaceView>,
}
