import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/GroupService';

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
          principalId: 'u_owner',
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
              actorId: 'u_owner',
              actorKind: 'user',
              conversationId: 'group-1',
              deviceId,
              memberId: 'member-new',
              occurredAt: '2026-06-04T10:00:00.000Z',
              originEventId: 'evt-member-join',
              originEventType: 'conversation.member_joined',
              payload: JSON.stringify({
                member: {
                  memberId: 'member-new',
                  principalId: 'u_new',
                  principalKind: 'user',
                  role: 'member',
                  state: 'joined',
                },
              }),
              payloadSchema: 'conversation.member.v1',
              principalId: 'u_owner',
              syncSeq: 1,
              tenantId: 'tenant-1',
            },
            {
              actorId: 'u_owner',
              actorKind: 'user',
              conversationId: 'group-1',
              deviceId,
              memberId: 'member-old',
              occurredAt: '2026-06-04T10:00:05.000Z',
              originEventId: 'evt-member-remove',
              originEventType: 'conversation.member_removed',
              payload: JSON.stringify({
                member: {
                  memberId: 'member-old',
                  principalId: 'u_old',
                  principalKind: 'user',
                  role: 'member',
                  state: 'removed',
                },
              }),
              payloadSchema: 'conversation.member.v1',
              principalId: 'u_owner',
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
  const service = createSdkworkGroupService(() => fakeClient);

  const changes = await service.syncGroupMembersFromDeviceFeed('device-pc-1');

  assert.deepEqual(
    syncFeedCalls,
    [
      { method: 'device.registrations.create', deviceId: 'device-pc-1', params: undefined },
      { method: 'device.syncFeed.retrieve', deviceId: 'device-pc-1', params: { afterSeq: 0, limit: 100 } },
    ],
    'group service must register the current IM device before consuming group member changes from the standard IM device sync feed',
  );
  assert.deepEqual(
    changes,
    [
      {
        activeCount: 1,
        groupId: 'group-1',
        memberCount: 1,
        members: ['u_new'],
      },
    ],
    'group member feed sync must incrementally apply joined and removed membership events',
  );

  console.log('sdkwork-chat-pc group device sync feed contract passed');
}

void main();
