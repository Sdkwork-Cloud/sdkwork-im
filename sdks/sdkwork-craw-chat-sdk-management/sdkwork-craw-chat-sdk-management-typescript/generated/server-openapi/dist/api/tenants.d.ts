import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class TenantsApi {
    private client;
    constructor(client: HttpClient);
    listTenants(): Promise<LooseJsonValue>;
    saveTenant(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteTenant(tenantId: string | number): Promise<LooseJsonValue>;
    listProjects(): Promise<LooseJsonValue>;
    saveProject(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteProject(projectId: string | number): Promise<LooseJsonValue>;
}
export declare function createTenantsApi(client: HttpClient): TenantsApi;
//# sourceMappingURL=tenants.d.ts.map