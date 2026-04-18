from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, UpdateRtcSessionRequest

class RtcApi:
    """rtc API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def create_rtc_session(self, body: CreateRtcSessionRequest) -> RtcSession:
        """Create an RTC session"""
        return self._client.post(f"/api/v1/rtc/sessions", json=body)

    def invite_rtc_session(self, rtc_session_id: str, body: InviteRtcSessionRequest) -> RtcSession:
        """Invite participants into an RTC session"""
        return self._client.post(f"/api/v1/rtc/sessions/{rtc_session_id}/invite", json=body)

    def accept_rtc_session(self, rtc_session_id: str, body: UpdateRtcSessionRequest) -> RtcSession:
        """Accept an RTC session"""
        return self._client.post(f"/api/v1/rtc/sessions/{rtc_session_id}/accept", json=body)

    def reject_rtc_session(self, rtc_session_id: str, body: UpdateRtcSessionRequest) -> RtcSession:
        """Reject an RTC session"""
        return self._client.post(f"/api/v1/rtc/sessions/{rtc_session_id}/reject", json=body)

    def end_rtc_session(self, rtc_session_id: str, body: UpdateRtcSessionRequest) -> RtcSession:
        """End an RTC session"""
        return self._client.post(f"/api/v1/rtc/sessions/{rtc_session_id}/end", json=body)

    def post_rtc_signal(self, rtc_session_id: str, body: PostRtcSignalRequest) -> RtcSignalEvent:
        """Post an RTC signaling event"""
        return self._client.post(f"/api/v1/rtc/sessions/{rtc_session_id}/signals", json=body)

    def issue_rtc_participant_credential(self, rtc_session_id: str, body: IssueRtcParticipantCredentialRequest) -> RtcParticipantCredential:
        """Issue an RTC participant credential"""
        return self._client.post(f"/api/v1/rtc/sessions/{rtc_session_id}/credentials", json=body)

    def get_rtc_recording_artifact(self, rtc_session_id: str) -> RtcRecordingArtifact:
        """Get the RTC recording artifact"""
        return self._client.get(f"/api/v1/rtc/sessions/{rtc_session_id}/artifacts/recording")
