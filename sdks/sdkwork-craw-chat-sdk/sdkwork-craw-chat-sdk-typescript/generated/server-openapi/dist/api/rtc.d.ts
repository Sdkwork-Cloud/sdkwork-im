import type { HttpClient } from '../http/client';
import type { CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostRtcSignalRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, UpdateRtcSessionRequest } from '../types';
export declare class RtcApi {
    private client;
    constructor(client: HttpClient);
    /** Create an RTC session */
    createRtcSession(body: CreateRtcSessionRequest): Promise<RtcSession>;
    /** Invite participants into an RTC session */
    inviteRtcSession(rtcSessionId: string | number, body: InviteRtcSessionRequest): Promise<RtcSession>;
    /** Accept an RTC session */
    acceptRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
    /** Reject an RTC session */
    rejectRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
    /** End an RTC session */
    endRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
    /** Post an RTC signaling event */
    postRtcSignal(rtcSessionId: string | number, body: PostRtcSignalRequest): Promise<RtcSignalEvent>;
    /** Issue an RTC participant credential */
    issueRtcParticipantCredential(rtcSessionId: string | number, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential>;
    /** Get the RTC recording artifact */
    getRtcRecordingArtifact(rtcSessionId: string | number): Promise<RtcRecordingArtifact>;
}
export declare function createRtcApi(client: HttpClient): RtcApi;
//# sourceMappingURL=rtc.d.ts.map