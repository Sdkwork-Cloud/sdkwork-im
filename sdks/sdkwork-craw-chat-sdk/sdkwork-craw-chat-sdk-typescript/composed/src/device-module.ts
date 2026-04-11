import type {
  DeviceSyncFeedResponse,
  QueryParams,
  RegisterDeviceRequest,
  RegisteredDeviceView,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatDevicesModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  register(body: RegisterDeviceRequest): Promise<RegisteredDeviceView> {
    return this.context.backendClient.device.register(body);
  }

  getSyncFeed(
    deviceId: string | number,
    params?: QueryParams,
  ): Promise<DeviceSyncFeedResponse> {
    return this.context.backendClient.device.getDeviceSyncFeed(deviceId, params);
  }
}
