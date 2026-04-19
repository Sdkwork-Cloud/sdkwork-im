import { apiPath } from './paths.js';
export class RtcApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Create an RTC session */
    async createRtcSession(body) {
        return this.client.post(apiPath(`/rtc/sessions`), body, undefined, undefined, 'application/json');
    }
    /** Invite participants into an RTC session */
    async inviteRtcSession(rtcSessionId, body) {
        return this.client.post(apiPath(`/rtc/sessions/${rtcSessionId}/invite`), body, undefined, undefined, 'application/json');
    }
    /** Accept an RTC session */
    async acceptRtcSession(rtcSessionId, body) {
        return this.client.post(apiPath(`/rtc/sessions/${rtcSessionId}/accept`), body, undefined, undefined, 'application/json');
    }
    /** Reject an RTC session */
    async rejectRtcSession(rtcSessionId, body) {
        return this.client.post(apiPath(`/rtc/sessions/${rtcSessionId}/reject`), body, undefined, undefined, 'application/json');
    }
    /** End an RTC session */
    async endRtcSession(rtcSessionId, body) {
        return this.client.post(apiPath(`/rtc/sessions/${rtcSessionId}/end`), body, undefined, undefined, 'application/json');
    }
    /** Post an RTC signaling event */
    async postRtcSignal(rtcSessionId, body) {
        return this.client.post(apiPath(`/rtc/sessions/${rtcSessionId}/signals`), body, undefined, undefined, 'application/json');
    }
    /** Issue an RTC participant credential */
    async issueRtcParticipantCredential(rtcSessionId, body) {
        return this.client.post(apiPath(`/rtc/sessions/${rtcSessionId}/credentials`), body, undefined, undefined, 'application/json');
    }
    /** Get the RTC recording artifact */
    async getRtcRecordingArtifact(rtcSessionId) {
        return this.client.get(apiPath(`/rtc/sessions/${rtcSessionId}/artifacts/recording`));
    }
}
export function createRtcApi(client) {
    return new RtcApi(client);
}
