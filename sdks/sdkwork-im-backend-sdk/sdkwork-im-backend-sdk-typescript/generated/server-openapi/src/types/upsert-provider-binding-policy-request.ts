export interface UpsertProviderBindingPolicyRequest {
  domain: string;
  expectedBaseVersion?: number | null;
  pluginId: string;
  tenantId?: string | null;
}
