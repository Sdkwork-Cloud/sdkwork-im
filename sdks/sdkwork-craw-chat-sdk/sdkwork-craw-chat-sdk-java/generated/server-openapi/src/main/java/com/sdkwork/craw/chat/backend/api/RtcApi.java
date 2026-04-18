package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class RtcApi {
    private final HttpClient client;
    
    public RtcApi(HttpClient client) {
        this.client = client;
    }

    /** Create an RTC session */
    public RtcSession createRtcSession(CreateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSession>() {});
    }

    /** Invite participants into an RTC session */
    public RtcSession inviteRtcSession(String rtcSessionId, InviteRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/invite"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSession>() {});
    }

    /** Accept an RTC session */
    public RtcSession acceptRtcSession(String rtcSessionId, UpdateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/accept"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSession>() {});
    }

    /** Reject an RTC session */
    public RtcSession rejectRtcSession(String rtcSessionId, UpdateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/reject"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSession>() {});
    }

    /** End an RTC session */
    public RtcSession endRtcSession(String rtcSessionId, UpdateRtcSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/end"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSession>() {});
    }

    /** Post an RTC signaling event */
    public RtcSignalEvent postRtcSignal(String rtcSessionId, PostRtcSignalRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/signals"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcSignalEvent>() {});
    }

    /** Issue an RTC participant credential */
    public RtcParticipantCredential issueRtcParticipantCredential(String rtcSessionId, IssueRtcParticipantCredentialRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/credentials"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RtcParticipantCredential>() {});
    }

    /** Get the RTC recording artifact */
    public RtcRecordingArtifact getRtcRecordingArtifact(String rtcSessionId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/rtc/sessions/" + rtcSessionId + "/artifacts/recording"));
        return client.convertValue(raw, new TypeReference<RtcRecordingArtifact>() {});
    }
}
