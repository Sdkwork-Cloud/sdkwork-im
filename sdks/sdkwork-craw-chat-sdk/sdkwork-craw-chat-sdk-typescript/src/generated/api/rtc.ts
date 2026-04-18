import { backendApiPath } from './paths.js';
import type { HttpClient } from '../http/client.js';
import type { QueryParams } from '../types/common.js';
import type { CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, UpdateRtcSessionRequest } from '../types/index.js';


export class RtcApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Create an RTC session */
  async createRtcSession(body: CreateRtcSessionRequest): Promise<RtcSession> {
    return this.client.post<RtcSession>(backendApiPath(`/rtc/sessions`), body, undefined, undefined, 'application/json');
  }

/** Invite participants into an RTC session */
  async inviteRtcSession(rtcSessionId: string | number, body: InviteRtcSessionRequest): Promise<RtcSession> {
    return this.client.post<RtcSession>(backendApiPath(`/rtc/sessions/${rtcSessionId}/invite`), body, undefined, undefined, 'application/json');
  }

/** Accept an RTC session */
  async acceptRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession> {
    return this.client.post<RtcSession>(backendApiPath(`/rtc/sessions/${rtcSessionId}/accept`), body, undefined, undefined, 'application/json');
  }

/** Reject an RTC session */
  async rejectRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession> {
    return this.client.post<RtcSession>(backendApiPath(`/rtc/sessions/${rtcSessionId}/reject`), body, undefined, undefined, 'application/json');
  }

/** End an RTC session */
  async endRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession> {
    return this.client.post<RtcSession>(backendApiPath(`/rtc/sessions/${rtcSessionId}/end`), body, undefined, undefined, 'application/json');
  }

/** Post an RTC signaling event */
  async postRtcSignal(rtcSessionId: string | number, body: PostRtcSignalRequest): Promise<RtcSignalEvent> {
    return this.client.post<RtcSignalEvent>(backendApiPath(`/rtc/sessions/${rtcSessionId}/signals`), body, undefined, undefined, 'application/json');
  }

/** Issue an RTC participant credential */
  async issueRtcParticipantCredential(rtcSessionId: string | number, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential> {
    return this.client.post<RtcParticipantCredential>(backendApiPath(`/rtc/sessions/${rtcSessionId}/credentials`), body, undefined, undefined, 'application/json');
  }

/** Get the RTC recording artifact */
  async getRtcRecordingArtifact(rtcSessionId: string | number): Promise<RtcRecordingArtifact> {
    return this.client.get<RtcRecordingArtifact>(backendApiPath(`/rtc/sessions/${rtcSessionId}/artifacts/recording`));
  }
}

export function createRtcApi(client: HttpClient): RtcApi {
  return new RtcApi(client);
}
