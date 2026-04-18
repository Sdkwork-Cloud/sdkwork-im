import { backendApiPath } from './paths.js';
export class PortalApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant portal home snapshot */
    async getHome() {
        return this.client.get(backendApiPath(`/portal/home`));
    }
    /** Read the tenant portal sign-in snapshot */
    async getAuth() {
        return this.client.get(backendApiPath(`/portal/auth`));
    }
    /** Read the current tenant workspace snapshot */
    async getWorkspace() {
        return this.client.get(backendApiPath(`/portal/workspace`));
    }
    /** Read the tenant dashboard snapshot */
    async getDashboard() {
        return this.client.get(backendApiPath(`/portal/dashboard`));
    }
    /** Read the tenant conversations snapshot */
    async getConversations() {
        return this.client.get(backendApiPath(`/portal/conversations`));
    }
    /** Read the tenant realtime snapshot */
    async getRealtime() {
        return this.client.get(backendApiPath(`/portal/realtime`));
    }
    /** Read the tenant media snapshot */
    async getMedia() {
        return this.client.get(backendApiPath(`/portal/media`));
    }
    /** Read the tenant automation snapshot */
    async getAutomation() {
        return this.client.get(backendApiPath(`/portal/automation`));
    }
    /** Read the tenant governance snapshot */
    async getGovernance() {
        return this.client.get(backendApiPath(`/portal/governance`));
    }
}
export function createPortalApi(client) {
    return new PortalApi(client);
}
