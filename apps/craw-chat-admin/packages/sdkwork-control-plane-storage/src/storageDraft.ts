import type {
  StorageConfigSnapshotRecord,
  StorageConfigUpsertInput,
  StorageCredentialMode,
  StorageFieldInputKind,
  StorageProviderSchemaRecord,
  StorageSchemaFieldRecord,
} from 'sdkwork-control-plane-types';

export type ScopeMode = 'global' | 'tenant';
export type StorageFieldDraftValue = string | boolean;

export interface StorageDraft {
  providerPluginId: string;
  enabled: boolean;
  commonFieldValues: Record<string, StorageFieldDraftValue>;
  credentialMode: StorageCredentialMode | '';
  credentialFieldValues: Record<string, StorageFieldDraftValue>;
  providerConfigText: string;
}

const TOP_LEVEL_CONFIG_FIELD_NAMES = new Set([
  'bucketOrContainer',
  'region',
  'endpoint',
  'publicBaseUrl',
  'uploadPrefix',
  'downloadPrefix',
]);

export function emptyStorageDraft(): StorageDraft {
  return {
    providerPluginId: '',
    enabled: true,
    commonFieldValues: {},
    credentialMode: '',
    credentialFieldValues: {},
    providerConfigText: '',
  };
}

export function fieldValueAsString(value: StorageFieldDraftValue | undefined): string {
  if (typeof value === 'boolean') {
    return value ? 'true' : '';
  }

  return value ?? '';
}

export function fieldValueAsBoolean(value: StorageFieldDraftValue | undefined): boolean {
  return value === true;
}

export function findStorageProvider(
  providers: readonly StorageProviderSchemaRecord[],
  providerPluginId: string | null | undefined,
): StorageProviderSchemaRecord | null {
  if (!providerPluginId) {
    return null;
  }

  return providers.find((provider) => provider.providerPluginId === providerPluginId) ?? null;
}

export function credentialFieldsForMode(
  providerSchema: StorageProviderSchemaRecord | null,
  credentialMode: StorageCredentialMode | '',
): StorageSchemaFieldRecord[] {
  if (!providerSchema) {
    return [];
  }

  return providerSchema.credentialFields.filter((field) => fieldAppliesToCredentialMode(field, credentialMode));
}

export function createStorageDraft(
  snapshot: StorageConfigSnapshotRecord | null,
  providerSchema: StorageProviderSchemaRecord | null,
): StorageDraft {
  const config = snapshot?.config;
  const providerConfig = asPlainRecord(config?.providerConfig);
  const schemaBoundProviderConfigKeys = new Set(
    (providerSchema?.commonFields ?? [])
      .map((field) => field.name)
      .filter((fieldName) => !TOP_LEVEL_CONFIG_FIELD_NAMES.has(fieldName)),
  );
  const providerConfigExtras = Object.fromEntries(
    Object.entries(providerConfig).filter(([fieldName]) => !schemaBoundProviderConfigKeys.has(fieldName)),
  );

  return {
    providerPluginId: snapshot?.binding?.providerPluginId ?? providerSchema?.providerPluginId ?? '',
    enabled: snapshot?.binding?.enabled ?? true,
    commonFieldValues: Object.fromEntries(
      (providerSchema?.commonFields ?? []).map((field) => [
        field.name,
        resolveConfiguredFieldValue(field, config, providerConfig),
      ]),
    ),
    credentialMode: snapshot?.secret?.credentialMode ?? providerSchema?.supportedCredentialModes[0] ?? '',
    credentialFieldValues: Object.fromEntries(
      (providerSchema?.credentialFields ?? []).map((field) => [
        field.name,
        defaultFieldValue(field.inputKind),
      ]),
    ),
    providerConfigText:
      Object.keys(providerConfigExtras).length > 0
        ? JSON.stringify(providerConfigExtras, null, 2)
        : '',
  };
}

