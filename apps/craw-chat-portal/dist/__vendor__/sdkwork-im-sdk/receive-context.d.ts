import type { ImInternalReceiverEvent } from './receiver-internal-types.js';
import type { ImReceiveContext, ImReceiveSource, RealtimeAckState } from './types.js';
export declare function toReceiveContext(event: ImInternalReceiverEvent, source: ImReceiveSource, ack: () => Promise<RealtimeAckState>): ImReceiveContext;
//# sourceMappingURL=receive-context.d.ts.map