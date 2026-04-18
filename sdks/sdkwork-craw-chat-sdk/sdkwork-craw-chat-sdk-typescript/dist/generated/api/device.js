import { backendApiPath } from './paths.js';
export class DeviceApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Register the current device */
    async register(body) {
        return this.client.post(backendApiPath(`/devices/register`), body, undefined, undefined, 'application/json');
    }
    /** Get device sync feed entries */
    async getDeviceSyncFeed(deviceId, params) {
        return this.client.get(backendApiPath(`/devices/${deviceId}/sync-feed`), params);
    }
}
export function createDeviceApi(client) {
    return new DeviceApi(client);
}
