import type { ImGeneratedConfig } from './generated-client-types.js';
import type { ImTransportClientLike } from './transport-client-like.js';
import type { ImSdkClientOptions, ImWebSocketFactory } from './types.js';
export declare const DEFAULT_REALTIME_WEBSOCKET_PATH = "api/v1/realtime/ws";
interface ImInternalTransportConfig {
    apiBaseUrl?: string;
    websocketBaseUrl?: string;
}
interface ImInternalResolvedSdkClientOptions {
    transportClient?: ImTransportClientLike;
    transportConfig?: ImGeneratedConfig;
    transport: ImInternalTransportConfig;
    authToken?: string;
    webSocketFactory?: ImWebSocketFactory;
}
interface ImInternalTransportClientOverrides {
    tokenManager?: ImGeneratedConfig['tokenManager'];
    timeout?: number;
    headers?: Record<string, string>;
}
type ImSdkClientRuntimeOptions = ImSdkClientOptions & ImInternalTransportClientOverrides & {
    transportClient?: ImTransportClientLike;
};
interface ImInternalSdkContextOptions {
    transportClient: ImTransportClientLike;
    transport?: ImInternalTransportConfig;
    authToken?: string;
    webSocketFactory?: ImWebSocketFactory;
}
export declare function createTransportClient(transportConfig: ImGeneratedConfig): ImTransportClientLike;
export declare function normalizeImSdkCreateOptions(options: ImSdkClientRuntimeOptions): ImInternalResolvedSdkClientOptions;
export declare function resolveTransportClient(options: ImSdkClientRuntimeOptions): ImTransportClientLike;
export declare function resolveImClientOptions(options: ImSdkClientRuntimeOptions): ImInternalSdkContextOptions;
export declare function resolveImWebSocketBaseUrl(baseUrl: string): string;
export declare class ImSdkContext {
    readonly transportClient: ImTransportClientLike;
    private readonly transport;
    readonly webSocketFactory?: ImInternalSdkContextOptions['webSocketFactory'];
    private authToken?;
    constructor(transportClient: ImTransportClientLike, transport?: ImInternalTransportConfig, webSocketFactory?: ImInternalSdkContextOptions['webSocketFactory'], initialAuthToken?: string);
    setAuthToken(token: string): void;
    clearAuthToken(): void;
    getAuthToken(): string | undefined;
    getApiBaseUrl(): string | undefined;
    getWebSocketBaseUrl(): string | undefined;
    resolveRealtimeWebSocketUrl(path?: string): string | undefined;
}
export {};
//# sourceMappingURL=sdk-context.d.ts.map