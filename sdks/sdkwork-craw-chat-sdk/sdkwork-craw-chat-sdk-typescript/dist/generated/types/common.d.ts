export interface BasePlusVO {
    id?: string | number;
    createdAt?: string;
    updatedAt?: string;
    createdBy?: string;
    updatedBy?: string;
}
export interface BasePlusEntity extends BasePlusVO {
    deleted?: boolean;
}
export interface QueryListForm {
    keyword?: string;
    status?: string | number;
    startTime?: string;
    endTime?: string;
    orderBy?: string;
    orderDirection?: 'asc' | 'desc';
}
export type { Page, PageResult, RequestConfig, RequestOptions, QueryParams } from '@sdkwork/sdk-common';
export { DEFAULT_TIMEOUT, SUCCESS_CODES } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
export type { AuthTokenManager };
export interface SdkworkBackendConfig {
    baseUrl: string;
    authToken?: string;
    tokenManager?: AuthTokenManager;
    timeout?: number;
    headers?: Record<string, string>;
}
export type CrawChatSdkConfig = SdkworkBackendConfig;
//# sourceMappingURL=common.d.ts.map