import type { DeviceSyncFeedResponse, QueryParams, RegisterDeviceRequest, RegisteredDeviceView } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatDevicesModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    register(body: RegisterDeviceRequest): Promise<RegisteredDeviceView>;
    getSyncFeed(deviceId: string | number, params?: QueryParams): Promise<DeviceSyncFeedResponse>;
}
//# sourceMappingURL=device-module.d.ts.map