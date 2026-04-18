import Foundation

public class MediaApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Create a media upload record
    public func createMediaUpload(body: CreateUploadRequest) async throws -> MediaAsset? {
        return try await client.post(ApiPaths.backendPath("/media/uploads"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MediaAsset.self)
    }

    /// Complete a media upload
    public func completeMediaUpload(mediaAssetId: String, body: CompleteUploadRequest) async throws -> MediaAsset? {
        return try await client.post(ApiPaths.backendPath("/media/uploads/\(mediaAssetId)/complete"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MediaAsset.self)
    }

    /// Issue a signed media download URL
    public func getMediaDownloadUrl(mediaAssetId: String, params: [String: Any]? = nil) async throws -> MediaDownloadUrlResponse? {
        return try await client.get(ApiPaths.backendPath("/media/\(mediaAssetId)/download-url"), params: params, responseType: MediaDownloadUrlResponse.self)
    }

    /// Get a media asset by id
    public func getMediaAsset(mediaAssetId: String) async throws -> MediaAsset? {
        return try await client.get(ApiPaths.backendPath("/media/\(mediaAssetId)"), responseType: MediaAsset.self)
    }

    /// Attach a ready media asset as a conversation message
    public func attachMediaAsset(mediaAssetId: String, body: AttachMediaRequest) async throws -> PostMessageResult? {
        return try await client.post(ApiPaths.backendPath("/media/\(mediaAssetId)/attach"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PostMessageResult.self)
    }
}
