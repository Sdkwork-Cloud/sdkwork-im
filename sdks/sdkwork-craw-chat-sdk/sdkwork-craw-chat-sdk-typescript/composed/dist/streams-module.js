import { buildTextFrameRequest } from './builders.js';
export class CrawChatStreamsModule {
    context;
    constructor(context) {
        this.context = context;
    }
    open(body) {
        return this.context.backendClient.stream.open(body);
    }
    listFrames(streamId, params) {
        return this.context.backendClient.stream.listStreamFrames(streamId, params);
    }
    appendFrame(streamId, body) {
        return this.context.backendClient.stream.appendStreamFrame(streamId, body);
    }
    appendTextFrame(streamId, options) {
        return this.appendFrame(streamId, buildTextFrameRequest(options));
    }
    checkpoint(streamId, body) {
        return this.context.backendClient.stream.checkpoint(streamId, body);
    }
    complete(streamId, body) {
        return this.context.backendClient.stream.complete(streamId, body);
    }
    abort(streamId, body) {
        return this.context.backendClient.stream.abort(streamId, body);
    }
}
