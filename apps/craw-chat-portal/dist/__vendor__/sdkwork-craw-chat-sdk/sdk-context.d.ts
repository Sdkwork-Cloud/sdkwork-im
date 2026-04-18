import type { CrawChatBackendClientLike } from './backend-client-like.js';
import type { CrawChatSdkClientOptions, CrawChatWebSocketFactory, SdkworkBackendConfig } from './types.js';
export declare const DEFAULT_REALTIME_WEBSOCKET_PATH = "api/v1/realtime/ws";
interface CrawChatInternalTransportConfig {
    apiBaseUrl?: string;
    websocketBaseUrl?: string;
}
interface CrawChatInternalResolvedSdkClientOptions {
    backendClient?: CrawChatBackendClientLike;
    backendConfig?: SdkworkBackendConfig;
    transport: CrawChatInternalTransportConfig;
    authToken?: string;
    webSocketFactory?: CrawChatWebSocketFactory;
}
interface CrawChatInternalGeneratedTransportOverrides {
    tokenManager?: SdkworkBackendConfig['tokenManager'];
    timeout?: number;
    headers?: Record<string, string>;
}
type CrawChatSdkClientRuntimeOptions = CrawChatSdkClientOptions & CrawChatInternalGeneratedTransportOverrides & {
    backendClient?: CrawChatBackendClientLike;
};
interface CrawChatInternalSdkContextOptions {
    backendClient: CrawChatBackendClientLike;
    transport?: CrawChatInternalTransportConfig;
    authToken?: string;
    webSocketFactory?: CrawChatWebSocketFactory;
}
export declare function createGeneratedBackendClient(backendConfig: SdkworkBackendConfig): CrawChatBackendClientLike;
export declare function normalizeCrawChatSdkCreateOptions(options: CrawChatSdkClientRuntimeOptions): CrawChatInternalResolvedSdkClientOptions;
export declare function resolveBackendClient(options: CrawChatSdkClientRuntimeOptions): CrawChatBackendClientLike;
export declare function resolveCrawChatClientOptions(options: CrawChatSdkClientRuntimeOptions): CrawChatInternalSdkContextOptions;
export declare function resolveCrawChatWebSocketBaseUrl(baseUrl: string): string;
export declare class CrawChatSdkContext {
    readonly backendClient: CrawChatBackendClientLike;
    private readonly transport;
    readonly webSocketFactory?: CrawChatInternalSdkContextOptions['webSocketFactory'];
    private authToken?;
    constructor(backendClient: CrawChatBackendClientLike, transport?: CrawChatInternalTransportConfig, webSocketFactory?: CrawChatInternalSdkContextOptions['webSocketFactory'], initialAuthToken?: string);
    setAuthToken(token: string): void;
    clearAuthToken(): void;
    getAuthToken(): string | undefined;
    getApiBaseUrl(): string | undefined;
    getWebSocketBaseUrl(): string | undefined;
    resolveRealtimeWebSocketUrl(path?: string): string | undefined;
}
export {};
//# sourceMappingURL=sdk-context.d.ts.map