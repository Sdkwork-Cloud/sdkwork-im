import type { AttachMediaRequest, AttachTextMediaOptions, CrawChatMediaUploadOptions, CrawChatUploadBody, CompleteUploadRequest, CreateUploadRequest, MediaAsset, MediaDownloadUrlResponse, MediaUploadMutationResponse, MediaUploadSession, PostMessageResult, QueryParams } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatMediaModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    createUpload(body: CreateUploadRequest): Promise<MediaUploadMutationResponse>;
    completeUpload(mediaAssetId: string | number, body: CompleteUploadRequest): Promise<MediaUploadMutationResponse>;
    uploadContent(upload: MediaUploadSession, body: CrawChatUploadBody, options?: CrawChatMediaUploadOptions): Promise<void>;
    upload(request: CreateUploadRequest, body: CrawChatUploadBody, options?: CrawChatMediaUploadOptions): Promise<MediaUploadMutationResponse>;
    getDownloadUrl(mediaAssetId: string | number, params?: QueryParams): Promise<MediaDownloadUrlResponse>;
    get(mediaAssetId: string | number): Promise<MediaAsset>;
    attach(mediaAssetId: string | number, body: AttachMediaRequest): Promise<PostMessageResult>;
    attachText(mediaAssetId: string | number, options: AttachTextMediaOptions): Promise<PostMessageResult>;
}
//# sourceMappingURL=media-module.d.ts.map