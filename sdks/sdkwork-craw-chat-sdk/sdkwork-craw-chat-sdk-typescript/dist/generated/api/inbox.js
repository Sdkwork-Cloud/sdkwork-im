import { backendApiPath } from './paths.js';
export class InboxApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Get inbox entries */
    async getInbox() {
        return this.client.get(backendApiPath(`/inbox`));
    }
}
export function createInboxApi(client) {
    return new InboxApi(client);
}
