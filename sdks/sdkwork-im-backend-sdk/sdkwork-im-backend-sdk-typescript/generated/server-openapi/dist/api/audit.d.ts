import type { HttpClient } from '../http/client';
import type { ExportRetrieveResponse, RecordsCreateResponse, RecordsListResponse } from '../types';
export declare class AuditExportApi {
    private client;
    constructor(client: HttpClient);
    /** Export audit bundle */
    retrieve(): Promise<ExportRetrieveResponse>;
}
export declare class AuditRecordsApi {
    private client;
    constructor(client: HttpClient);
    /** List audit records */
    list(): Promise<RecordsListResponse>;
    /** Record audit anchor */
    create(): Promise<RecordsCreateResponse>;
}
export declare class AuditApi {
    private client;
    readonly records: AuditRecordsApi;
    readonly export: AuditExportApi;
    constructor(client: HttpClient);
}
export declare function createAuditApi(client: HttpClient): AuditApi;
//# sourceMappingURL=audit.d.ts.map