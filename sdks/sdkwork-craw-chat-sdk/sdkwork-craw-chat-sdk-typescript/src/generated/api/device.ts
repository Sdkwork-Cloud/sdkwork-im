import { backendApiPath } from './paths.js';
import type { HttpClient } from '../http/client.js';
import type { QueryParams } from '../types/common.js';
import type { DeviceSyncFeedResponse, RegisterDeviceRequest, RegisteredDeviceView } from '../types/index.js';


export class DeviceApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Register the current device */
  async register(body: RegisterDeviceRequest): Promise<RegisteredDeviceView> {
    return this.client.post<RegisteredDeviceView>(backendApiPath(`/devices/register`), body, undefined, undefined, 'application/json');
  }

/** Get device sync feed entries */
  async getDeviceSyncFeed(deviceId: string | number, params?: QueryParams): Promise<DeviceSyncFeedResponse> {
    return this.client.get<DeviceSyncFeedResponse>(backendApiPath(`/devices/${deviceId}/sync-feed`), params);
  }
}

export function createDeviceApi(client: HttpClient): DeviceApi {
  return new DeviceApi(client);
}
