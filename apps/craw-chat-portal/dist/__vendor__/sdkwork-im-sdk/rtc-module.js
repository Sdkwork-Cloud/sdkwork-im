import { buildJsonRtcSignalRequest } from './builders.js';
export class ImRtcModule {
    context;
    constructor(context) {
        this.context = context;
    }
    create(body) {
        return this.context.transportClient.rtc.createRtcSession(body);
    }
    invite(rtcSessionId, body) {
        return this.context.transportClient.rtc.inviteRtcSession(rtcSessionId, body);
    }
    accept(rtcSessionId, body) {
        return this.context.transportClient.rtc.acceptRtcSession(rtcSessionId, body);
    }
    reject(rtcSessionId, body) {
        return this.context.transportClient.rtc.rejectRtcSession(rtcSessionId, body);
    }
    end(rtcSessionId, body) {
        return this.context.transportClient.rtc.endRtcSession(rtcSessionId, body);
    }
    postSignal(rtcSessionId, body) {
        return this.context.transportClient.rtc.postRtcSignal(rtcSessionId, body);
    }
    postJsonSignal(rtcSessionId, signalType, options) {
        return this.postSignal(rtcSessionId, buildJsonRtcSignalRequest(signalType, options));
    }
    issueParticipantCredential(rtcSessionId, body) {
        return this.context.transportClient.rtc.issueRtcParticipantCredential(rtcSessionId, body);
    }
    getRecordingArtifact(rtcSessionId) {
        return this.context.transportClient.rtc.getRtcRecordingArtifact(rtcSessionId);
    }
}
