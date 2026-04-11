import type {
  PresenceDeviceRequest,
  PresenceSnapshotView,
  ResumeSessionRequest,
  SessionResumeView,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatSessionModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  resume(body: ResumeSessionRequest): Promise<SessionResumeView> {
    return this.context.backendClient.session.resume(body);
  }

  disconnectDevice(body: PresenceDeviceRequest): Promise<PresenceSnapshotView> {
    return this.context.backendClient.session.disconnect(body);
  }
}
