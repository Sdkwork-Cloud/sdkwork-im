import { normalizeUploadSession, performPresignedMediaUpload, } from './media-upload-runtime.js';
export class ImMediaModule {
    context;
    constructor(context) {
        this.context = context;
    }
    async createUpload(body) {
        const response = await this.context.transportClient.media.createMediaUpload(body);
        return normalizeUploadSession(response);
    }
    createUploadSession(body) {
        return this.createUpload(body);
    }
    completeUpload(mediaAssetId, body) {
        return this.context.transportClient.media.completeMediaUpload(mediaAssetId, body);
    }
    uploadAndComplete(options) {
        return this.upload(options);
    }
    async upload(options) {
        return performPresignedMediaUpload(this.context, options);
    }
    getDownloadUrl(mediaAssetId, params) {
        return this.context.transportClient.media.getMediaDownloadUrl(mediaAssetId, params);
    }
    get(mediaAssetId) {
        return this.context.transportClient.media.getMediaAsset(mediaAssetId);
    }
    attach(mediaAssetId, body) {
        return this.context.transportClient.media.attachMediaAsset(mediaAssetId, body);
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
