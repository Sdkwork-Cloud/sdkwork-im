import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { retrieveDeviceSyncFeedWindow } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/DeviceSyncFeedService';

const calls: Array<{
  deviceId: string;
  method: string;
}> = [];
const registeredDeviceIds = new Set<string>();

const fakeClient = {
  device: {
    registrations: {
      async create(body: { deviceId?: string }) {
        const deviceId = body.deviceId ?? '';
        calls.push({ method: 'device.registrations.create', deviceId });
        registeredDeviceIds.add(deviceId);
        return {
          deviceId,
          principalId: 'u_alice',
          registeredAt: '2026-06-04T09:59:00.000Z',
          tenantId: 'tenant-1',
        };
      },
    },
    syncFeed: {
      async retrieve(deviceId: string) {
        calls.push({ method: 'device.syncFeed.retrieve', deviceId });
        if (!registeredDeviceIds.has(deviceId)) {
          throw Object.assign(new Error(`device scope forbidden: ${deviceId}`), {
            code: 'device_scope_forbidden',
            status: 403,
          });
        }
        return {
          items: [],
          nextAfterSeq: 0,
          hasMore: false,
          trimmedThroughSeq: 0,
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const result = await retrieveDeviceSyncFeedWindow(
    fakeClient,
    'chat',
    'd_mpz9hezz_t1rjq2g8',
    new Map(),
  );

  assert.deepEqual(
    calls,
    [
      { method: 'device.registrations.create', deviceId: 'd_mpz9hezz_t1rjq2g8' },
      { method: 'device.syncFeed.retrieve', deviceId: 'd_mpz9hezz_t1rjq2g8' },
    ],
    'device sync feed retrieval must register the path device id before reading the scoped feed',
  );
  assert.deepEqual(result, {
    deviceId: 'd_mpz9hezz_t1rjq2g8',
    entries: [],
    nextAfterSeq: 0,
    trimmedThroughSeq: 0,
  });

  console.log('sdkwork-chat-pc device sync registration guard contract passed');
}

void main();
