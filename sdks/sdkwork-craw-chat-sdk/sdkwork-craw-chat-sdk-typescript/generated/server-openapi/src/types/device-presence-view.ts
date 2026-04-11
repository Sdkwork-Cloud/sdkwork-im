import type { DevicePresenceStatus } from './device-presence-status';

export interface DevicePresenceView {
  tenantId: string;
  principalId: string;
  deviceId: string;
  platform?: string;
  sessionId?: string;
  status: DevicePresenceStatus;
  lastSyncSeq: number;
  lastResumeAt?: string;
  lastSeenAt?: string;
}
