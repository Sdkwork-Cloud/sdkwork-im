import type { MediaResource } from './media-resource';

export interface CreateUploadRequest {
  mediaAssetId: string;
  resource: MediaResource;
}
