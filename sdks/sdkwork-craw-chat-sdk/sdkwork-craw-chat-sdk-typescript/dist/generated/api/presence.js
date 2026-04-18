import { backendApiPath } from './paths.js';
export class PresenceApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Refresh device presence */
    async heartbeat(body) {
        return this.client.post(backendApiPath(`/presence/heartbeat`), body, undefined, undefined, 'application/json');
    }
    /** Get current presence */
    async getPresenceMe() {
        return this.client.get(backendApiPath(`/presence/me`));
    }
}
export function createPresenceApi(client) {
    return new PresenceApi(client);
}
