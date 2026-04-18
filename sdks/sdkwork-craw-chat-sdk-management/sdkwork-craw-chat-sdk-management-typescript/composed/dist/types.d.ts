import type { SdkworkBackendClient, SdkworkBackendConfig } from '@sdkwork/craw-chat-management-backend-sdk';
export type { SdkworkBackendConfig } from '@sdkwork/craw-chat-management-backend-sdk';
export interface CrawChatSdkManagementClientCreateOptions {
    backendClient?: CrawChatManagementBackendClientLike;
    backendConfig?: SdkworkBackendConfig;
}
export interface CrawChatSdkManagementClientOptions {
    backendClient: CrawChatManagementBackendClientLike;
}
export type CrawChatManagementBackendClientLike = Pick<SdkworkBackendClient, 'auth' | 'users' | 'marketing' | 'tenants' | 'access' | 'routing' | 'catalog' | 'usage' | 'billing' | 'operations' | 'setApiKey' | 'setAuthToken' | 'setAccessToken' | 'setTokenManager' | 'http'>;
//# sourceMappingURL=types.d.ts.map