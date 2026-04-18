import Foundation

public class StreamApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Open a stream session
    public func open_(body: OpenStreamRequest) async throws -> StreamSession? {
        return try await client.post(ApiPaths.backendPath("/streams"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamSession.self)
    }

    /// List stream frames
    public func listStreamFrames(streamId: String, params: [String: Any]? = nil) async throws -> StreamFrameWindow? {
        return try await client.get(ApiPaths.backendPath("/streams/\(streamId)/frames"), params: params, responseType: StreamFrameWindow.self)
    }

    /// Append a frame to a stream
    public func appendStreamFrame(streamId: String, body: AppendStreamFrameRequest) async throws -> StreamFrame? {
        return try await client.post(ApiPaths.backendPath("/streams/\(streamId)/frames"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamFrame.self)
    }

    /// Checkpoint a stream session
    public func checkpoint(streamId: String, body: CheckpointStreamRequest) async throws -> StreamSession? {
        return try await client.post(ApiPaths.backendPath("/streams/\(streamId)/checkpoint"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamSession.self)
    }

    /// Complete a stream session
    public func complete(streamId: String, body: CompleteStreamRequest) async throws -> StreamSession? {
        return try await client.post(ApiPaths.backendPath("/streams/\(streamId)/complete"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamSession.self)
    }

    /// Abort a stream session
    public func abort(streamId: String, body: AbortStreamRequest) async throws -> StreamSession? {
        return try await client.post(ApiPaths.backendPath("/streams/\(streamId)/abort"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StreamSession.self)
    }
}
