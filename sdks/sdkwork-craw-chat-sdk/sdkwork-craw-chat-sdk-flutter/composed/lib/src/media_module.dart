import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';
import 'types.dart';

class CrawChatMediaModule {
  final CrawChatSdkContext context;

  CrawChatMediaModule(this.context);

  Future<MediaAsset?> createUpload(CreateUploadRequest body) {
    return context.backendClient.media.createMediaUpload(body);
  }

  Future<MediaAsset?> completeUpload(
    String mediaAssetId,
    CompleteUploadRequest body,
  ) {
    return context.backendClient.media.completeMediaUpload(mediaAssetId, body);
  }

  Future<MediaDownloadUrlResponse?> getDownloadUrl(
    String mediaAssetId, [
    CrawChatQueryParams? params,
  ]) {
    return context.backendClient.media.getMediaDownloadUrl(mediaAssetId, params);
  }

  Future<MediaAsset?> get(String mediaAssetId) {
    return context.backendClient.media.getMediaAsset(mediaAssetId);
  }

  Future<PostMessageResult?> attach(
    String mediaAssetId,
    AttachMediaRequest body,
  ) {
    return context.backendClient.media.attachMediaAsset(mediaAssetId, body);
  }

  Future<PostMessageResult?> attachText(
    String mediaAssetId,
    CrawChatAttachTextMediaOptions options,
  ) {
    return attach(
      mediaAssetId,
      AttachMediaRequest(
        conversationId: options.conversationId,
        clientMsgId: options.clientMsgId,
        summary: options.summary,
        text: options.text,
        renderHints: options.renderHints,
      ),
    );
  }
}
