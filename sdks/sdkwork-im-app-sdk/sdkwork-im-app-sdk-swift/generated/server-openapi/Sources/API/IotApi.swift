import Foundation

public class IotApi {
    private let client: HttpClient

    public init(client: HttpClient) {
        self.client = client
    }

    /// Retrieve IoT access provider health
    public func accessProviderHealthRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/iot/access/provider_health"), responseType: [String: Any].self)
    }

    /// Retrieve IoT protocol provider health
    public func protocolProviderHealthRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/iot/protocol/provider_health"), responseType: [String: Any].self)
    }

    /// Ingest IoT protocol uplink
    public func protocolUplinkCreate() async throws -> [String: Any]? {
        return try await client.post(ApiPaths.appPath("/iot/protocol/uplink"), body: nil, responseType: [String: Any].self)
    }

    /// Ingest IoT protocol downlink
    public func protocolDownlinkCreate() async throws -> [String: Any]? {
        return try await client.post(ApiPaths.appPath("/iot/protocol/downlink"), body: nil, responseType: [String: Any].self)
    }



}
