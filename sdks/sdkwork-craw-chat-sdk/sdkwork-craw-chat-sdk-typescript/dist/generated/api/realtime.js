import { backendApiPath } from './paths.js';
export class RealtimeApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Replace realtime subscriptions for the current device */
    async syncRealtimeSubscriptions(body) {
        return this.client.post(backendApiPath(`/realtime/subscriptions/sync`), body, undefined, undefined, 'application/json');
    }
    /** Pull realtime events for the current device */
    async listRealtimeEvents(params) {
        return this.client.get(backendApiPath(`/realtime/events`), params);
    }
    /** Ack realtime events for the current device */
    async ackRealtimeEvents(body) {
        return this.client.post(backendApiPath(`/realtime/events/ack`), body, undefined, undefined, 'application/json');
    }
}
export function createRealtimeApi(client) {
    return new RealtimeApi(client);
}
