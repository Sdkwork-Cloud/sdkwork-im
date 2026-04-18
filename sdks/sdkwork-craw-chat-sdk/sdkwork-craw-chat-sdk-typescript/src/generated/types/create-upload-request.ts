import type { MediaResource } from './media-resource.js';

export interface CreateUploadRequest {
  mediaAssetId: string;
  bucket: string;
  objectKey?: string;
  expiresInSeconds?: number;
  resource: MediaResource;
}
