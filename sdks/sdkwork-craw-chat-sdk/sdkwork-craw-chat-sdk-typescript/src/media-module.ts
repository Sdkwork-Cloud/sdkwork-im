import type {
  AttachMediaRequest,
  AttachTextMediaOptions,
  CrawChatMediaUploadOptions,
  CrawChatMediaUploadSession,
  CrawChatUploadedMediaAsset,
  CompleteUploadRequest,
  CreateUploadRequest,
  MediaAsset,
  MediaDownloadUrlResponse,
  PostMessageResult,
  QueryParams,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
import {
  normalizeUploadSession,
  performPresignedMediaUpload,
} from './media-upload-runtime.js';

export class CrawChatMediaModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  async createUpload(body: CreateUploadRequest): Promise<CrawChatMediaUploadSession> {
    const response = await this.context.backendClient.media.createMediaUpload(body);
    return normalizeUploadSession(response);
  }

  createUploadSession(body: CreateUploadRequest): Promise<CrawChatMediaUploadSession> {
    return this.createUpload(body);
  }

  completeUpload(
    mediaAssetId: string | number,
    body: CompleteUploadRequest,
  ): Promise<MediaAsset> {
    return this.context.backendClient.media.completeMediaUpload(mediaAssetId, body);
  }

  uploadAndComplete(
    options: CrawChatMediaUploadOptions,
  ): Promise<CrawChatUploadedMediaAsset> {
    return this.upload(options);
  }

  async upload(options: CrawChatMediaUploadOptions): Promise<CrawChatUploadedMediaAsset> {
    return performPresignedMediaUpload(this.context, options);
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
