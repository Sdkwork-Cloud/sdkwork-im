import Foundation

public class InboxApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Get inbox entries
    public func getInbox() async throws -> InboxResponse? {
        return try await client.get(ApiPaths.backendPath("/inbox"), responseType: InboxResponse.self)
    }
}
