export interface DeviceTwinView {
  tenantId: string;
  deviceId: string;
  desiredStateJson: string;
  reportedStateJson: string;
  updatedAt: string;
}
