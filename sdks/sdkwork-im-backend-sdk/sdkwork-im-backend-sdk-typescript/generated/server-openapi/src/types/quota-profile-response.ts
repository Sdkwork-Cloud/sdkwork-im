export interface QuotaProfileResponse {
  maxConcurrentSessionsPerTenant: number;
  maxInflightMessages: number;
  maxPayloadBytes: number;
  maxSubscriptionsPerSession: number;
  profileId: string;
}
