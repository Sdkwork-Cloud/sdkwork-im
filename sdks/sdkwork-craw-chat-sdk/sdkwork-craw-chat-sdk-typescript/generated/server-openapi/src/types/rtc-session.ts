import type { RtcSessionState } from './rtc-session-state';

export interface RtcSession {
  tenantId: string;
  rtcSessionId: string;
  conversationId?: string;
  rtcMode: string;
  initiatorId: string;
  providerPluginId?: string;
  providerSessionId?: string;
  accessEndpoint?: string;
  providerRegion?: string;
  state: RtcSessionState;
  signalingStreamId?: string;
  artifactMessageId?: string;
  startedAt: string;
  endedAt?: string;
}
