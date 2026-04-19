import type { HttpClient } from '../http/client.js';
import type { EditMessageRequest, MessageMutationResult } from '../types/index.js';
export declare class MessageApi {
    private client;
    constructor(client: HttpClient);
    /** Edit a posted message */
    edit(messageId: string | number, body: EditMessageRequest): Promise<MessageMutationResult>;
    /** Recall a posted message */
    recall(messageId: string | number): Promise<MessageMutationResult>;
}
export declare function createMessageApi(client: HttpClient): MessageApi;
//# sourceMappingURL=message.d.ts.map