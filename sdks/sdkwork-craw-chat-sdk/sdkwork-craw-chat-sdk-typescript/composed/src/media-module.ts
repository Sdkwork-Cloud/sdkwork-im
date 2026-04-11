import type {
  AttachMediaRequest,
  AttachTextMediaOptions,
  CompleteUploadRequest,
  CreateUploadRequest,
  MediaAsset,
  MediaDownloadUrlResponse,
  PostMessageResult,
  QueryParams,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatMediaModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  createUpload(body: CreateUploadRequest): Promise<MediaAsset> {
    return this.context.backendClient.media.createMediaUpload(body);
  }

  completeUpload(
    mediaAssetId: string | number,
    body: CompleteUploadRequest,
  ): Promise<MediaAsset> {
    return this.context.backendClient.media.completeMediaUpload(mediaAssetId, body);
  }

  getDownloadUrl(
    mediaAssetId: string | number,
    params?: QueryParams,
  ): Promise<MediaDownloadUrlResponse> {
    return this.context.backendClient.media.getMediaDownloadUrl(mediaAssetId, params);
  }

  get(mediaAssetId: string | number): Promise<MediaAsset> {
    return this.context.backendClient.media.getMediaAsset(mediaAssetId);
  }

  attach(
    mediaAssetId: string | number,
    body: AttachMediaRequest,
  ): Promise<PostMessageResult> {
    return this.context.backendClient.media.attachMediaAsset(mediaAssetId, body);
  }

  attachText(
    mediaAssetId: string | number,
    options: AttachTextMediaOptions,
  ): Promise<PostMessageResult> {
    return this.attach(mediaAssetId, {
      conversationId: options.conversationId,
      clientMsgId: options.clientMsgId,
      summary: options.summary,
      text: options.text,
      renderHints: options.renderHints,
    });
  }
}
