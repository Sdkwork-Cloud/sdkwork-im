import type { AttachMediaRequest, AttachTextMediaOptions, CompleteUploadRequest, CreateUploadRequest, MediaAsset, MediaDownloadUrlResponse, PostMessageResult, QueryParams } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatMediaModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    createUpload(body: CreateUploadRequest): Promise<MediaAsset>;
    completeUpload(mediaAssetId: string | number, body: CompleteUploadRequest): Promise<MediaAsset>;
    getDownloadUrl(mediaAssetId: string | number, params?: QueryParams): Promise<MediaDownloadUrlResponse>;
    get(mediaAssetId: string | number): Promise<MediaAsset>;
    attach(mediaAssetId: string | number, body: AttachMediaRequest): Promise<PostMessageResult>;
    attachText(mediaAssetId: string | number, options: AttachTextMediaOptions): Promise<PostMessageResult>;
}
//# sourceMappingURL=media-module.d.ts.map