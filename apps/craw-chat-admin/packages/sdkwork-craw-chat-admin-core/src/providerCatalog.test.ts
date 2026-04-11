// @ts-nocheck
import test from 'node:test';
import assert from 'node:assert/strict';

import {
  buildProviderSaveInput,
  emptyProviderDraft,
  providerDraftFromRecord,
} from './providerCatalog.ts';
import type { ProviderCatalogRecord, ProviderRecordWithIntegration } from 'sdkwork-craw-chat-admin-types';

function createProviderRecord(
  overrides: Partial<ProviderCatalogRecord> = {},
): ProviderCatalogRecord {
  return {
    id: 'provider-openrouter-main',
    channel_id: 'openrouter',
    extension_id: 'sdkwork.provider.openrouter',
    adapter_kind: 'openrouter',
    protocol_kind: 'openai',
    base_url: 'https://openrouter.ai/api/v1',
    display_name: 'OpenRouter Main',
    channel_bindings: [{ channel_id: 'openrouter', is_primary: true }],
    integration: {
      mode: 'default_plugin',
      default_plugin_family: 'openrouter',
    },
    execution: {
      binding_kind: 'builtin',
      runtime: 'builtin',
      runtime_key: 'openrouter',
      passthrough_protocol: 'openai',
      supports_provider_adapter: true,
      supports_raw_plugin: false,
      fail_closed: true,
      route_readiness: {
        openai: { executable: true, supported: true },
        anthropic: { executable: false, supported: false },
        gemini: { executable: false, supported: false },
      },
      reason: null,
    },
    credential_readiness: {
      ready: true,
      state: 'ready',
    },
    ...overrides,
  };
}

test('emptyProviderDraft defaults to standard passthrough openai mode', () => {
  const draft = emptyProviderDraft('openai');

  assert.equal(draft.integration_mode, 'standard_passthrough');
  assert.equal(draft.standard_protocol, 'openai');
  assert.equal(draft.default_plugin_family, '');
  assert.equal(draft.adapter_kind, 'openai');
  assert.equal(draft.protocol_kind, 'openai');
  assert.deepEqual(draft.bound_channel_ids, ['openai']);
});

test('providerDraftFromRecord promotes default plugin family to first-class draft mode', () => {
  const draft = providerDraftFromRecord(createProviderRecord());

  assert.equal(draft.integration_mode, 'default_plugin');
  assert.equal(draft.default_plugin_family, 'openrouter');
  assert.equal(draft.standard_protocol, 'openai');
  assert.equal(draft.adapter_kind, 'openrouter');
  assert.equal(draft.protocol_kind, 'openai');
});

test('buildProviderSaveInput serializes standard passthrough providers without plugin fields', () => {
  const payload = buildProviderSaveInput({
    ...emptyProviderDraft('openai'),
    id: 'provider-gemini-main',
    display_name: 'Gemini Main',
    integration_mode: 'standard_passthrough',
    standard_protocol: 'gemini',
    base_url: 'https://generativelanguage.googleapis.com/v1beta/openai',
  });

  assert.deepEqual(payload, {
    id: 'provider-gemini-main',
    channel_id: 'openai',
    adapter_kind: 'gemini',
    base_url: 'https://generativelanguage.googleapis.com/v1beta/openai',
    display_name: 'Gemini Main',
    channel_bindings: [{ channel_id: 'openai', is_primary: true }],
  });
});

test('buildProviderSaveInput serializes default plugin providers with default_plugin_family only', () => {
  const payload = buildProviderSaveInput({
    ...emptyProviderDraft('openrouter'),
    id: 'provider-openrouter-main',
    display_name: 'OpenRouter Main',
    integration_mode: 'default_plugin',
    default_plugin_family: 'openrouter',
    adapter_kind: 'openrouter',
    protocol_kind: 'openai',
    extension_id: 'sdkwork.provider.openrouter',
    base_url: 'https://openrouter.ai/api/v1',
  });

  assert.deepEqual(payload, {
    id: 'provider-openrouter-main',
    channel_id: 'openrouter',
    default_plugin_family: 'openrouter',
    base_url: 'https://openrouter.ai/api/v1',
    display_name: 'OpenRouter Main',
    channel_bindings: [{ channel_id: 'openrouter', is_primary: true }],
  });
});

test('buildProviderSaveInput preserves advanced custom plugin fields', () => {
  const payload = buildProviderSaveInput({
    ...emptyProviderDraft('anthropic'),
    id: 'provider-claude-relay',
    display_name: 'Claude Relay',
    integration_mode: 'custom_plugin',
    adapter_kind: 'native-dynamic',
    protocol_kind: 'anthropic',
    extension_id: 'sdkwork.provider.claude.relay',
    base_url: 'https://relay.example.com',
  });

  assert.deepEqual(payload, {
    id: 'provider-claude-relay',
    channel_id: 'anthropic',
    adapter_kind: 'native-dynamic',
    protocol_kind: 'anthropic',
    extension_id: 'sdkwork.provider.claude.relay',
    base_url: 'https://relay.example.com',
    display_name: 'Claude Relay',
    channel_bindings: [{ channel_id: 'anthropic', is_primary: true }],
  });
});

test('providerDraftFromRecord keeps custom plugin mode for non-default integrations', () => {
  const record: ProviderRecordWithIntegration = {
    id: 'provider-claude-relay',
    channel_id: 'anthropic',
    extension_id: 'sdkwork.provider.claude.relay',
    adapter_kind: 'native-dynamic',
    protocol_kind: 'anthropic',
    base_url: 'https://relay.example.com',
    display_name: 'Claude Relay',
    channel_bindings: [{ channel_id: 'anthropic', is_primary: true }],
    integration: {
      mode: 'custom_plugin',
      default_plugin_family: null,
    },
  };

  const draft = providerDraftFromRecord(record);

  assert.equal(draft.integration_mode, 'custom_plugin');
  assert.equal(draft.adapter_kind, 'native-dynamic');
  assert.equal(draft.protocol_kind, 'anthropic');
  assert.equal(draft.extension_id, 'sdkwork.provider.claude.relay');
});
