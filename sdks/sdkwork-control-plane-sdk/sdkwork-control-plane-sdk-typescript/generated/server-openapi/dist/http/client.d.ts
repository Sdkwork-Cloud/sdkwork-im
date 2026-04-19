import type { ControlPlaneBackendConfig, JsonObject, QueryParams } from '../types/common.js';
type HttpRequestOptions = {
    method?: string;
    params?: QueryParams;
    body?: JsonObject;
    headers?: Record<string, string>;
};
export declare class HttpClient {
    private readonly config;
    constructor(config: ControlPlaneBackendConfig);
    setAuthToken(token: string): void;
    private buildUrl;
    private buildHeaders;
    private parsePayload;
    request<T extends JsonObject>(path: string, options?: HttpRequestOptions): Promise<T>;
    get<T extends JsonObject>(path: string, params?: QueryParams): Promise<T>;
    post<T extends JsonObject>(path: string, body?: JsonObject, params?: QueryParams): Promise<T>;
}
export declare function createHttpClient(config: ControlPlaneBackendConfig): HttpClient;
export {};
//# sourceMappingURL=client.d.ts.map