import type { CrawChatMediaUploadOptions, CrawChatMediaUploadSession, CrawChatUploadedMediaAsset } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare function normalizeUploadSession(value: unknown): CrawChatMediaUploadSession;
export declare function performPresignedMediaUpload(context: CrawChatSdkContext, options: CrawChatMediaUploadOptions): Promise<CrawChatUploadedMediaAsset>;
//# sourceMappingURL=media-upload-runtime.d.ts.map