import { decodeRealtimeEvent } from './message-codec.js';
import { toReceiveContext } from './receive-context.js';
import type {
  CrawChatCatchUpBatch,
  QueryParams,
  RealtimeAckState,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatSyncModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  async catchUp(params: QueryParams = {}): Promise<CrawChatCatchUpBatch> {
    const rawWindow = await this.context.backendClient.realtime.listRealtimeEvents(params);
    const items = rawWindow.items.map((item) => {
      const decoded = decodeRealtimeEvent(item);
      return toReceiveContext(decoded, 'catch_up', () => this.ack(decoded.realtimeSeq));
    });
    const highestSequence = items.reduce(
      (currentMax, item) => Math.max(currentMax, item.sequence),
      rawWindow.ackedThroughSeq ?? 0,
    );

    return {
      items,
      highestSequence,
      rawWindow,
    };
  }

  ack(batchOrSequence: CrawChatCatchUpBatch | number): Promise<RealtimeAckState> {
    const ackedSeq =
      typeof batchOrSequence === 'number'
        ? batchOrSequence
        : batchOrSequence.highestSequence;
    return this.context.backendClient.realtime.ackRealtimeEvents({ ackedSeq });
  }
}
