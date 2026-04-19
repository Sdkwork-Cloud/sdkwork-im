import type { CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, PostJsonRtcSignalOptions, PostRtcSignalRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, UpdateRtcSessionRequest } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImRtcModule {
    private readonly context;
    constructor(context: ImSdkContext);
    create(body: CreateRtcSessionRequest): Promise<RtcSession>;
    invite(rtcSessionId: string | number, body: InviteRtcSessionRequest): Promise<RtcSession>;
    accept(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
    reject(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
    end(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
    postSignal(rtcSessionId: string | number, body: PostRtcSignalRequest): Promise<RtcSignalEvent>;
    postJsonSignal(rtcSessionId: string | number, signalType: string, options: PostJsonRtcSignalOptions): Promise<RtcSignalEvent>;
    issueParticipantCredential(rtcSessionId: string | number, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential>;
    getRecordingArtifact(rtcSessionId: string | number): Promise<RtcRecordingArtifact>;
}
//# sourceMappingURL=rtc-module.d.ts.map