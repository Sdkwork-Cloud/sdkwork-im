use crate::{build_json_rtc_signal, CrawChatSdkContext, JsonRtcSignalOptions};
use serde_json::Value;
use sdkwork_craw_chat_backend_sdk::{
  CreateRtcSessionRequest,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  PostRtcSignalRequest,
  RtcParticipantCredential,
  RtcRecordingArtifact,
  RtcSession,
  RtcSignalEvent,
  SdkworkError,
  UpdateRtcSessionRequest,
};

#[derive(Clone)]
pub struct CrawChatRtcModule {
  context: CrawChatSdkContext,
}

impl CrawChatRtcModule {
  pub(crate) fn new(context: CrawChatSdkContext) -> Self {
    Self { context }
  }

  pub async fn create(&self, body: CreateRtcSessionRequest) -> Result<RtcSession, SdkworkError> {
    self.context.backend_client().rtc().create_rtc_session(&body).await
  }

  pub async fn invite(
    &self,
    rtc_session_id: impl AsRef<str>,
    body: InviteRtcSessionRequest,
  ) -> Result<RtcSession, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .invite_rtc_session(&rtc_session_id, &body)
      .await
  }

  pub async fn accept(
    &self,
    rtc_session_id: impl AsRef<str>,
    body: UpdateRtcSessionRequest,
  ) -> Result<RtcSession, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .accept_rtc_session(&rtc_session_id, &body)
      .await
  }

  pub async fn reject(
    &self,
    rtc_session_id: impl AsRef<str>,
    body: UpdateRtcSessionRequest,
  ) -> Result<RtcSession, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .reject_rtc_session(&rtc_session_id, &body)
      .await
  }

  pub async fn end(
    &self,
    rtc_session_id: impl AsRef<str>,
    body: UpdateRtcSessionRequest,
  ) -> Result<RtcSession, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .end_rtc_session(&rtc_session_id, &body)
      .await
  }

  pub async fn post_signal(
    &self,
    rtc_session_id: impl AsRef<str>,
    body: PostRtcSignalRequest,
  ) -> Result<RtcSignalEvent, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .post_rtc_signal(&rtc_session_id, &body)
      .await
  }

  pub async fn post_json_signal(
    &self,
    rtc_session_id: impl AsRef<str>,
    signal_type: impl Into<String>,
    payload: &Value,
    options: JsonRtcSignalOptions,
  ) -> Result<RtcSignalEvent, SdkworkError> {
    self
      .post_signal(
        rtc_session_id,
        build_json_rtc_signal(signal_type, payload, options),
      )
      .await
  }

  pub async fn issue_participant_credential(
    &self,
    rtc_session_id: impl AsRef<str>,
    body: IssueRtcParticipantCredentialRequest,
  ) -> Result<RtcParticipantCredential, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .issue_rtc_participant_credential(&rtc_session_id, &body)
      .await
  }

  pub async fn get_recording_artifact(
    &self,
    rtc_session_id: impl AsRef<str>,
  ) -> Result<RtcRecordingArtifact, SdkworkError> {
    let rtc_session_id = rtc_session_id.as_ref().to_string();
    self
      .context
      .backend_client()
      .rtc()
      .get_rtc_recording_artifact(&rtc_session_id)
      .await
  }
}
