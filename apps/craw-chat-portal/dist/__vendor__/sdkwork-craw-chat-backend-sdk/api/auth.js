import { backendApiPath } from './paths.js';
export class AuthApi {
    constructor(client) {
        this.client = client;
    }
    /** Sign in to the tenant portal */
    async login(body) {
        return this.client.post(backendApiPath(`/auth/login`), body, undefined, undefined, 'application/json');
    }
    /** Read the current portal session */
    async me() {
        return this.client.get(backendApiPath(`/auth/me`));
    }
}
export function createAuthApi(client) {
    return new AuthApi(client);
}
