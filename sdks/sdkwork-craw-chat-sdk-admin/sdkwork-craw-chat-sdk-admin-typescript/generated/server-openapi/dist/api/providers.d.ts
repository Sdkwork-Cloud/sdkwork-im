import type { HttpClient } from '../http/client';
export declare class ProvidersApi {
    private client;
    constructor(client: HttpClient);
    /** Get provider-bindings */
    getApiV1ControlProviderBindings(): Promise<void>;
    /** Post provider-bindings */
    postApiV1ControlProviderBindings(): Promise<void>;
    /** Get provider-policies */
    getApiV1ControlProviderPolicies(): Promise<void>;
    /** Get provider-policies diff */
    getApiV1ControlProviderPoliciesDiff(): Promise<void>;
    /** Post provider-policies preview */
    postApiV1ControlProviderPoliciesPreview(): Promise<void>;
    /** Post provider-policies rollback */
    postApiV1ControlProviderPoliciesRollback(): Promise<void>;
    /** Get provider registry snapshot */
    getApiV1ControlProviderRegistry(): Promise<void>;
}
export declare function createProvidersApi(client: HttpClient): ProvidersApi;
//# sourceMappingURL=providers.d.ts.map