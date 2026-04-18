import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class MediaApi {
  final HttpClient _client;

  MediaApi(this._client);

  /// Create a media upload record
  Future<MediaUploadMutationResponse?> createMediaUpload(CreateUploadRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/media/uploads'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MediaUploadMutationResponse.fromJson(map);
    })();
  }

  /// Complete a media upload
  Future<MediaUploadMutationResponse?> completeMediaUpload(String mediaAssetId, CompleteUploadRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/media/uploads/$mediaAssetId/complete'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MediaUploadMutationResponse.fromJson(map);
    })();
  }

  /// Issue a signed media download URL
  Future<MediaDownloadUrlResponse?> getMediaDownloadUrl(String mediaAssetId, Map<String, dynamic>? params) async {
    final response = await _client.get(ApiPaths.backendPath('/media/$mediaAssetId/download-url'), params: params);
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MediaDownloadUrlResponse.fromJson(map);
    })();
  }

  /// Get a media asset by id
  Future<MediaAsset?> getMediaAsset(String mediaAssetId) async {
    final response = await _client.get(ApiPaths.backendPath('/media/$mediaAssetId'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MediaAsset.fromJson(map);
    })();
  }

  /// Attach a ready media asset as a conversation message
  Future<PostMessageResult?> attachMediaAsset(String mediaAssetId, AttachMediaRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/media/$mediaAssetId/attach'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PostMessageResult.fromJson(map);
    })();
  }
}
