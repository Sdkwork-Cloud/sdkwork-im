import type { MediaAsset } from './media-asset';
import type { StringMap } from './string-map';

export interface MediaUploadSessionResponse {
  mediaAssetId: string;
  mediaAsset: MediaAsset;
  bucket: string;
  objectKey: string;
  storageProvider: string;
  uploadMethod: string;
  uploadUrl: string;
  uploadHeaders?: StringMap;
  uploadExpiresInSeconds?: number;
  requestKey?: string;
  deliveryStatus?: string;
  proofVersion?: string;
}
