import type {
  AttachMediaRequest,
  AttachTextMediaOptions,
  CrawChatMediaUploadOptions,
  CrawChatUploadBody,
  CrawChatUploadFetchLike,
  CompleteUploadRequest,
  CreateUploadRequest,
  MediaAsset,
  MediaDownloadUrlResponse,
  MediaUploadMutationResponse,
  MediaUploadSession,
  PostMessageResult,
  QueryParams,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

function resolveUploadFetch(fetchOverride?: CrawChatUploadFetchLike): CrawChatUploadFetchLike {
  const fetchImpl = fetchOverride ?? globalThis.fetch;
  if (typeof fetchImpl !== 'function') {
    throw new Error(
      'CrawChat media upload requires a fetch implementation. Pass options.fetch or use an environment with global fetch.',
    );
  }
  return fetchImpl as CrawChatUploadFetchLike;
}

function requireUploadSession(response: MediaUploadMutationResponse): MediaUploadSession {
  if (response.upload) {
    return response.upload;
  }
  throw new Error(
    `Media asset ${response.mediaAssetId} did not include a presigned upload session.`,
  );
}

function buildCompleteUploadRequest(
  upload: MediaUploadSession,
  checksum?: string,
): CompleteUploadRequest {
  return {
    bucket: upload.bucket,
    objectKey: upload.objectKey,
    storageProvider: upload.storageProvider,
    url: upload.url,
    checksum,
  };
}

async function assertUploadSucceeded(response: Awaited<ReturnType<CrawChatUploadFetchLike>>) {
  if (response.ok) {
    return;
  }

  let detail = '';
  try {
    detail = (await response.text()).trim();
  } catch {
    detail = '';
  }

  const suffix = detail ? `: ${detail}` : '';
  throw new Error(`CrawChat media upload failed with status ${response.status}${suffix}`);
}

export class CrawChatMediaModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  createUpload(body: CreateUploadRequest): Promise<MediaUploadMutationResponse> {
    return this.context.backendClient.media.createMediaUpload(body);
  }

  completeUpload(
    mediaAssetId: string | number,
    body: CompleteUploadRequest,
  ): Promise<MediaUploadMutationResponse> {
    return this.context.backendClient.media.completeMediaUpload(mediaAssetId, body);
  }

  async uploadContent(
    upload: MediaUploadSession,
    body: CrawChatUploadBody,
    options: CrawChatMediaUploadOptions = {},
  ): Promise<void> {
    const fetchImpl = resolveUploadFetch(options.fetch);
    const response = await fetchImpl(upload.url, {
      method: upload.method,
      headers: upload.headers,
      body,
    });
    await assertUploadSucceeded(response);
  }

  async upload(
    request: CreateUploadRequest,
    body: CrawChatUploadBody,
    options: CrawChatMediaUploadOptions = {},
  ): Promise<MediaUploadMutationResponse> {
    const created = await this.createUpload(request);
    const upload = requireUploadSession(created);
    await this.uploadContent(upload, body, options);
    return this.completeUpload(
      created.mediaAssetId,
      buildCompleteUploadRequest(upload, options.checksum),
    );
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
