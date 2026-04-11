import type { PresenceSnapshotView } from './presence-snapshot-view';

export interface SessionResumeView {
  tenantId: string;
  actorId: string;
  actorKind: string;
  sessionId?: string;
  deviceId: string;
  resumeRequired: boolean;
  resumeFromSyncSeq: number;
  latestSyncSeq: number;
  resumedAt: string;
  presence: PresenceSnapshotView;
}
