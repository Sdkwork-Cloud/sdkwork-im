import type { HttpClient } from '../http/client.js';
import type { InboxResponse } from '../types/index.js';
export declare class InboxApi {
    private client;
    constructor(client: HttpClient);
    /** Get inbox entries */
    getInbox(): Promise<InboxResponse>;
}
export declare function createInboxApi(client: HttpClient): InboxApi;
//# sourceMappingURL=inbox.d.ts.map