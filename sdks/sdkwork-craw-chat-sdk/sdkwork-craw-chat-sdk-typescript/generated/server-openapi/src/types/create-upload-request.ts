import type { MediaResource } from './media-resource';

export interface CreateUploadRequest {
  mediaAssetId: string;
  bucket: string;
  objectKey?: string;
  expiresInSeconds?: number;
  resource: MediaResource;
}
