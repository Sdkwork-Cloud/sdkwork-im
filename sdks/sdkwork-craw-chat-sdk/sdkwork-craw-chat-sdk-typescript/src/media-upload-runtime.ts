import { CrawChatSdkError } from './errors.js';
import type {
  CrawChatMediaUploadOptions,
  CrawChatMediaUploadSession,
  CrawChatUploadedMediaAsset,
  MediaAsset,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export function normalizeUploadSession(value: unknown): CrawChatMediaUploadSession {
  const session = value as Partial<CrawChatMediaUploadSession> | undefined;
  if (
    !session
    || typeof session.mediaAssetId !== 'string'
    || typeof session.uploadUrl !== 'string'
    || typeof session.uploadMethod !== 'string'
    || typeof session.bucket !== 'string'
    || typeof session.objectKey !== 'string'
  ) {
    throw new CrawChatSdkError(
      'media_upload_session_invalid',
      'createMediaUpload did not return a presigned upload session. Refresh the backend schema and generated SDK surfaces.',
    );
  }

  return {
    mediaAssetId: session.mediaAssetId,
    mediaAsset: (session.mediaAsset ?? { mediaAssetId: session.mediaAssetId }) as MediaAsset,
    bucket: session.bucket,
    objectKey: session.objectKey,
    storageProvider: typeof session.storageProvider === 'string'
      ? session.storageProvider
      : '',
    uploadMethod: session.uploadMethod,
    uploadUrl: session.uploadUrl,
    uploadHeaders: session.uploadHeaders,
    uploadExpiresInSeconds: session.uploadExpiresInSeconds,
    requestKey: session.requestKey,
    deliveryStatus: session.deliveryStatus,
    proofVersion: session.proofVersion,
  };
}

export async function performPresignedMediaUpload(
  context: CrawChatSdkContext,
  options: CrawChatMediaUploadOptions,
): Promise<CrawChatUploadedMediaAsset> {
  const session = normalizeUploadSession(
    await context.backendClient.media.createMediaUpload({
      mediaAssetId: options.mediaAssetId,
      bucket: options.bucket,
      resource: options.resource,
      ...(options.objectKey != null ? { objectKey: options.objectKey } : {}),
      ...(options.expiresInSeconds != null
        ? { expiresInSeconds: options.expiresInSeconds }
        : {}),
    }),
  );

  const uploadResponse = await executePresignedUpload(
    session.uploadUrl,
    session.uploadMethod,
    session.uploadHeaders,
    options.body,
  );
  const completedAsset = await context.backendClient.media.completeMediaUpload(
    session.mediaAssetId,
    {
      bucket: session.bucket,
      objectKey: session.objectKey,
      url: session.uploadUrl,
      storageProvider: session.storageProvider,
      checksum: options.checksum,
      etag: uploadResponse.etag,
    },
  );

  return {
    mediaAssetId: session.mediaAssetId,
    createdAsset: session.mediaAsset,
    completedAsset,
    asset: completedAsset,
    url: completedAsset.resource?.url,
    session,
    etag: uploadResponse.etag,
  };
}

async function executePresignedUpload(
  url: string,
  method: string,
  headers: Record<string, string> | undefined,
  body: Exclude<RequestInit['body'], null | undefined>,
): Promise<{ etag?: string }> {
  if (typeof globalThis.fetch !== 'function') {
    throw new CrawChatSdkError(
      'upload_fetch_required',
      'Global fetch is required to upload binary media in this runtime.',
    );
  }

  const response = await globalThis.fetch(url, {
    method,
    headers,
    body,
  });

  if (!response.ok) {
    throw new CrawChatSdkError(
      'upload_request_failed',
      `Presigned media upload failed with status ${response.status}.`,
    );
  }

  return {
    etag: response.headers.get('etag') ?? response.headers.get('ETag') ?? undefined,
  };
}
