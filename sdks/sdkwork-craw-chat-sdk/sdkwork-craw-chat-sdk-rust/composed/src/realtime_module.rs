use crate::{CrawChatSdkContext, QueryParams};
use sdkwork_craw_chat_backend_sdk::{
  AckRealtimeEventsRequest,
  RealtimeAckState,
  RealtimeEventWindow,
  RealtimeSubscriptionSnapshot,
  SdkworkError,
  SyncRealtimeSubscriptionsRequest,
};

#[derive(Clone)]
pub struct CrawChatRealtimeModule {
  context: CrawChatSdkContext,
}

impl CrawChatRealtimeModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn sync_subscriptions(
    &self,
    body: SyncRealtimeSubscriptionsRequest,
  ) -> Result<RealtimeSubscriptionSnapshot, SdkworkError> {
    self
      .context
      .backend_client()
      .realtime()
      .sync_realtime_subscriptions(&body)
      .await
  }

  pub async fn list_events(
    &self,
    params: Option<&QueryParams>,
  ) -> Result<RealtimeEventWindow, SdkworkError> {
    self.context.backend_client().realtime().list_realtime_events(params).await
  }

  pub async fn ack_events(
    &self,
    body: AckRealtimeEventsRequest,
  ) -> Result<RealtimeAckState, SdkworkError> {
    self.context.backend_client().realtime().ack_realtime_events(&body).await
  }
}
