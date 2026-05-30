import type { HttpClient } from '../http/client';
import type { DeviceTwinView, UpdateDeviceTwinDesiredRequest, UpdateDeviceTwinReportedRequest } from '../types';
export declare class DeviceTwinReportedApi {
    private client;
    constructor(client: HttpClient);
    /** Update the reported state for a device twin */
    create(deviceId: string, body: UpdateDeviceTwinReportedRequest): Promise<DeviceTwinView>;
}
export declare class DeviceTwinDesiredApi {
    private client;
    constructor(client: HttpClient);
    /** Update the desired state for a device twin */
    create(deviceId: string, body: UpdateDeviceTwinDesiredRequest): Promise<DeviceTwinView>;
}
export declare class DeviceTwinApi {
    private client;
    readonly desired: DeviceTwinDesiredApi;
    readonly reported: DeviceTwinReportedApi;
    constructor(client: HttpClient);
    /** Get the device twin */
    list(deviceId: string): Promise<DeviceTwinView>;
}
export declare class DeviceApi {
    private client;
    readonly twin: DeviceTwinApi;
    constructor(client: HttpClient);
}
export declare function createDeviceApi(client: HttpClient): DeviceApi;
//# sourceMappingURL=device.d.ts.map