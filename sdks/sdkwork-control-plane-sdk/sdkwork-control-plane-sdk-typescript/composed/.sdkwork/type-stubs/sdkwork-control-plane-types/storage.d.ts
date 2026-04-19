export type StorageScopeKind = 'global' | 'tenant';
export type StorageFieldInputKind = 'text' | 'url' | 'number' | 'boolean' | 'secret' | 'json';
export type StorageCredentialMode = 'access-key-pair' | 'session-access-key-pair' | 'role-assumption' | 'interoperability-key' | 'service-account-json' | 'account-key' | 'sas-token' | 'service-principal';
export interface StorageScopeRef {
    kind: StorageScopeKind;
    scopeId: string | null;
}
export interface StorageSchemaFieldRecord {
    name: string;
    label: string;
    inputKind: StorageFieldInputKind;
    required: boolean;
    helpText?: string | null;
    credentialModes?: StorageCredentialMode[] | null;
}
export interface StorageProviderSchemaRecord {
    providerPluginId: string;
    displayName: string;
    providerFamily: string;
    commonFields: StorageSchemaFieldRecord[];
    credentialFields: StorageSchemaFieldRecord[];
    supportedCredentialModes: StorageCredentialMode[];
    capabilities: string[];
}
export interface StorageBindingRecord {
    scope: StorageScopeRef;
    providerPluginId: string;
    enabled: boolean;
}
export interface StorageConfigRecord {
    scope: StorageScopeRef;
    providerPluginId: string;
    bucketOrContainer: string | null;
    region: string | null;
    endpoint: string | null;
    publicBaseUrl: string | null;
    uploadPrefix: string | null;
    downloadPrefix: string | null;
    providerConfig: Record<string, unknown>;
}
export interface StorageSecretSummaryRecord {
    scope: StorageScopeRef;
    providerPluginId: string;
    credentialMode: StorageCredentialMode;
    configured: boolean;
    secretFingerprint: string;
}
export interface StorageConfigSnapshotRecord {
    scope: StorageScopeRef;
    binding: StorageBindingRecord | null;
    config: StorageConfigRecord | null;
    secret: StorageSecretSummaryRecord | null;
}
export interface StorageEffectiveConfigRecord {
    requestedScope: StorageScopeRef;
    resolvedScope: StorageScopeRef;
    binding: StorageBindingRecord;
    config: StorageConfigRecord;
    secret: StorageSecretSummaryRecord | null;
}
export interface StorageValidationRecord {
    scope: StorageScopeRef;
    status: 'healthy' | 'degraded' | 'invalid' | 'unknown';
    stage: 'schema' | 'credentials' | 'bucket' | 'presign' | 'readback';
    message: string;
    providerPluginId?: string | null;
}
export interface StorageAuditRecord {
    id: string;
    action: string;
    scope: StorageScopeRef;
    providerPluginId: string;
    createdAtMs: number;
}
export interface StorageConfigUpsertInput {
    binding: {
        providerPluginId: string;
        enabled?: boolean;
    };
    config: {
        bucketOrContainer?: string | null;
        region?: string | null;
        endpoint?: string | null;
        publicBaseUrl?: string | null;
        uploadPrefix?: string | null;
        downloadPrefix?: string | null;
        providerConfig?: Record<string, unknown>;
    };
    secret?: {
        credentialMode: StorageCredentialMode;
        encryptedSecretPayload: string;
        secretFingerprint?: string;
    } | null;
}
//# sourceMappingURL=storage.d.ts.map