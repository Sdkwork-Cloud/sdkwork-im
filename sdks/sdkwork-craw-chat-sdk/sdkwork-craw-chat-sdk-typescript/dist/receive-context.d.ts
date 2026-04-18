import type { CrawChatInternalReceiverEvent } from './receiver-internal-types.js';
import type { CrawChatReceiveContext, CrawChatReceiveSource, RealtimeAckState } from './types.js';
export declare function toReceiveContext(event: CrawChatInternalReceiverEvent, source: CrawChatReceiveSource, ack: () => Promise<RealtimeAckState>): CrawChatReceiveContext;
//# sourceMappingURL=receive-context.d.ts.map