import { decodeRealtimeEvent } from './message-codec.js';
import { toReceiveContext } from './receive-context.js';
export class ImSyncModule {
    context;
    constructor(context) {
        this.context = context;
    }
    async catchUp(params = {}) {
        const rawWindow = await this.context.transportClient.realtime.listRealtimeEvents(params);
        const items = rawWindow.items.map((item) => {
            const decoded = decodeRealtimeEvent(item);
            return toReceiveContext(decoded, 'catch_up', () => this.ack(decoded.realtimeSeq));
        });
        const highestSequence = items.reduce((currentMax, item) => Math.max(currentMax, item.sequence), rawWindow.ackedThroughSeq ?? 0);
        return {
            items,
            highestSequence,
            rawWindow,
        };
    }
    ack(batchOrSequence) {
        const ackedSeq = typeof batchOrSequence === 'number'
            ? batchOrSequence
            : batchOrSequence.highestSequence;
        return this.context.transportClient.realtime.ackRealtimeEvents({ ackedSeq });
    }
}
