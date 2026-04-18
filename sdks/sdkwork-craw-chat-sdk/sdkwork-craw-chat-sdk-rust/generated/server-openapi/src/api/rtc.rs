use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, UpdateRtcSessionRequest};

#[derive(Clone)]
pub struct RtcApi {
    client: Arc<SdkworkHttpClient>,
}

impl RtcApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Create an RTC session
    pub async fn create_rtc_session(&self, body: &CreateRtcSessionRequest) -> Result<RtcSession, SdkworkError> {
        let path = backend_path(&"/rtc/sessions".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Invite participants into an RTC session
    pub async fn invite_rtc_session(&self, rtc_session_id: &str, body: &InviteRtcSessionRequest) -> Result<RtcSession, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/invite", rtc_session_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Accept an RTC session
    pub async fn accept_rtc_session(&self, rtc_session_id: &str, body: &UpdateRtcSessionRequest) -> Result<RtcSession, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/accept", rtc_session_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Reject an RTC session
    pub async fn reject_rtc_session(&self, rtc_session_id: &str, body: &UpdateRtcSessionRequest) -> Result<RtcSession, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/reject", rtc_session_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// End an RTC session
    pub async fn end_rtc_session(&self, rtc_session_id: &str, body: &UpdateRtcSessionRequest) -> Result<RtcSession, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/end", rtc_session_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Post an RTC signaling event
    pub async fn post_rtc_signal(&self, rtc_session_id: &str, body: &PostRtcSignalRequest) -> Result<RtcSignalEvent, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/signals", rtc_session_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Issue an RTC participant credential
    pub async fn issue_rtc_participant_credential(&self, rtc_session_id: &str, body: &IssueRtcParticipantCredentialRequest) -> Result<RtcParticipantCredential, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/credentials", rtc_session_id));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Get the RTC recording artifact
    pub async fn get_rtc_recording_artifact(&self, rtc_session_id: &str) -> Result<RtcRecordingArtifact, SdkworkError> {
        let path = backend_path(&format!("/rtc/sessions/{}/artifacts/recording", rtc_session_id));
        self.client.get(&path, None, None).await
    }
}
