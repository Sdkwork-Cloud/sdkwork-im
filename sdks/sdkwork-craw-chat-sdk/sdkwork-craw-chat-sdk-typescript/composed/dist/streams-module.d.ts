import type { AbortStreamRequest, AppendStreamFrameRequest, AppendTextFrameOptions, CheckpointStreamRequest, CompleteStreamRequest, OpenStreamRequest, QueryParams, StreamFrame, StreamFrameWindow, StreamSession } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatStreamsModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    open(body: OpenStreamRequest): Promise<StreamSession>;
    listFrames(streamId: string | number, params?: QueryParams): Promise<StreamFrameWindow>;
    appendFrame(streamId: string | number, body: AppendStreamFrameRequest): Promise<StreamFrame>;
    appendTextFrame(streamId: string | number, options: AppendTextFrameOptions): Promise<StreamFrame>;
    checkpoint(streamId: string | number, body: CheckpointStreamRequest): Promise<StreamSession>;
    complete(streamId: string | number, body: CompleteStreamRequest): Promise<StreamSession>;
    abort(streamId: string | number, body: AbortStreamRequest): Promise<StreamSession>;
}
//# sourceMappingURL=streams-module.d.ts.map