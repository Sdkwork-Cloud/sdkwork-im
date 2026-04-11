export class CrawChatMediaModule {
    context;
    constructor(context) {
        this.context = context;
    }
    createUpload(body) {
        return this.context.backendClient.media.createMediaUpload(body);
    }
    completeUpload(mediaAssetId, body) {
        return this.context.backendClient.media.completeMediaUpload(mediaAssetId, body);
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
