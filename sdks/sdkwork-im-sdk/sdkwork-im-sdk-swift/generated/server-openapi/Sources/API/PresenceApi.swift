import Foundation

public class PresenceApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Publish current device presence heartbeat
    public func heartbeatCreate(body: DevicePresenceRequest) async throws -> PresenceView? {
        return try await client.post(ApiPaths.imPath("/presence/heartbeat"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PresenceView.self)
    }

    /// Retrieve current principal presence
    public func meRetrieve() async throws -> PresenceView? {
        return try await client.get(ApiPaths.imPath("/presence/me"), responseType: PresenceView.self)
    }



}
