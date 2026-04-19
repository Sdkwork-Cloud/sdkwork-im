import { apiPath } from './paths.js';
export class AuthApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Sign in to the tenant portal */
    async login(body) {
        return this.client.post(apiPath(`/auth/login`), body, undefined, undefined, 'application/json');
    }
    /** Read the current portal session */
    async me() {
        return this.client.get(apiPath(`/auth/me`));
    }
}
export function createAuthApi(client) {
    return new AuthApi(client);
}
