import { backendApiPath } from './paths.js';
export class SessionApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Resume the current app session */
    async resume(body) {
        return this.client.post(backendApiPath(`/sessions/resume`), body, undefined, undefined, 'application/json');
    }
    /** Disconnect the current app session device route */
    async disconnect(body) {
        return this.client.post(backendApiPath(`/sessions/disconnect`), body, undefined, undefined, 'application/json');
    }
}
export function createSessionApi(client) {
    return new SessionApi(client);
}
