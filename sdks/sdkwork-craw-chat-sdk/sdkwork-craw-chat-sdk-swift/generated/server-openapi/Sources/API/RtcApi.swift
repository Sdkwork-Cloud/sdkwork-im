import Foundation

public class RtcApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Create an RTC session
    public func createRtcSession(body: CreateRtcSessionRequest) async throws -> RtcSession? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcSession.self)
    }

    /// Invite participants into an RTC session
    public func inviteRtcSession(rtcSessionId: String, body: InviteRtcSessionRequest) async throws -> RtcSession? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/invite"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcSession.self)
    }

    /// Accept an RTC session
    public func acceptRtcSession(rtcSessionId: String, body: UpdateRtcSessionRequest) async throws -> RtcSession? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/accept"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcSession.self)
    }

    /// Reject an RTC session
    public func rejectRtcSession(rtcSessionId: String, body: UpdateRtcSessionRequest) async throws -> RtcSession? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/reject"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcSession.self)
    }

    /// End an RTC session
    public func endRtcSession(rtcSessionId: String, body: UpdateRtcSessionRequest) async throws -> RtcSession? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/end"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcSession.self)
    }

    /// Post an RTC signaling event
    public func postRtcSignal(rtcSessionId: String, body: PostRtcSignalRequest) async throws -> RtcSignalEvent? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/signals"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcSignalEvent.self)
    }

    /// Issue an RTC participant credential
    public func issueRtcParticipantCredential(rtcSessionId: String, body: IssueRtcParticipantCredentialRequest) async throws -> RtcParticipantCredential? {
        return try await client.post(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/credentials"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RtcParticipantCredential.self)
    }

    /// Get the RTC recording artifact
    public func getRtcRecordingArtifact(rtcSessionId: String) async throws -> RtcRecordingArtifact? {
        return try await client.get(ApiPaths.backendPath("/rtc/sessions/\(rtcSessionId)/artifacts/recording"), responseType: RtcRecordingArtifact.self)
    }
}
