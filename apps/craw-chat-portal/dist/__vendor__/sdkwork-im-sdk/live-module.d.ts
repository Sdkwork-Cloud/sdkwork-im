import type { ImConnectOptions, ImLiveConnection } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImLiveModule {
    private readonly context;
    constructor(context: ImSdkContext);
    connect(options?: ImConnectOptions): Promise<ImLiveConnection>;
}
//# sourceMappingURL=live-module.d.ts.map