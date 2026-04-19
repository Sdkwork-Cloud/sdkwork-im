import type { HttpClient } from '../http/client.js';
import type { QueryParams } from '../types/common.js';
import type { AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest, OpenStreamRequest, StreamFrame, StreamFrameWindow, StreamSession } from '../types/index.js';
export declare class StreamApi {
    private client;
    constructor(client: HttpClient);
    /** Open a stream session */
    open(body: OpenStreamRequest): Promise<StreamSession>;
    /** List stream frames */
    listStreamFrames(streamId: string | number, params?: QueryParams): Promise<StreamFrameWindow>;
    /** Append a frame to a stream */
    appendStreamFrame(streamId: string | number, body: AppendStreamFrameRequest): Promise<StreamFrame>;
    /** Checkpoint a stream session */
    checkpoint(streamId: string | number, body: CheckpointStreamRequest): Promise<StreamSession>;
    /** Complete a stream session */
    complete(streamId: string | number, body: CompleteStreamRequest): Promise<StreamSession>;
    /** Abort a stream session */
    abort(streamId: string | number, body: AbortStreamRequest): Promise<StreamSession>;
}
export declare function createStreamApi(client: HttpClient): StreamApi;
//# sourceMappingURL=stream.d.ts.map