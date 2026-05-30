import type { HttpClient } from '../http/client';
import type { AccessProviderHealthRetrieveResponse, ProtocolDownlinkCreateResponse, ProtocolProviderHealthRetrieveResponse, ProtocolUplinkCreateResponse } from '../types';
export declare class IotProtocolDownlinkApi {
    private client;
    constructor(client: HttpClient);
    /** Ingest IoT protocol downlink */
    create(): Promise<ProtocolDownlinkCreateResponse>;
}
export declare class IotProtocolUplinkApi {
    private client;
    constructor(client: HttpClient);
    /** Ingest IoT protocol uplink */
    create(): Promise<ProtocolUplinkCreateResponse>;
}
export declare class IotProtocolApi {
    private client;
    readonly uplink: IotProtocolUplinkApi;
    readonly downlink: IotProtocolDownlinkApi;
    constructor(client: HttpClient);
}
export declare class IotProtocolProviderHealthApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve IoT protocol provider health */
    retrieve(): Promise<ProtocolProviderHealthRetrieveResponse>;
}
export declare class IotAccessProviderHealthApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve IoT access provider health */
    retrieve(): Promise<AccessProviderHealthRetrieveResponse>;
}
export declare class IotApi {
    private client;
    readonly accessProviderHealth: IotAccessProviderHealthApi;
    readonly protocolProviderHealth: IotProtocolProviderHealthApi;
    readonly protocol: IotProtocolApi;
    constructor(client: HttpClient);
}
export declare function createIotApi(client: HttpClient): IotApi;
//# sourceMappingURL=iot.d.ts.map