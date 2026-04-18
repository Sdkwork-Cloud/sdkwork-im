import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class UsersApi {
    private client;
    constructor(client: HttpClient);
    listOperatorUsers(): Promise<LooseJsonValue>;
    saveOperatorUser(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteOperatorUser(userId: string | number): Promise<LooseJsonValue>;
    updateOperatorUserStatus(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    resetOperatorUserPassword(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    listPortalUsers(): Promise<LooseJsonValue>;
    savePortalUser(body: LooseJsonObject): Promise<LooseJsonValue>;
    deletePortalUser(userId: string | number): Promise<LooseJsonValue>;
    updatePortalUserStatus(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    resetPortalUserPassword(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare function createUsersApi(client: HttpClient): UsersApi;
//# sourceMappingURL=users.d.ts.map