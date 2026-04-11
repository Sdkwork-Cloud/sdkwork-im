import { buildJsonRtcSignalRequest } from './builders.js';
export class CrawChatRtcModule {
    context;
    constructor(context) {
        this.context = context;
    }
    create(body) {
        return this.context.backendClient.rtc.createRtcSession(body);
    }
    invite(rtcSessionId, body) {
        return this.context.backendClient.rtc.inviteRtcSession(rtcSessionId, body);
    }
    accept(rtcSessionId, body) {
        return this.context.backendClient.rtc.acceptRtcSession(rtcSessionId, body);
    }
    reject(rtcSessionId, body) {
        return this.context.backendClient.rtc.rejectRtcSession(rtcSessionId, body);
    }
    end(rtcSessionId, body) {
        return this.context.backendClient.rtc.endRtcSession(rtcSessionId, body);
    }
    postSignal(rtcSessionId, body) {
        return this.context.backendClient.rtc.postRtcSignal(rtcSessionId, body);
    }
    postJsonSignal(rtcSessionId, signalType, options) {
        return this.postSignal(rtcSessionId, buildJsonRtcSignalRequest(signalType, options));
    }
    issueParticipantCredential(rtcSessionId, body) {
        return this.context.backendClient.rtc.issueRtcParticipantCredential(rtcSessionId, body);
    }
    getRecordingArtifact(rtcSessionId) {
        return this.context.backendClient.rtc.getRtcRecordingArtifact(rtcSessionId);
    }
}
