import Foundation

public class SessionApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Resume the current app session
    public func resume(body: ResumeSessionRequest) async throws -> SessionResumeView? {
        return try await client.post(ApiPaths.backendPath("/sessions/resume"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SessionResumeView.self)
    }

    /// Disconnect the current app session device route
    public func disconnect(body: PresenceDeviceRequest) async throws -> PresenceSnapshotView? {
        return try await client.post(ApiPaths.backendPath("/sessions/disconnect"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PresenceSnapshotView.self)
    }
}
