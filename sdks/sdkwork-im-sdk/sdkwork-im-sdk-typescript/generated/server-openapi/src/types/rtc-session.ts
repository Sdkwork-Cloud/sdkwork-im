export interface RtcSession {
  tenantId: string;
  rtcSessionId: string;
  conversationId?: string | null;
  providerPluginId?: string | null;
  providerSessionId?: string | null;
  rtcMode: string;
  state: string;
  createdAt: string;
  updatedAt: string;
}
