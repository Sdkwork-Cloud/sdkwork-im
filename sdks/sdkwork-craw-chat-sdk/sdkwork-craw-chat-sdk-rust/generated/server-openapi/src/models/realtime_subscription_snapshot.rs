use serde::{Deserialize, Serialize};

use crate::models::{RealtimeSubscription};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeSubscriptionSnapshot {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    pub items: Vec<RealtimeSubscription>,

    #[serde(rename = "syncedAt")]
    pub synced_at: String,
}
