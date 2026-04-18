import type { Sender } from './sender.js';

export interface RtcSignalEvent {
  tenantId: string;
  rtcSessionId: string;
  conversationId?: string;
  rtcMode: string;
  signalType: string;
  schemaRef?: string;
  payload: string;
  sender: Sender;
  signalingStreamId?: string;
  occurredAt: string;
}
