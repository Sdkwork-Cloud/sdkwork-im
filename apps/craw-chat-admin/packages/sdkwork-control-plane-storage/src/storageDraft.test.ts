// @ts-nocheck
import assert from 'node:assert/strict';
import test from 'node:test';

import { buildStorageUpsertInput } from './storageDraft.ts';

function createGoogleProviderSchema() {
  return {
    providerPluginId: 'object-storage-google',
    displayName: 'Google Cloud Storage',
    providerFamily: 'google-cloud-storage',
    commonFields: [
      {
        name: 'bucketOrContainer',
        label: 'Bucket',
        inputKind: 'text',
        required: true,
      },
    ],
    credentialFields: [
      {
        name: 'serviceAccountJson',
        label: 'Service Account JSON',
        inputKind: 'json',
        required: true,
        credentialModes: ['service-account-json'],
      },
      {
        name: 'interoperabilityAccessKey',
        label: 'Interoperability Access Key',
        inputKind: 'text',
        required: true,
        credentialModes: ['interoperability-key'],
      },
      {
        name: 'interoperabilitySecretKey',
        label: 'Interoperability Secret Key',
        inputKind: 'secret',
        required: true,
        credentialModes: ['interoperability-key'],
      },
    ],
    supportedCredentialModes: ['service-account-json', 'interoperability-key'],
    capabilities: ['presign'],
  };
}

test('buildStorageUpsertInput only serializes credential fields active for the selected mode', () => {
  const payload = buildStorageUpsertInput({
    draft: {
      providerPluginId: 'object-storage-google',
      enabled: true,
      commonFieldValues: {
        bucketOrContainer: 'tenant-assets',
      },
      credentialMode: 'service-account-json',
      credentialFieldValues: {
        serviceAccountJson: '{"projectId":"tenant-assets"}',
        interoperabilityAccessKey: 'stale-access-key',
        interoperabilitySecretKey: 'stale-secret-key',
      },
      providerConfigText: '',
    },
    providerSchema: createGoogleProviderSchema(),
    currentSnapshot: null,
    nowMs: 42,
  });

  assert.deepEqual(JSON.parse(payload.secret.encryptedSecretPayload), {
    serviceAccountJson: {
      projectId: 'tenant-assets',
    },
  });
});

test('buildStorageUpsertInput requires all credential fields marked required for the selected mode', () => {
  assert.throws(
    () =>
      buildStorageUpsertInput({
        draft: {
          providerPluginId: 'object-storage-google',
          enabled: true,
          commonFieldValues: {
            bucketOrContainer: 'tenant-assets',
          },
          credentialMode: 'interoperability-key',
          credentialFieldValues: {
            interoperabilityAccessKey: 'interop-access-key',
            interoperabilitySecretKey: '',
          },
          providerConfigText: '',
        },
        providerSchema: createGoogleProviderSchema(),
        currentSnapshot: null,
        nowMs: 43,
      }),
    /Interoperability Secret Key is required/,
  );
});
