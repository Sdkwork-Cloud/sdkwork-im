import { decodeRealtimeEvent } from './message-codec.js';
export class CrawChatReceiver {
    realtime;
    anyHandlers = new Set();
    messageEventHandlers = new Set();
    dataEventHandlers = new Set();
    rtcSignalEventHandlers = new Set();
    constructor(realtime) {
        this.realtime = realtime;
    }
    onEvent(handler) {
        this.anyHandlers.add(handler);
        return () => {
            this.anyHandlers.delete(handler);
        };
    }
    onMessageEvent(handler) {
        this.messageEventHandlers.add(handler);
        return () => {
            this.messageEventHandlers.delete(handler);
        };
    }
    onDataEvent(handler) {
        this.dataEventHandlers.add(handler);
        return () => {
            this.dataEventHandlers.delete(handler);
        };
    }
    onRtcSignalEvent(handler) {
        this.rtcSignalEventHandlers.add(handler);
        return () => {
            this.rtcSignalEventHandlers.delete(handler);
        };
    }
    onScope(scopeType, scopeId, handler) {
        const normalizedScopeId = String(scopeId);
        return this.onEvent((event) => {
            if (event.scopeType === scopeType && event.scopeId === normalizedScopeId) {
                handler(event);
            }
        });
    }
    dispatchRealtimeEvent(event) {
        const decoded = decodeRealtimeEvent(event);
        for (const handler of this.anyHandlers) {
            handler(decoded);
        }
        if (decoded.kind === 'message') {
            for (const handler of this.messageEventHandlers) {
                handler(decoded);
            }
        }
        else if (decoded.kind === 'data') {
            for (const handler of this.dataEventHandlers) {
                handler(decoded);
            }
        }
        else if (decoded.kind === 'rtc_signal') {
            for (const handler of this.rtcSignalEventHandlers) {
                handler(decoded);
            }
        }
        return decoded;
    }
    async pull(params) {
        const rawWindow = await this.realtime.pullEvents(params);
        const items = rawWindow.items.map((item) => this.dispatchRealtimeEvent(item));
        const highestSeq = items.reduce((currentMax, item) => Math.max(currentMax, item.realtimeSeq), rawWindow.ackedThroughSeq ?? 0);
        return {
            items,
            highestSeq,
            rawWindow,
        };
    }
    ack(batchOrSeq) {
        const ackedSeq = typeof batchOrSeq === 'number' ? batchOrSeq : batchOrSeq.highestSeq;
        return this.realtime.ackEvents({ ackedSeq });
    }
    async pullAndAck(params) {
        const batch = await this.pull(params);
        const ack = batch.highestSeq > 0 ? await this.ack(batch) : undefined;
        return {
            batch,
            ack,
        };
    }
}
