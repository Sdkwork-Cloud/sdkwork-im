import assert from 'node:assert/strict';
import { createSdkworkImSyncCoordinatorService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ImSyncCoordinatorService';

type DeviceSyncParams = {
  afterSeq?: number;
  limit?: number;
};

const calls: Array<{
  deviceId?: string;
  method: string;
  params?: DeviceSyncParams;
  rtcSessionId?: string;
  type?: string;
}> = [];

const fakeClient = {
  device: {
    registrations: {
      async create(body: Record<string, unknown>) {
        calls.push({ method: 'device.registrations.create', deviceId: String(body.deviceId) });
        return {
          deviceId: body.deviceId,
          principalId: 'u_alice',
          registeredAt: '2026-06-04T09:59:00.000Z',
          tenantId: 'tenant-1',
        };
      },
    },
    syncFeed: {
      async retrieve(deviceId: string, params?: DeviceSyncParams) {
        calls.push({ method: 'device.syncFeed.retrieve', deviceId, params });
        return {
          items: [
            {
              actorId: 'u_bob',
              actorKind: 'user',
              conversationId: 'conversation-rtc-1',
              deviceId,
              messageId: 'message-rtc-signal-1',
              messageSeq: 7,
              occurredAt: '2026-06-04T10:00:00.000Z',
              originEventId: 'evt-message-rtc-signal-1',
              originEventType: 'message.posted',
              payload: JSON.stringify({
                rtcSessionId: 'rtc-startup-1',
                body: {
                  parts: [
                    {
                      kind: 'signal',
                      payload: JSON.stringify({
                        conversationId: 'conversation-rtc-1',
                        rtcMode: 'video',
                        rtcSessionId: 'rtc-startup-1',
                        state: 'accepted',
                      }),
                      signalType: 'rtc.accept',
                    },
                  ],
                },
              }),
              payloadSchema: 'message.posted.v1',
              principalId: 'u_alice',
              summary: 'rtc.accept',
              syncSeq: 1,
              tenantId: 'tenant-1',
            },
          ],
          nextAfterSeq: 1,
          hasMore: false,
          trimmedThroughSeq: 0,
        };
      },
    },
  },
};

async function main(): Promise<void> {
  const service = createSdkworkImSyncCoordinatorService({
    callService: {
      async recoverRtcSession(rtcSessionId, options) {
        calls.push({ method: 'call.recoverRtcSession', rtcSessionId, type: options?.type });
        return {
          state: 'connected',
          conversationId: 'conversation-rtc-1',
          isAudioMuted: false,
          isVideoMuted: false,
          rtcSessionId,
          type: options?.type,
        };
      },
    },
    chatService: {
      async syncOfflineMessages(deviceId) {
        calls.push({ method: 'chat.syncOfflineMessages', deviceId });
        return {
          appliedMessages: 1,
          appliedReadCursors: 0,
          deviceId: deviceId ?? '',
          nextAfterSeq: 1,
          trimmedThroughSeq: 0,
        };
      },
    },
    contactService: {
      async syncContactsFromDeviceFeed(deviceId) {
        calls.push({ method: 'contact.syncContactsFromDeviceFeed', deviceId });
        return {
          added: [],
          deviceId: deviceId ?? '',
          nextAfterSeq: 1,
          removedUserIds: [],
          trimmedThroughSeq: 0,
        };
      },
    },
    getClient: () => fakeClient,
    groupService: {
      async syncGroupMembersFromDeviceFeed(deviceId) {
        calls.push({ method: 'group.syncGroupMembersFromDeviceFeed', deviceId });
        return [];
      },
    },
  });

  const result = await service.syncStartup({
    deviceId: 'device-pc-1',
  });

  assert.deepEqual(
    calls,
    [
      { method: 'chat.syncOfflineMessages', deviceId: 'device-pc-1' },
      { method: 'contact.syncContactsFromDeviceFeed', deviceId: 'device-pc-1' },
      { method: 'group.syncGroupMembersFromDeviceFeed', deviceId: 'device-pc-1' },
      { method: 'device.registrations.create', deviceId: 'device-pc-1' },
      { method: 'device.syncFeed.retrieve', deviceId: 'device-pc-1', params: { afterSeq: 0, limit: 100 } },
      { method: 'call.recoverRtcSession', rtcSessionId: 'rtc-startup-1', type: 'video' },
    ],
    'startup sync must use the same device id for offline messages, contacts, group members, and RTC session backfill',
  );
  assert.equal(result.deviceId, 'device-pc-1');
  assert.equal(result.recoveredRtcSessions.length, 1);
  assert.equal(result.recoveredRtcSessions[0]?.rtcSessionId, 'rtc-startup-1');
  assert.equal(result.errors.length, 0);

  console.log('sdkwork-chat-pc startup sync orchestration contract passed');
}

void main();
