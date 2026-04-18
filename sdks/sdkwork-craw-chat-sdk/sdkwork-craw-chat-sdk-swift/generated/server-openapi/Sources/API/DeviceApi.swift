import Foundation

public class DeviceApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Register the current device
    public func register(body: RegisterDeviceRequest) async throws -> RegisteredDeviceView? {
        return try await client.post(ApiPaths.backendPath("/devices/register"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RegisteredDeviceView.self)
    }

    /// Get device sync feed entries
    public func getDeviceSyncFeed(deviceId: String, params: [String: Any]? = nil) async throws -> DeviceSyncFeedResponse? {
        return try await client.get(ApiPaths.backendPath("/devices/\(deviceId)/sync-feed"), params: params, responseType: DeviceSyncFeedResponse.self)
    }
}
