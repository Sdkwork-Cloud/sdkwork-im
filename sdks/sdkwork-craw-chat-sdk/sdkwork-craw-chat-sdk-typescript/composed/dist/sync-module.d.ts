import type { CrawChatCatchUpBatch, QueryParams, RealtimeAckState } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatSyncModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    catchUp(params?: QueryParams): Promise<CrawChatCatchUpBatch>;
    ack(batchOrSequence: CrawChatCatchUpBatch | number): Promise<RealtimeAckState>;
}
//# sourceMappingURL=sync-module.d.ts.map