import Foundation

public class RtcApi {
    private let client: HttpClient

    public init(client: HttpClient) {
        self.client = client
    }

    /// Map RTC provider callback
    public func providerCallbacksCreate() async throws -> [String: Any]? {
        return try await client.post(ApiPaths.appPath("/rtc/provider_callbacks"), body: nil, responseType: [String: Any].self)
    }

    /// Retrieve RTC provider health
    public func providerHealthRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/rtc/provider_health"), responseType: [String: Any].self)
    }



}
