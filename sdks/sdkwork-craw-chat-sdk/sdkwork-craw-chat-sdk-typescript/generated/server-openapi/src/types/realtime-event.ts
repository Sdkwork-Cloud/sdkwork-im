export interface RealtimeEvent {
  tenantId: string;
  principalId: string;
  deviceId: string;
  realtimeSeq: number;
  scopeType: string;
  scopeId: string;
  eventType: string;
  deliveryClass: string;
  payload: string;
  occurredAt: string;
}
