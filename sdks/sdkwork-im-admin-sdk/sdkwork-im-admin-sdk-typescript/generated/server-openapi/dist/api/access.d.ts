import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class AccessApi {
    private client;
    constructor(client: HttpClient);
    listApiKeys(): Promise<LooseJsonValue>;
    createApiKey(body: LooseJsonObject): Promise<LooseJsonValue>;
    updateApiKey(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteApiKey(hashedKey: string | number): Promise<LooseJsonValue>;
    updateApiKeyStatus(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    listApiKeyGroups(): Promise<LooseJsonValue>;
    createApiKeyGroup(body: LooseJsonObject): Promise<LooseJsonValue>;
    updateApiKeyGroup(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteApiKeyGroup(groupId: string | number): Promise<LooseJsonValue>;
    updateApiKeyGroupStatus(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare function createAccessApi(client: HttpClient): AccessApi;
//# sourceMappingURL=access.d.ts.map