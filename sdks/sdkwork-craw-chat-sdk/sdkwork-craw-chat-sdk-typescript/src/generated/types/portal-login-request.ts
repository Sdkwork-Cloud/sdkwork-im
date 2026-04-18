export interface PortalLoginRequest {
  tenantId: string;
  login: string;
  password: string;
  deviceId?: string;
  sessionId?: string;
  clientKind?: string;
}
