import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { DeviceSyncFeedResponse, RegisterDeviceRequest, RegisteredDeviceView } from '../types';
export declare class DeviceApi {
    private client;
    constructor(client: HttpClient);
    /** Register the current device */
    register(body: RegisterDeviceRequest): Promise<RegisteredDeviceView>;
    /** Get device sync feed entries */
    getDeviceSyncFeed(deviceId: string | number, params?: QueryParams): Promise<DeviceSyncFeedResponse>;
}
export declare function createDeviceApi(client: HttpClient): DeviceApi;
//# sourceMappingURL=device.d.ts.map