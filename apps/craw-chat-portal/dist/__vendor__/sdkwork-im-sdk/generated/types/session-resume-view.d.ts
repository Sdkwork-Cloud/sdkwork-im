import type { PresenceSnapshotView } from './presence-snapshot-view.js';
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
//# sourceMappingURL=session-resume-view.d.ts.map