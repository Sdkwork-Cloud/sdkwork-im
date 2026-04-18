import type { SdkworkBackendClient } from '@sdkwork/craw-chat-admin-backend-sdk';
import type { SdkworkBackendConfig } from './generated-backend-types.js';
export type { SdkworkBackendConfig } from './generated-backend-types.js';
export interface CrawChatSdkAdminClientCreateOptions {
    backendClient?: CrawChatAdminBackendClientLike;
    backendConfig?: SdkworkBackendConfig;
}
export interface CrawChatSdkAdminClientOptions {
    backendClient: CrawChatAdminBackendClientLike;
}
export type CrawChatAdminBackendClientLike = Pick<SdkworkBackendClient, 'cluster' | 'protocol' | 'providers' | 'social' | 'system' | 'setApiKey' | 'setAuthToken' | 'setAccessToken' | 'setTokenManager' | 'http'>;
//# sourceMappingURL=types.d.ts.map