export function applyProviderSchema(
  draft: StorageDraft,
  providerSchema: StorageProviderSchemaRecord,
): StorageDraft {
  const providerChanged = draft.providerPluginId !== providerSchema.providerPluginId;
  const nextCredentialMode = providerSchema.supportedCredentialModes.includes(
    draft.credentialMode as StorageCredentialMode,
  )
    ? draft.credentialMode
    : (providerSchema.supportedCredentialModes[0] ?? '');

  return {
    ...draft,
    providerPluginId: providerSchema.providerPluginId,
    commonFieldValues: Object.fromEntries(
      providerSchema.commonFields.map((field) => [
        field.name,
        draft.commonFieldValues[field.name] ?? defaultFieldValue(field.inputKind),
      ]),
    ),
    credentialMode: nextCredentialMode,
    credentialFieldValues: Object.fromEntries(
      providerSchema.credentialFields.map((field) => [
        field.name,
        providerChanged
          ? defaultFieldValue(field.inputKind)
          : (draft.credentialFieldValues[field.name] ?? defaultFieldValue(field.inputKind)),
      ]),
    ),
  };
}

export function resolveCredentialModeLabel(mode: StorageCredentialMode | ''): string {
  switch (mode) {
    case 'access-key-pair':
      return 'Access Key Pair';
    case 'session-access-key-pair':
      return 'Session Access Key Pair';
    case 'role-assumption':
      return 'Role Assumption';
    case 'interoperability-key':
      return 'Interoperability Key';
    case 'service-account-json':
      return 'Service Account JSON';
    case 'account-key':
      return 'Account Key';
    case 'sas-token':
      return 'SAS Token';
    case 'service-principal':
      return 'Service Principal';
    default:
      return 'Credential mode';
  }
}

export function buildStorageUpsertInput(input: {
  draft: StorageDraft;
  providerSchema: StorageProviderSchemaRecord;
  currentSnapshot: StorageConfigSnapshotRecord | null;
  nowMs: number;
}): StorageConfigUpsertInput {
  const { currentSnapshot, draft, nowMs, providerSchema } = input;
  if (!draft.providerPluginId) {
    throw new Error('Choose a storage provider before saving.');
  }

  const providerConfigText = draft.providerConfigText.trim();
  const providerConfig = providerConfigText
    ? parseJsonRecord(providerConfigText, 'Provider configuration JSON must be a valid object.')
    : {};

  for (const field of providerSchema.commonFields) {
    const parsedFieldValue = parseFieldPayloadValue(field, draft.commonFieldValues[field.name]);
    if (field.required && !parsedFieldValue.include) {
      throw new Error(`${field.label} is required.`);
    }

    if (TOP_LEVEL_CONFIG_FIELD_NAMES.has(field.name)) {
      continue;
    }

    if (parsedFieldValue.include) {
      providerConfig[field.name] = parsedFieldValue.value;
    }
  }

  const activeCredentialFields = credentialFieldsForMode(providerSchema, draft.credentialMode);
  const credentialPayload = Object.fromEntries(
    activeCredentialFields.flatMap((field) => {
      const parsedFieldValue = parseFieldPayloadValue(field, draft.credentialFieldValues[field.name]);
      return parsedFieldValue.include ? [[field.name, parsedFieldValue.value] as const] : [];
    }),
  );

  const hasReplacementSecret = Object.keys(credentialPayload).length > 0;
  const existingSecret = currentSnapshot?.secret ?? null;
  const existingProviderPluginId = currentSnapshot?.binding?.providerPluginId ?? null;
  const preservesExistingSecret =
    Boolean(existingSecret)
    && existingProviderPluginId === draft.providerPluginId
    && existingSecret?.credentialMode === draft.credentialMode
    && !hasReplacementSecret;

  if (hasReplacementSecret) {
    for (const field of activeCredentialFields) {
      const parsedFieldValue = parseFieldPayloadValue(field, draft.credentialFieldValues[field.name]);
      if (field.required && !parsedFieldValue.include) {
        throw new Error(`${field.label} is required.`);
      }
    }
  }

  if (!hasReplacementSecret && !preservesExistingSecret) {
    if (!existingSecret) {
      throw new Error('Enter provider credentials before saving storage configuration.');
    }

    if (existingProviderPluginId !== draft.providerPluginId) {
      throw new Error('Switching storage provider requires a fresh credential submission.');
    }

    if (existingSecret.credentialMode !== draft.credentialMode) {
      throw new Error('Changing credential mode requires a fresh credential submission.');
    }
  }

  return {
    binding: {
      providerPluginId: draft.providerPluginId,
      enabled: draft.enabled,
    },
    config: {
      bucketOrContainer: topLevelConfigValue(draft.commonFieldValues.bucketOrContainer, 'Bucket or container'),
      region: topLevelConfigValue(draft.commonFieldValues.region, 'Region'),
      endpoint: topLevelConfigValue(draft.commonFieldValues.endpoint, 'Endpoint'),
      publicBaseUrl: topLevelConfigValue(draft.commonFieldValues.publicBaseUrl, 'Public Base URL'),
      uploadPrefix: topLevelConfigValue(draft.commonFieldValues.uploadPrefix, 'Upload Prefix'),
      downloadPrefix: topLevelConfigValue(draft.commonFieldValues.downloadPrefix, 'Download Prefix'),
      providerConfig,
    },
    ...(hasReplacementSecret
      ? {
          secret: {
            credentialMode: ensureCredentialMode(draft.credentialMode),
            encryptedSecretPayload: JSON.stringify(credentialPayload),
            secretFingerprint: `${draft.providerPluginId}-${nowMs}`,
          },
        }
      : {}),
  };
}

