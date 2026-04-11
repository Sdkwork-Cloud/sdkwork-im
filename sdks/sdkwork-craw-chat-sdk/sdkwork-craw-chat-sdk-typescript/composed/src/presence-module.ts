import type {
  PresenceDeviceRequest,
  PresenceSnapshotView,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatPresenceModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  heartbeat(body: PresenceDeviceRequest): Promise<PresenceSnapshotView> {
    return this.context.backendClient.presence.heartbeat(body);
  }

  current(): Promise<PresenceSnapshotView> {
    return this.context.backendClient.presence.getPresenceMe();
  }
}
