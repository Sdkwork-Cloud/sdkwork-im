import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ContactService';

type DeviceSyncParams = {
  afterSeq?: number;
  limit?: number;
};

const syncFeedCalls: Array<{
  deviceId: string;
  method: string;
  params?: DeviceSyncParams;
}> = [];

const fakeClient = {
  device: {
    registrations: {
      async create(body: Record<string, unknown>) {
        syncFeedCalls.push({ method: 'device.registrations.create', deviceId: String(body.deviceId), params: undefined });
        return {
          deviceId: body.deviceId,
          principalId: 'current-user',
          registeredAt: '2026-06-04T09:59:00.000Z',
          tenantId: 'tenant-1',
        };
      },
    },
    syncFeed: {
      async retrieve(deviceId: string, params?: DeviceSyncParams) {
        syncFeedCalls.push({ method: 'device.syncFeed.retrieve', deviceId, params });
        return {
          items: [
            {
              actorId: 'current-user',
              actorKind: 'user',
              deviceId,
              occurredAt: '2026-06-04T10:00:00.000Z',
              originEventId: 'evt-friendship-activated',
              originEventType: 'friendship.activated',
              payload: JSON.stringify({
                friendshipId: 'friendship-1',
                userLowId: 'current-user',
                userHighId: 'u_alice',
                directChatId: 'direct-chat-1',
                conversationId: 'chat-alice',
                activatedAt: '2026-06-04T10:00:00.000Z',
              }),
              payloadSchema: 'social.friendship.activated.v1',
              principalId: 'current-user',
              syncSeq: 1,
              tenantId: 'tenant-1',
            },
            {
              actorId: 'current-user',
              actorKind: 'user',
              deviceId,
              occurredAt: '2026-06-04T10:00:05.000Z',
              originEventId: 'evt-friendship-removed',
              originEventType: 'friendship.removed',
              payload: JSON.stringify({
                friendshipId: 'friendship-2',
                userLowId: 'current-user',
                userHighId: 'u_removed',
                removedAt: '2026-06-04T10:00:05.000Z',
              }),
              payloadSchema: 'social.friendship.removed.v1',
              principalId: 'current-user',
              syncSeq: 2,
              tenantId: 'tenant-1',
            },
          ],
          nextAfterSeq: 2,
          hasMore: false,
          trimmedThroughSeq: 0,
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkContactService(() => fakeClient);

  const result = await service.syncContactsFromDeviceFeed('device-pc-1');

  assert.deepEqual(
    syncFeedCalls,
    [
      { method: 'device.registrations.create', deviceId: 'device-pc-1', params: undefined },
      { method: 'device.syncFeed.retrieve', deviceId: 'device-pc-1', params: { afterSeq: 0, limit: 100 } },
    ],
    'contact service must register the current IM device before consuming friendship/contact changes from the standard IM device sync feed',
  );
  assert.deepEqual(
    result.added.map((user) => [user.id, user.name, user.departmentId]),
    [['u_alice', 'u_alice', undefined]],
    'friendship activation feed entries must hydrate SDK-backed contact projection state without assigning synthetic departments',
  );
  assert.deepEqual(
    result.removedUserIds,
    ['u_removed'],
    'friendship removal feed entries must evict removed contacts from local contact projection state',
  );
  assert.equal(result.nextAfterSeq, 2);
  assert.equal(result.trimmedThroughSeq, 0);

  console.log('sdkwork-chat-pc contact device sync feed contract passed');
}

void main();
