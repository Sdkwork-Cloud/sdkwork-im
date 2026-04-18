import type { SdkworkBackendConfig } from '../types/common';
import type { RequestOptions, QueryParams } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { BaseHttpClient } from '@sdkwork/sdk-common';
type HttpRequestOptions = RequestOptions & {
    body?: unknown;
    headers?: Record<string, string>;
    contentType?: string;
};
export declare class HttpClient extends BaseHttpClient {
    private static readonly API_KEY_HEADER;
    private static readonly API_KEY_USE_BEARER;
    constructor(config: SdkworkBackendConfig);
    private getInternalAuthConfig;
    private getInternalHeaders;
    private buildRequestHeaders;
    setApiKey(apiKey: string): void;
    setAuthToken(token: string): void;
    setAccessToken(token: string): void;
    setTokenManager(manager: AuthTokenManager): void;
    request<T>(path: string, options?: HttpRequestOptions): Promise<T>;
    get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T>;
    post<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T>;
    put<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T>;
    delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T>;
    patch<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T>;
}
export declare function createHttpClient(config: SdkworkBackendConfig): HttpClient;
export {};
//# sourceMappingURL=client.d.ts.map