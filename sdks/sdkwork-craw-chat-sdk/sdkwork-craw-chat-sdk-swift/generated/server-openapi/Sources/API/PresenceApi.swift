import Foundation

public class PresenceApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Refresh device presence
    public func heartbeat(body: PresenceDeviceRequest) async throws -> PresenceSnapshotView? {
        return try await client.post(ApiPaths.backendPath("/presence/heartbeat"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PresenceSnapshotView.self)
    }

    /// Get current presence
    public func getPresenceMe() async throws -> PresenceSnapshotView? {
        return try await client.get(ApiPaths.backendPath("/presence/me"), responseType: PresenceSnapshotView.self)
    }
}
