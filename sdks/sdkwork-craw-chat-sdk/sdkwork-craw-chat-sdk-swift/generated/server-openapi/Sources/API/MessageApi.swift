import Foundation

public class MessageApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Edit a posted message
    public func edit(messageId: String, body: EditMessageRequest) async throws -> MessageMutationResult? {
        return try await client.post(ApiPaths.backendPath("/messages/\(messageId)/edit"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessageMutationResult.self)
    }

    /// Recall a posted message
    public func recall(messageId: String) async throws -> MessageMutationResult? {
        return try await client.post(ApiPaths.backendPath("/messages/\(messageId)/recall"), body: nil, responseType: MessageMutationResult.self)
    }
}
