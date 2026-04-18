import type { MediaResource } from './media-resource';
export interface CreateUploadRequest {
    mediaAssetId: string;
    bucket: string;
    objectKey?: string;
    expiresInSeconds?: number;
    resource: MediaResource;
}
//# sourceMappingURL=create-upload-request.d.ts.map