import Foundation

public class RealtimeApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Replace realtime subscriptions for the current device
    public func syncRealtimeSubscriptions(body: SyncRealtimeSubscriptionsRequest) async throws -> RealtimeSubscriptionSnapshot? {
        return try await client.post(ApiPaths.backendPath("/realtime/subscriptions/sync"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RealtimeSubscriptionSnapshot.self)
    }

    /// Pull realtime events for the current device
    public func listRealtimeEvents(params: [String: Any]? = nil) async throws -> RealtimeEventWindow? {
        return try await client.get(ApiPaths.backendPath("/realtime/events"), params: params, responseType: RealtimeEventWindow.self)
    }

    /// Ack realtime events for the current device
    public func ackRealtimeEvents(body: AckRealtimeEventsRequest) async throws -> RealtimeAckState? {
        return try await client.post(ApiPaths.backendPath("/realtime/events/ack"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RealtimeAckState.self)
    }
}
