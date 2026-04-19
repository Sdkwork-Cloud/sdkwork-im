import { apiPath } from './paths.js';
export class StreamApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Open a stream session */
    async open(body) {
        return this.client.post(apiPath(`/streams`), body, undefined, undefined, 'application/json');
    }
    /** List stream frames */
    async listStreamFrames(streamId, params) {
        return this.client.get(apiPath(`/streams/${streamId}/frames`), params);
    }
    /** Append a frame to a stream */
    async appendStreamFrame(streamId, body) {
        return this.client.post(apiPath(`/streams/${streamId}/frames`), body, undefined, undefined, 'application/json');
    }
    /** Checkpoint a stream session */
    async checkpoint(streamId, body) {
        return this.client.post(apiPath(`/streams/${streamId}/checkpoint`), body, undefined, undefined, 'application/json');
    }
    /** Complete a stream session */
    async complete(streamId, body) {
        return this.client.post(apiPath(`/streams/${streamId}/complete`), body, undefined, undefined, 'application/json');
    }
    /** Abort a stream session */
    async abort(streamId, body) {
        return this.client.post(apiPath(`/streams/${streamId}/abort`), body, undefined, undefined, 'application/json');
    }
}
export function createStreamApi(client) {
    return new StreamApi(client);
}
