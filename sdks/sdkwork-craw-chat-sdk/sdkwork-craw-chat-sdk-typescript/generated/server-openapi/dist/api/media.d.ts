import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { AttachMediaRequest, CompleteUploadRequest, CreateUploadRequest, MediaAsset, MediaDownloadUrlResponse, PostMessageResult } from '../types';
export declare class MediaApi {
    private client;
    constructor(client: HttpClient);
    /** Create a media upload record */
    createMediaUpload(body: CreateUploadRequest): Promise<MediaAsset>;
    /** Complete a media upload */
    completeMediaUpload(mediaAssetId: string | number, body: CompleteUploadRequest): Promise<MediaAsset>;
    /** Issue a signed media download URL */
    getMediaDownloadUrl(mediaAssetId: string | number, params?: QueryParams): Promise<MediaDownloadUrlResponse>;
    /** Get a media asset by id */
    getMediaAsset(mediaAssetId: string | number): Promise<MediaAsset>;
    /** Attach a ready media asset as a conversation message */
    attachMediaAsset(mediaAssetId: string | number, body: AttachMediaRequest): Promise<PostMessageResult>;
}
export declare function createMediaApi(client: HttpClient): MediaApi;
//# sourceMappingURL=media.d.ts.map