function asPlainRecord(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return {};
  }

  return value as Record<string, unknown>;
}

function defaultFieldValue(inputKind: StorageFieldInputKind): StorageFieldDraftValue {
  return inputKind === 'boolean' ? false : '';
}

function resolveConfiguredFieldValue(
  field: StorageSchemaFieldRecord,
  config: StorageConfigSnapshotRecord['config'] | null | undefined,
  providerConfig: Record<string, unknown>,
): StorageFieldDraftValue {
  let rawValue: unknown;
  switch (field.name) {
    case 'bucketOrContainer':
      rawValue = config?.bucketOrContainer;
      break;
    case 'region':
      rawValue = config?.region;
      break;
    case 'endpoint':
      rawValue = config?.endpoint;
      break;
    case 'publicBaseUrl':
      rawValue = config?.publicBaseUrl;
      break;
    case 'uploadPrefix':
      rawValue = config?.uploadPrefix;
      break;
    case 'downloadPrefix':
      rawValue = config?.downloadPrefix;
      break;
    default:
      rawValue = providerConfig[field.name];
      break;
  }

  if (field.inputKind === 'boolean') {
    return Boolean(rawValue);
  }

  if (rawValue == null) {
    return '';
  }

  if (field.inputKind === 'json') {
    return typeof rawValue === 'string' ? rawValue : JSON.stringify(rawValue, null, 2);
  }

  return String(rawValue);
}

function parseFieldPayloadValue(
  field: StorageSchemaFieldRecord,
  value: StorageFieldDraftValue | undefined,
): { include: boolean; value: unknown } {
  if (field.inputKind === 'boolean') {
    return {
      include: true,
      value: fieldValueAsBoolean(value),
    };
  }

  const normalized = fieldValueAsString(value).trim();
  if (!normalized) {
    return {
      include: false,
      value: null,
    };
  }

  if (field.inputKind === 'number') {
    const parsedNumber = Number(normalized);
    if (!Number.isFinite(parsedNumber)) {
      throw new Error(`${field.label} must be a valid number.`);
    }

    return {
      include: true,
      value: parsedNumber,
    };
  }

  if (field.inputKind === 'json') {
    return {
      include: true,
      value: JSON.parse(normalized),
    };
  }

  return {
    include: true,
    value: normalized,
  };
}

function parseJsonRecord(value: string, errorMessage: string): Record<string, unknown> {
  const parsedValue = JSON.parse(value);
  if (!parsedValue || typeof parsedValue !== 'object' || Array.isArray(parsedValue)) {
    throw new Error(errorMessage);
  }

  return parsedValue as Record<string, unknown>;
}

function topLevelConfigValue(value: StorageFieldDraftValue | undefined, label: string): string | null {
  const normalized = fieldValueAsString(value).trim();
  if (!normalized) {
    return null;
  }

  if (normalized.length > 4096) {
    throw new Error(`${label} is too long.`);
  }

  return normalized;
}

function ensureCredentialMode(mode: StorageCredentialMode | ''): StorageCredentialMode {
  if (!mode) {
    throw new Error('Choose a credential mode before saving credentials.');
  }

  return mode;
}

function fieldAppliesToCredentialMode(
  field: StorageSchemaFieldRecord,
  credentialMode: StorageCredentialMode | '',
): boolean {
  if (!field.credentialModes?.length) {
    return true;
  }

  if (!credentialMode) {
    return false;
  }

  return field.credentialModes.includes(credentialMode);
}
