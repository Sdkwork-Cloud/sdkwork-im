import type {
  DeviceSyncFeedResponse,
  QueryParams,
  RegisterDeviceRequest,
  RegisteredDeviceView,
} from '@sdkwork/im-sdk-generated';
import type { ImTransportClientLike } from './transport-client-like';

export class ImDeviceModule {
  readonly registrations = {
    create: (body: RegisterDeviceRequest): Promise<RegisteredDeviceView> =>
      this.transportClient.device.registrations.create(body),
  };

  readonly syncFeed = {
    retrieve: (
      deviceId: string | number,
      params?: QueryParams & { afterSeq?: number; limit?: number },
    ): Promise<DeviceSyncFeedResponse> =>
      this.transportClient.device.syncFeed.retrieve(deviceId, params),
  };

  constructor(private readonly transportClient: ImTransportClientLike) {}
}
