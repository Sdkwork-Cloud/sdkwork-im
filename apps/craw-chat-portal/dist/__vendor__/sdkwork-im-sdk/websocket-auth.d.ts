import type { ImWebSocketAuthMode, ImWebSocketAuthOptions as ImWebSocketAuthOptionsShape } from './types.js';
export interface ResolvedImWebSocketAuthOptions {
    mode: ImWebSocketAuthMode;
    headerName: string;
    queryParameterName: string;
    scheme: string;
    credentialProvider?: ImWebSocketAuthOptionsShape['credentialProvider'];
}
export declare function normalizeImWebSocketAuthOptions(value?: ImWebSocketAuthOptionsShape): ResolvedImWebSocketAuthOptions;
export declare function resolveAutomaticImWebSocketAuthMode({ hasSocket, hasWebSocketFactory, }?: {
    hasSocket?: boolean;
    hasWebSocketFactory?: boolean;
}): Exclude<ImWebSocketAuthMode, 'automatic'>;
//# sourceMappingURL=websocket-auth.d.ts.map