import type { CrawChatInternalReceiverEvent } from './receiver-internal-types.js';
import type {
  CrawChatDataContext,
  CrawChatMessageContext,
  CrawChatReceiveContext,
  CrawChatReceiveSource,
  CrawChatSignalContext,
  CrawChatUnknownContext,
  RealtimeAckState,
} from './types.js';

export function toReceiveContext(
  event: CrawChatInternalReceiverEvent,
  source: CrawChatReceiveSource,
  ack: () => Promise<RealtimeAckState>,
): CrawChatReceiveContext {
  const base = {
    sequence: event.realtimeSeq,
    source,
    receivedAt: event.rawEvent.occurredAt,
    sender: {
      principalId: event.rawEvent.principalId,
      deviceId: event.rawEvent.deviceId,
    },
    eventType: event.eventType,
    scopeType: event.scopeType,
    scopeId: event.scopeId,
    payload: event.payload,
    rawEvent: event.rawEvent,
    ack,
  };

  switch (event.kind) {
    case 'message':
      return {
        kind: 'message',
        ...base,
        messageId: event.messageId,
        conversationId: event.conversationId,
        message: event.message,
      } satisfies CrawChatMessageContext;
    case 'data':
      return {
        kind: 'data',
        ...base,
        data: event.data,
      } satisfies CrawChatDataContext;
    case 'rtc_signal':
      return {
        kind: 'signal',
        ...base,
        signal: event.signal,
      } satisfies CrawChatSignalContext;
    default:
      return {
        kind: 'unknown',
        ...base,
      } satisfies CrawChatUnknownContext;
  }
}
