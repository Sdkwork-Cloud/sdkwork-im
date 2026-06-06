import assert from 'node:assert/strict';
import type { SdkworkImAppClient } from '@sdkwork/clawchat-pc-core';
import {
  createSdkworkSettingsService,
  DEFAULT_SIDEBAR_MODULES,
} from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/SettingsService';

const calls: string[] = [];

const fakeClient = {
  device: {
    twin: {
      async retrieve(deviceId: string) {
        calls.push(`device.twin.retrieve:${deviceId}`);
        return {
          tenantId: 't_test',
          deviceId,
          desiredStateJson: JSON.stringify({
            loginDevices: [
              {
                id: 'd_pad',
                name: 'iPad Pro',
                time: '2026-06-04 09:30 active',
              },
            ],
          }),
          reportedStateJson: JSON.stringify({
            currentDeviceName: 'Windows PC',
            loginDevices: [
              {
                id: 'd_phone',
                name: 'iPhone 15 Pro',
                time: '2026-06-04 08:10 active',
              },
            ],
          }),
          updatedAt: '2026-06-04T00:00:00.000Z',
        };
      },
      desired: {
        async update(deviceId: string, body: { desiredStateJson: string }) {
          calls.push(`device.twin.desired.update:${deviceId}`);
          assert.deepEqual(JSON.parse(body.desiredStateJson), {
            disabledLoginDeviceIds: ['d_phone'],
          });
          return {
            tenantId: 't_test',
            deviceId,
            desiredStateJson: body.desiredStateJson,
            reportedStateJson: '{}',
            updatedAt: '2026-06-04T00:00:00.000Z',
          };
        },
      },
    },
  },
  portal: {
    home: {
      async retrieve() {
        calls.push('portal.home.retrieve');
        return {
          enabledModules: ['chat', 'contacts', 'agent', 'enterprise'],
        };
      },
    },
  },
} as unknown as SdkworkImAppClient;

function installLocalStorage(initial: Record<string, string>): void {
  const store = new Map(Object.entries(initial));
  Object.defineProperty(globalThis, 'localStorage', {
    configurable: true,
    value: {
      getItem(key: string) {
        return store.get(key) ?? null;
      },
      setItem(key: string, value: string) {
        store.set(key, value);
      },
      removeItem(key: string) {
        store.delete(key);
      },
      clear() {
        store.clear();
      },
    },
  });
}

async function main(): Promise<void> {
  installLocalStorage({});
  const defaultService = createSdkworkSettingsService(() => fakeClient, () => 'd_pc');
  assert.deepEqual(
    (await defaultService.getSettings()).sidebarModules,
    DEFAULT_SIDEBAR_MODULES,
    'new installations must only show the product-ready sidebar modules by default',
  );

  installLocalStorage({
    'clawchat-settings': JSON.stringify({
      sidebarModules: [
        'chat',
        'workspace',
        'orders',
        'shop',
        'calendar',
        'notary',
        'knowledge',
        'enterprise',
        'devices',
        'community',
        'voice',
        'agent',
        'course',
        'contacts',
        'favorites',
      ],
    }),
  });
  const migratedService = createSdkworkSettingsService(() => fakeClient, () => 'd_pc');
  assert.deepEqual(
    (await migratedService.getSettings()).sidebarModules,
    DEFAULT_SIDEBAR_MODULES,
    'legacy all-module settings must migrate to the product-ready default sidebar modules',
  );

  installLocalStorage({});
  const service = createSdkworkSettingsService(() => fakeClient, () => 'd_pc');

  const serverModules = await service.getServerModules();
  assert.deepEqual(serverModules, ['chat', 'contacts', 'agent', 'enterprise']);

  const devices = await service.getDevices();
  assert.deepEqual(devices, [
    {
      id: 'd_phone',
      name: 'iPhone 15 Pro',
      time: '2026-06-04 08:10 active',
    },
    {
      id: 'd_pad',
      name: 'iPad Pro',
      time: '2026-06-04 09:30 active',
    },
  ]);

  await service.removeDevice('d_phone');

  assert.deepEqual(calls, [
    'portal.home.retrieve',
    'device.twin.retrieve:d_pc',
    'device.twin.desired.update:d_pc',
  ]);

  console.log('sdkwork-chat-pc settings real-logic contract passed');
}

void main();
