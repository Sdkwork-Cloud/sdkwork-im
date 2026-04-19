import type { ImMediaUploadOptions, ImMediaUploadSession, ImUploadedMediaAsset } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare function normalizeUploadSession(value: unknown): ImMediaUploadSession;
export declare function performPresignedMediaUpload(context: ImSdkContext, options: ImMediaUploadOptions): Promise<ImUploadedMediaAsset>;
//# sourceMappingURL=media-upload-runtime.d.ts.map