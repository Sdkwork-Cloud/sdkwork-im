import { decodeRealtimeEvent } from './message-codec.js';
import type { CrawChatRealtimeModule } from './realtime-module.js';
import type {
  CrawChatInternalRealtimeBatch,
  CrawChatInternalReceiverDataEvent,
  CrawChatInternalReceiverEvent,
  CrawChatInternalReceiverMessageEvent,
  CrawChatInternalReceiverPullAckResult,
  CrawChatInternalReceiverRtcSignalEvent,
} from './receiver-internal-types.js';
import type {
  CrawChatSubscription,
  QueryParams,
  RealtimeAckState,
  RealtimeEvent,
} from './types.js';

export class CrawChatReceiver {
  private readonly anyHandlers = new Set<(event: CrawChatInternalReceiverEvent) => void>();
  private readonly messageEventHandlers = new Set<
    (event: CrawChatInternalReceiverMessageEvent) => void
  >();
  private readonly dataEventHandlers = new Set<
    (event: CrawChatInternalReceiverDataEvent) => void
  >();
  private readonly rtcSignalEventHandlers = new Set<
    (event: CrawChatInternalReceiverRtcSignalEvent) => void
  >();

  constructor(private readonly realtime: CrawChatRealtimeModule) {}

  onEvent(handler: (event: CrawChatInternalReceiverEvent) => void): CrawChatSubscription {
    this.anyHandlers.add(handler);
    return () => {
      this.anyHandlers.delete(handler);
    };
  }

  onMessageEvent(
    handler: (event: CrawChatInternalReceiverMessageEvent) => void,
  ): CrawChatSubscription {
    this.messageEventHandlers.add(handler);
    return () => {
      this.messageEventHandlers.delete(handler);
    };
  }

  onDataEvent(
    handler: (event: CrawChatInternalReceiverDataEvent) => void,
  ): CrawChatSubscription {
    this.dataEventHandlers.add(handler);
    return () => {
      this.dataEventHandlers.delete(handler);
    };
  }

  onRtcSignalEvent(
    handler: (event: CrawChatInternalReceiverRtcSignalEvent) => void,
  ): CrawChatSubscription {
    this.rtcSignalEventHandlers.add(handler);
    return () => {
      this.rtcSignalEventHandlers.delete(handler);
    };
  }

  onScope(
    scopeType: string,
    scopeId: string | number,
    handler: (event: CrawChatInternalReceiverEvent) => void,
  ): CrawChatSubscription {
    const normalizedScopeId = String(scopeId);
    return this.onEvent((event) => {
      if (event.scopeType === scopeType && event.scopeId === normalizedScopeId) {
        handler(event);
      }
    });
  }

  dispatchRealtimeEvent(
    event: Parameters<typeof decodeRealtimeEvent>[0],
  ): CrawChatInternalReceiverEvent {
    const decoded = decodeRealtimeEvent(event);

    for (const handler of this.anyHandlers) {
      handler(decoded);
    }

    if (decoded.kind === 'message') {
      for (const handler of this.messageEventHandlers) {
        handler(decoded);
      }
    } else if (decoded.kind === 'data') {
      for (const handler of this.dataEventHandlers) {
        handler(decoded);
      }
    } else if (decoded.kind === 'rtc_signal') {
      for (const handler of this.rtcSignalEventHandlers) {
        handler(decoded);
      }
    }

    return decoded;
  }

  async pull(params?: QueryParams): Promise<CrawChatInternalRealtimeBatch> {
    const rawWindow = await this.realtime.pullEvents(params);
    const items = rawWindow.items.map((item: RealtimeEvent) =>
      this.dispatchRealtimeEvent(item),
    );
    const highestSeq = items.reduce(
      (currentMax: number, item: CrawChatInternalReceiverEvent) =>
        Math.max(currentMax, item.realtimeSeq),
      rawWindow.ackedThroughSeq ?? 0,
    );

    return {
      items,
      highestSeq,
      rawWindow,
    };
  }

  ack(batchOrSeq: CrawChatInternalRealtimeBatch | number): Promise<RealtimeAckState> {
    const ackedSeq =
      typeof batchOrSeq === 'number' ? batchOrSeq : batchOrSeq.highestSeq;
    return this.realtime.ackEvents({ ackedSeq });
  }

  async pullAndAck(
    params?: QueryParams,
  ): Promise<CrawChatInternalReceiverPullAckResult> {
    const batch = await this.pull(params);
    const ack = batch.highestSeq > 0 ? await this.ack(batch) : undefined;

    return {
      batch,
      ack,
    };
  }
}
