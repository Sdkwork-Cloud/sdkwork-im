import { backendApiPath } from './paths.js';
export class MediaApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Create a media upload session with presigned client-upload metadata */
    async createMediaUpload(body) {
        return this.client.post(backendApiPath(`/media/uploads`), body, undefined, undefined, 'application/json');
    }
    /** Complete a media upload */
    async completeMediaUpload(mediaAssetId, body) {
        return this.client.post(backendApiPath(`/media/uploads/${mediaAssetId}/complete`), body, undefined, undefined, 'application/json');
    }
    /** Issue a signed media download URL */
    async getMediaDownloadUrl(mediaAssetId, params) {
        return this.client.get(backendApiPath(`/media/${mediaAssetId}/download-url`), params);
    }
    /** Get a media asset by id */
    async getMediaAsset(mediaAssetId) {
        return this.client.get(backendApiPath(`/media/${mediaAssetId}`));
    }
    /** Attach a ready media asset as a conversation message */
    async attachMediaAsset(mediaAssetId, body) {
        return this.client.post(backendApiPath(`/media/${mediaAssetId}/attach`), body, undefined, undefined, 'application/json');
    }
}
export function createMediaApi(client) {
    return new MediaApi(client);
}
