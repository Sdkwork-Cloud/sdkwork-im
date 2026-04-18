import { backendApiPath } from './paths.js';
export class MessageApi {
    constructor(client) {
        this.client = client;
    }
    /** Edit a posted message */
    async edit(messageId, body) {
        return this.client.post(backendApiPath(`/messages/${messageId}/edit`), body, undefined, undefined, 'application/json');
    }
    /** Recall a posted message */
    async recall(messageId) {
        return this.client.post(backendApiPath(`/messages/${messageId}/recall`));
    }
}
export function createMessageApi(client) {
    return new MessageApi(client);
}
