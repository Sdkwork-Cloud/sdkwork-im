import type { ImAuthLoginRequest, ImAuthLoginResult, ImAuthSession } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImAuthModule {
    private readonly context;
    constructor(context: ImSdkContext);
    login(body: ImAuthLoginRequest): Promise<ImAuthLoginResult>;
    me(): Promise<ImAuthSession>;
    useToken(token: string): void;
    clearToken(): void;
}
//# sourceMappingURL=auth-module.d.ts.map