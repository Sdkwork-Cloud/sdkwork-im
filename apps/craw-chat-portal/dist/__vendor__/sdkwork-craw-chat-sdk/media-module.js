import { normalizeUploadSession, performPresignedMediaUpload, } from './media-upload-runtime.js';
export class CrawChatMediaModule {
    context;
    constructor(context) {
        this.context = context;
    }
    async createUpload(body) {
        const response = await this.context.backendClient.media.createMediaUpload(body);
        return normalizeUploadSession(response);
    }
    createUploadSession(body) {
        return this.createUpload(body);
    }
    completeUpload(mediaAssetId, body) {
        return this.context.backendClient.media.completeMediaUpload(mediaAssetId, body);
    }
    uploadAndComplete(options) {
        return this.upload(options);
    }
    async upload(options) {
        return performPresignedMediaUpload(this.context, options);
    }
    getDownloadUrl(mediaAssetId, params) {
        return this.context.backendClient.media.getMediaDownloadUrl(mediaAssetId, params);
    }
    get(mediaAssetId) {
        return this.context.backendClient.media.getMediaAsset(mediaAssetId);
    }
    attach(mediaAssetId, body) {
        return this.context.backendClient.media.attachMediaAsset(mediaAssetId, body);
    }
    attachText(mediaAssetId, options) {
        return this.attach(mediaAssetId, {
            conversationId: options.conversationId,
            clientMsgId: options.clientMsgId,
            summary: options.summary,
            text: options.text,
            renderHints: options.renderHints,
        });
    }
}
