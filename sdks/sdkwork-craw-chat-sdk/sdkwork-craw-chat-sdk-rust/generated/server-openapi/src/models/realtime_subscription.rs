use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RealtimeSubscription {
    #[serde(rename = "scopeType")]
    pub scope_type: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "eventTypes")]
    pub event_types: Vec<String>,

    #[serde(rename = "subscribedAt")]
    pub subscribed_at: String,
}
