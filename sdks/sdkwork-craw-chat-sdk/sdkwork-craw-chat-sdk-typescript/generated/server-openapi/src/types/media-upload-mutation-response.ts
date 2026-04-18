import type { MediaProcessingState } from './media-processing-state';
import type { MediaResource } from './media-resource';
import type { MediaUploadDeliveryStatus } from './media-upload-delivery-status';
import type { MediaUploadSession } from './media-upload-session';

export interface MediaUploadMutationResponse {
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
  upload?: MediaUploadSession;
  requestKey: string;
  deliveryStatus: MediaUploadDeliveryStatus;
  proofVersion: string;
}
