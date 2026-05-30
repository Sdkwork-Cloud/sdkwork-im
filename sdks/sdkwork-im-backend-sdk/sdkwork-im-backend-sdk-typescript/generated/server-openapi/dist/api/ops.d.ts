import type { HttpClient } from '../http/client';
import type { ClusterRetrieveResponse, CommercialReadinessRetrieveResponse, DiagnosticsRetrieveResponse, HealthRetrieveResponse, LagRetrieveResponse, OpsProviderBindingsDriftRetrieveResponse, OpsProviderBindingsListResponse, ReplayStatusRetrieveResponse, RuntimeDirRetrieveResponse } from '../types';
export declare class OpsDiagnosticsApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve diagnostics */
    retrieve(): Promise<DiagnosticsRetrieveResponse>;
}
export declare class OpsProviderBindingsDriftApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve provider binding drift */
    list(): Promise<OpsProviderBindingsDriftRetrieveResponse>;
}
export declare class OpsProviderBindingsApi {
    private client;
    readonly drift: OpsProviderBindingsDriftApi;
    constructor(client: HttpClient);
    /** List provider bindings */
    list(): Promise<OpsProviderBindingsListResponse>;
}
export declare class OpsRuntimeDirApi {
    private client;
    constructor(client: HttpClient);
    /** Inspect runtime directory */
    retrieve(): Promise<RuntimeDirRetrieveResponse>;
}
export declare class OpsCommercialReadinessApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve commercial readiness */
    retrieve(): Promise<CommercialReadinessRetrieveResponse>;
}
export declare class OpsReplayStatusApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve replay status */
    retrieve(): Promise<ReplayStatusRetrieveResponse>;
}
export declare class OpsLagApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve projection lag */
    retrieve(): Promise<LagRetrieveResponse>;
}
export declare class OpsClusterApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve cluster state */
    retrieve(): Promise<ClusterRetrieveResponse>;
}
export declare class OpsHealthApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve ops health */
    retrieve(): Promise<HealthRetrieveResponse>;
}
export declare class OpsApi {
    private client;
    readonly health: OpsHealthApi;
    readonly cluster: OpsClusterApi;
    readonly lag: OpsLagApi;
    readonly replayStatus: OpsReplayStatusApi;
    readonly commercialReadiness: OpsCommercialReadinessApi;
    readonly runtimeDir: OpsRuntimeDirApi;
    readonly providerBindings: OpsProviderBindingsApi;
    readonly diagnostics: OpsDiagnosticsApi;
    constructor(client: HttpClient);
}
export declare function createOpsApi(client: HttpClient): OpsApi;
//# sourceMappingURL=ops.d.ts.map