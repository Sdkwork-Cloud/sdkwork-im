import type { ImCatchUpBatch, QueryParams, RealtimeAckState } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImSyncModule {
    private readonly context;
    constructor(context: ImSdkContext);
    catchUp(params?: QueryParams): Promise<ImCatchUpBatch>;
    ack(batchOrSequence: ImCatchUpBatch | number): Promise<RealtimeAckState>;
}
//# sourceMappingURL=sync-module.d.ts.map