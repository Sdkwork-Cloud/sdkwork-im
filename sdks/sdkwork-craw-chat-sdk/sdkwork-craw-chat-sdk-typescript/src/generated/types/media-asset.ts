import type { MediaProcessingState } from './media-processing-state.js';
import type { MediaResource } from './media-resource.js';

export interface MediaAsset {
  tenantId: string;
  principalId: string;
  principalKind: string;
  mediaAssetId: string;
  bucket?: string;
  objectKey?: string;
  storageProvider?: string;
  checksum?: string;
  processingState: MediaProcessingState;
  resource: MediaResource;
  createdAt: string;
  completedAt?: string;
}
