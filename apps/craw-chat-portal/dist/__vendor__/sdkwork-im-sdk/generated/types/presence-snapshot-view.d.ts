import type { DevicePresenceView } from './device-presence-view.js';
export interface PresenceSnapshotView {
    tenantId: string;
    principalId: string;
    currentDeviceId?: string;
    devices: DevicePresenceView[];
}
//# sourceMappingURL=presence-snapshot-view.d.ts.map