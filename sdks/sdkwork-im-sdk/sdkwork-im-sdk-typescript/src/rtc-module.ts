import type { RtcSession } from '@sdkwork/im-sdk-generated';
import type { ImTransportClientLike } from './transport-client-like';

export class ImRtcModule {
  constructor(private readonly transportClient: ImTransportClientLike) {}

  retrieve(rtcSessionId: string | number): Promise<RtcSession> {
    return this.transportClient.rtc.sessions.retrieve(rtcSessionId);
  }
}
