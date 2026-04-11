import { buildJsonRtcSignalRequest } from './builders.js';
import type {
  CreateRtcSessionRequest,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  PostJsonRtcSignalOptions,
  PostRtcSignalRequest,
  RtcParticipantCredential,
  RtcRecordingArtifact,
  RtcSession,
  RtcSignalEvent,
  UpdateRtcSessionRequest,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatRtcModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  create(body: CreateRtcSessionRequest): Promise<RtcSession> {
    return this.context.backendClient.rtc.createRtcSession(body);
  }

  invite(
    rtcSessionId: string | number,
    body: InviteRtcSessionRequest,
  ): Promise<RtcSession> {
    return this.context.backendClient.rtc.inviteRtcSession(rtcSessionId, body);
  }

  accept(
    rtcSessionId: string | number,
    body: UpdateRtcSessionRequest,
  ): Promise<RtcSession> {
    return this.context.backendClient.rtc.acceptRtcSession(rtcSessionId, body);
  }

  reject(
    rtcSessionId: string | number,
    body: UpdateRtcSessionRequest,
  ): Promise<RtcSession> {
    return this.context.backendClient.rtc.rejectRtcSession(rtcSessionId, body);
  }

  end(
    rtcSessionId: string | number,
    body: UpdateRtcSessionRequest,
  ): Promise<RtcSession> {
    return this.context.backendClient.rtc.endRtcSession(rtcSessionId, body);
  }

  postSignal(
    rtcSessionId: string | number,
    body: PostRtcSignalRequest,
  ): Promise<RtcSignalEvent> {
    return this.context.backendClient.rtc.postRtcSignal(rtcSessionId, body);
  }

  postJsonSignal(
    rtcSessionId: string | number,
    signalType: string,
    options: PostJsonRtcSignalOptions,
  ): Promise<RtcSignalEvent> {
    return this.postSignal(
      rtcSessionId,
      buildJsonRtcSignalRequest(signalType, options),
    );
  }

  issueParticipantCredential(
    rtcSessionId: string | number,
    body: IssueRtcParticipantCredentialRequest,
  ): Promise<RtcParticipantCredential> {
    return this.context.backendClient.rtc.issueRtcParticipantCredential(
      rtcSessionId,
      body,
    );
  }

  getRecordingArtifact(
    rtcSessionId: string | number,
  ): Promise<RtcRecordingArtifact> {
    return this.context.backendClient.rtc.getRtcRecordingArtifact(rtcSessionId);
  }
}
