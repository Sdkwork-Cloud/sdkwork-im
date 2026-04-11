import type {
  ProviderCatalogRecord,
  ProviderIntegrationMode,
  ProviderRecordWithIntegration,
  SaveProviderInput,
} from 'sdkwork-craw-chat-admin-types';

export type StandardProviderProtocol = 'openai' | 'anthropic' | 'gemini';
export type DefaultPluginFamily = 'openrouter' | 'ollama';

export type ProviderDraft = {
  id: string;
  display_name: string;
  integration_mode: ProviderIntegrationMode;
  standard_protocol: StandardProviderProtocol;
  default_plugin_family: string;
  adapter_kind: string;
  protocol_kind: string;
  extension_id: string;
  base_url: string;
  primary_channel_id: string;
  bound_channel_ids: string[];
};

export const STANDARD_PROVIDER_PROTOCOL_OPTIONS: Array<{
  label: string;
  value: StandardProviderProtocol;
}> = [
  { label: 'OpenAI', value: 'openai' },
  { label: 'Anthropic', value: 'anthropic' },
  { label: 'Gemini', value: 'gemini' },
];

export const DEFAULT_PLUGIN_FAMILY_OPTIONS: Array<{
  label: string;
  value: DefaultPluginFamily;
}> = [
  { label: 'OpenRouter', value: 'openrouter' },
  { label: 'Ollama', value: 'ollama' },
];

export const CUSTOM_PLUGIN_PROTOCOL_OPTIONS: Array<{
  label: string;
  value: string;
}> = [
  { label: 'OpenAI', value: 'openai' },
  { label: 'Anthropic', value: 'anthropic' },
  { label: 'Gemini', value: 'gemini' },
  { label: 'Custom', value: 'custom' },
];

function normalizeText(value: string | null | undefined): string {
  return value?.trim() ?? '';
}

function normalizeProviderIntegrationMode(
  value: string | null | undefined,
): ProviderIntegrationMode {
  if (
    value === 'standard_passthrough'
    || value === 'default_plugin'
    || value === 'custom_plugin'
  ) {
    return value;
  }
  return 'custom_plugin';
}

function normalizeStandardProtocol(
  value: string | null | undefined,
): StandardProviderProtocol {
  const normalized = normalizeText(value).toLowerCase();
  if (
    normalized === 'openai'
    || normalized === 'anthropic'
    || normalized === 'gemini'
  ) {
    return normalized;
  }
  return 'openai';
}

function deriveDefaultPluginProtocol(
  defaultPluginFamily: string,
): StandardProviderProtocol | 'custom' {
  switch (normalizeText(defaultPluginFamily).toLowerCase()) {
    case 'openrouter':
      return 'openai';
    case 'ollama':
      return 'custom';
    default:
      return 'custom';
  }
}

function deriveDefaultPluginExtensionId(defaultPluginFamily: string): string {
  switch (normalizeText(defaultPluginFamily).toLowerCase()) {
    case 'openrouter':
      return 'sdkwork.provider.openrouter';
    case 'ollama':
      return 'sdkwork.provider.ollama';
    default:
      return '';
  }
}

function collectProviderChannelIds(
  provider: ProviderRecordWithIntegration | ProviderCatalogRecord,
): string[] {
  const ids = new Set<string>([provider.channel_id]);
  for (const binding of provider.channel_bindings) {
    ids.add(binding.channel_id);
  }
  return Array.from(ids);
}

function baseProviderSaveInput(draft: ProviderDraft): SaveProviderInput {
  const primaryChannelId = normalizeText(draft.primary_channel_id);
  const bindingIds = Array.from(
    new Set(
      [primaryChannelId, ...draft.bound_channel_ids]
        .map((value) => value.trim())
        .filter(Boolean),
    ),
  );

  return {
    id: normalizeText(draft.id),
    channel_id: primaryChannelId,
    base_url: normalizeText(draft.base_url),
    display_name: normalizeText(draft.display_name),
    channel_bindings: bindingIds.map((channelId) => ({
      channel_id: channelId,
      is_primary: channelId === primaryChannelId,
    })),
  };
}

export function emptyProviderDraft(defaultChannelId: string): ProviderDraft {
  return {
    id: '',
    display_name: '',
    integration_mode: 'standard_passthrough',
    standard_protocol: 'openai',
    default_plugin_family: '',
    adapter_kind: 'openai',
    protocol_kind: 'openai',
    extension_id: '',
    base_url: '',
    primary_channel_id: defaultChannelId,
    bound_channel_ids: defaultChannelId ? [defaultChannelId] : [],
  };
}

