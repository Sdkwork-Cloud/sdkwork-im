use std::sync::Arc;

use crate::api::base::{QueryParams};
use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{AckRealtimeEventsRequest, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest};

#[derive(Clone)]
pub struct RealtimeApi {
    client: Arc<SdkworkHttpClient>,
}

impl RealtimeApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Replace realtime subscriptions for the current device
    pub async fn sync_realtime_subscriptions(&self, body: &SyncRealtimeSubscriptionsRequest) -> Result<RealtimeSubscriptionSnapshot, SdkworkError> {
        let path = backend_path(&"/realtime/subscriptions/sync".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Pull realtime events for the current device
    pub async fn list_realtime_events(&self, query: Option<&QueryParams>) -> Result<RealtimeEventWindow, SdkworkError> {
        let path = backend_path(&"/realtime/events".to_string());
        self.client.get(&path, query, None).await
    }

    /// Ack realtime events for the current device
    pub async fn ack_realtime_events(&self, body: &AckRealtimeEventsRequest) -> Result<RealtimeAckState, SdkworkError> {
        let path = backend_path(&"/realtime/events/ack".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }
}
