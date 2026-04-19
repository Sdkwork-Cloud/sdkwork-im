import type { AttachMediaRequest, AttachTextMediaOptions, ImMediaUploadOptions, ImMediaUploadSession, ImUploadedMediaAsset, CompleteUploadRequest, CreateUploadRequest, MediaAsset, MediaDownloadUrlResponse, PostMessageResult, QueryParams } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImMediaModule {
    private readonly context;
    constructor(context: ImSdkContext);
    createUpload(body: CreateUploadRequest): Promise<ImMediaUploadSession>;
    createUploadSession(body: CreateUploadRequest): Promise<ImMediaUploadSession>;
    completeUpload(mediaAssetId: string | number, body: CompleteUploadRequest): Promise<MediaAsset>;
    uploadAndComplete(options: ImMediaUploadOptions): Promise<ImUploadedMediaAsset>;
    upload(options: ImMediaUploadOptions): Promise<ImUploadedMediaAsset>;
    getDownloadUrl(mediaAssetId: string | number, params?: QueryParams): Promise<MediaDownloadUrlResponse>;
    get(mediaAssetId: string | number): Promise<MediaAsset>;
    attach(mediaAssetId: string | number, body: AttachMediaRequest): Promise<PostMessageResult>;
    attachText(mediaAssetId: string | number, options: AttachTextMediaOptions): Promise<PostMessageResult>;
}
//# sourceMappingURL=media-module.d.ts.map