export function providerDraftFromRecord(
  provider: ProviderRecordWithIntegration | ProviderCatalogRecord,
): ProviderDraft {
  const integrationMode = normalizeProviderIntegrationMode(provider.integration.mode);
  const standardProtocol = normalizeStandardProtocol(
    integrationMode === 'standard_passthrough'
      ? provider.protocol_kind || provider.adapter_kind
      : provider.protocol_kind,
  );

  return {
    id: provider.id,
    display_name: provider.display_name,
    integration_mode: integrationMode,
    standard_protocol: standardProtocol,
    default_plugin_family: normalizeText(provider.integration.default_plugin_family),
    adapter_kind: provider.adapter_kind,
    protocol_kind: normalizeText(provider.protocol_kind),
    extension_id: normalizeText(provider.extension_id),
    base_url: provider.base_url,
    primary_channel_id: provider.channel_id,
    bound_channel_ids: collectProviderChannelIds(provider),
  };
}

export function buildProviderSaveInput(draft: ProviderDraft): SaveProviderInput {
  const base = baseProviderSaveInput(draft);

  if (draft.integration_mode === 'default_plugin') {
    return {
      ...base,
      default_plugin_family: normalizeText(draft.default_plugin_family),
    };
  }

  if (draft.integration_mode === 'custom_plugin') {
    const payload: SaveProviderInput = {
      ...base,
      adapter_kind: normalizeText(draft.adapter_kind),
    };
    const protocolKind = normalizeText(draft.protocol_kind);
    const extensionId = normalizeText(draft.extension_id);

    if (protocolKind) {
      payload.protocol_kind = protocolKind;
    }
    if (extensionId) {
      payload.extension_id = extensionId;
    }

    return payload;
  }

  return {
    ...base,
    adapter_kind: draft.standard_protocol,
  };
}

export function applyProviderIntegrationMode(
  draft: ProviderDraft,
  integrationMode: ProviderIntegrationMode,
): ProviderDraft {
  if (integrationMode === 'default_plugin') {
    const defaultPluginFamily =
      normalizeText(draft.default_plugin_family) || 'openrouter';
    const protocolKind = deriveDefaultPluginProtocol(defaultPluginFamily);

    return {
      ...draft,
      integration_mode: integrationMode,
      default_plugin_family: defaultPluginFamily,
      adapter_kind: defaultPluginFamily,
      protocol_kind: protocolKind === 'custom' ? 'custom' : protocolKind,
      extension_id: deriveDefaultPluginExtensionId(defaultPluginFamily),
    };
  }

  if (integrationMode === 'custom_plugin') {
    return {
      ...draft,
      integration_mode: integrationMode,
      adapter_kind: normalizeText(draft.adapter_kind) || 'native-dynamic',
      protocol_kind: normalizeText(draft.protocol_kind) || 'custom',
    };
  }

  return {
    ...draft,
    integration_mode: integrationMode,
    default_plugin_family: '',
    adapter_kind: draft.standard_protocol,
    protocol_kind: draft.standard_protocol,
    extension_id: '',
  };
}

export function applyProviderStandardProtocol(
  draft: ProviderDraft,
  protocol: StandardProviderProtocol,
): ProviderDraft {
  return {
    ...draft,
    standard_protocol: protocol,
    adapter_kind: protocol,
    protocol_kind: protocol,
    extension_id: '',
  };
}

export function applyProviderDefaultPluginFamily(
  draft: ProviderDraft,
  defaultPluginFamily: DefaultPluginFamily,
): ProviderDraft {
  const protocolKind = deriveDefaultPluginProtocol(defaultPluginFamily);

  return {
    ...draft,
    default_plugin_family: defaultPluginFamily,
    adapter_kind: defaultPluginFamily,
    protocol_kind: protocolKind === 'custom' ? 'custom' : protocolKind,
    extension_id: deriveDefaultPluginExtensionId(defaultPluginFamily),
  };
}

export function describeProviderIntegration(
  provider: ProviderRecordWithIntegration | ProviderCatalogRecord,
): string {
  if (provider.integration.mode === 'default_plugin') {
    return `default-plugin/${provider.integration.default_plugin_family ?? provider.adapter_kind}`;
  }
  if (provider.integration.mode === 'standard_passthrough') {
    return `standard/${provider.protocol_kind}`;
  }
  return `custom/${provider.adapter_kind}`;
}
