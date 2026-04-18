function resolveUploadFetch(fetchOverride) {
    const fetchImpl = fetchOverride ?? globalThis.fetch;
    if (typeof fetchImpl !== 'function') {
        throw new Error('CrawChat media upload requires a fetch implementation. Pass options.fetch or use an environment with global fetch.');
    }
    return fetchImpl;
}
function requireUploadSession(response) {
    if (response.upload) {
        return response.upload;
    }
    throw new Error(`Media asset ${response.mediaAssetId} did not include a presigned upload session.`);
}
function buildCompleteUploadRequest(upload, checksum) {
    return {
        bucket: upload.bucket,
        objectKey: upload.objectKey,
        storageProvider: upload.storageProvider,
        url: upload.url,
        checksum,
    };
}
async function assertUploadSucceeded(response) {
    if (response.ok) {
        return;
    }
    let detail = '';
    try {
        detail = (await response.text()).trim();
    }
    catch {
        detail = '';
    }
    const suffix = detail ? `: ${detail}` : '';
    throw new Error(`CrawChat media upload failed with status ${response.status}${suffix}`);
}
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
    async uploadContent(upload, body, options = {}) {
        const fetchImpl = resolveUploadFetch(options.fetch);
        const response = await fetchImpl(upload.url, {
            method: upload.method,
            headers: upload.headers,
            body,
        });
        await assertUploadSucceeded(response);
    }
    async upload(request, body, options = {}) {
        const created = await this.createUpload(request);
        const upload = requireUploadSession(created);
        await this.uploadContent(upload, body, options);
        return this.completeUpload(created.mediaAssetId, buildCompleteUploadRequest(upload, options.checksum));
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
