import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ChatService';

type DeviceSyncParams = {
  afterSeq?: number;
  limit?: number;
};

const calls: Array<{
  body?: Record<string, unknown>;
  deviceId?: string;
  method: string;
  params?: DeviceSyncParams;
}> = [];

const fakeClient = {
  conversations: {
    async listMessages() {
      return {
        hasMore: false,
        items: [],
      };
    },
    async updateReadCursor(_conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'chat.conversations.readCursor.update', body });
      return {
        readSeq: body.readSeq,
      };
    },
    async updatePreferences(_conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'chat.conversations.preferences.update', body });
      return body;
    },
  },
  device: {
    registrations: {
      async create(body: Record<string, unknown>) {
        calls.push({ method: 'device.registrations.create', body });
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
        if ((params?.afterSeq ?? 0) === 0) {
          return {
            items: [
              {
                actorId: 'u_bob',
                actorKind: 'user',
                conversationId: 'chat-1',
                deviceId,
                messageId: 'message-offline-1',
                messageSeq: 41,
                occurredAt: '2026-06-04T10:00:00.000Z',
                originEventId: 'evt-message-offline-1',
                originEventType: 'message.posted',
                principalId: 'u_alice',
                summary: 'offline window message',
                syncSeq: 1,
                tenantId: 'tenant-1',
              },
              {
                actorId: 'u_bob',
                actorKind: 'user',
                conversationId: 'chat-1',
                deviceId,
                messageId: 'message-offline-image-1',
                messageSeq: 42,
                occurredAt: '2026-06-04T10:00:02.000Z',
                originEventId: 'evt-message-offline-image-1',
                originEventType: 'message.posted',
                payloadSchema: 'message.posted.v1',
                payload: JSON.stringify({
                  body: {
                    parts: [
                      {
                        kind: 'media',
                        resource: {
                          fileName: 'offline-image.png',
                          kind: 'image',
                          publicUrl: 'https://cdn.example.test/offline-image.png',
                          sizeBytes: 4096,
                        },
                      },
                    ],
                    renderHints: {
                      fileName: 'offline-image.png',
                      sdkworkChatPcType: 'image',
                    },
                    replyTo: {
                      messageId: 'message-offline-1',
                      senderDisplayName: 'Bob',
                      contentPreview: 'offline window message',
                    },
                    summary: 'offline image',
                  },
                  messageType: 'standard',
                  sender: {
                    id: 'u_bob',
                    kind: 'user',
                    metadata: {},
                  },
                }),
                principalId: 'u_alice',
                summary: 'offline image',
                syncSeq: 2,
                tenantId: 'tenant-1',
              },
            ],
            nextAfterSeq: 2,
            hasMore: true,
            trimmedThroughSeq: 0,
          };
        }

        return {
          items: [
            {
              actorId: 'u_alice',
              actorKind: 'user',
              conversationId: 'chat-1',
              deviceId,
              lastReadMessageId: 'message-offline-1',
              occurredAt: '2026-06-04T10:00:05.000Z',
              originEventId: 'evt-read-1',
              originEventType: 'conversation.read_cursor_updated',
              principalId: 'u_alice',
              readSeq: 42,
              syncSeq: 3,
              tenantId: 'tenant-1',
            },
          ],
          nextAfterSeq: 3,
          hasMore: false,
          trimmedThroughSeq: 0,
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);

  const result = await service.syncDeviceFeed('device-pc-1');

  assert.deepEqual(
    calls.filter((call) => call.method === 'device.registrations.create' || call.method === 'device.syncFeed.retrieve'),
    [
      { method: 'device.registrations.create', body: { deviceId: 'device-pc-1' } },
      { method: 'device.syncFeed.retrieve', deviceId: 'device-pc-1', params: { afterSeq: 0, limit: 100 } },
      { method: 'device.syncFeed.retrieve', deviceId: 'device-pc-1', params: { afterSeq: 2, limit: 100 } },
    ],
    'chat service must register the current IM device before paging the standard IM device sync feed window',
  );
  assert.deepEqual(
    result,
    {
      appliedMessages: 2,
      appliedReadCursors: 1,
      deviceId: 'device-pc-1',
      nextAfterSeq: 3,
      trimmedThroughSeq: 0,
    },
    'chat service must report deterministic device sync feed progress',
  );

  const messages = await service.getMessages('chat-1');
  assert.deepEqual(
    messages.map((message) => [
      message.id,
      message.chatId,
      message.senderId,
      message.content,
      message.type,
      message.timestamp,
    ]),
    [
      [
        'message-offline-1',
        'chat-1',
        'u_bob',
        'offline window message',
        'text',
        Date.parse('2026-06-04T10:00:00.000Z'),
      ],
      [
        'message-offline-image-1',
        'chat-1',
        'u_bob',
        'https://cdn.example.test/offline-image.png',
        'image',
        Date.parse('2026-06-04T10:00:02.000Z'),
      ],
    ],
    'offline device sync feed messages must hydrate complete message body projection without realtime delivery',
  );
  assert.equal(messages[1].fileName, 'offline-image.png');
  assert.equal(messages[1].fileSize, '4096');
  assert.deepEqual(messages[1].replyTo, {
    id: 'message-offline-1',
    senderName: 'Bob',
    content: 'offline window message',
  });

  await service.markAsRead('chat-1');
  assert.deepEqual(
    calls.slice(-2),
    [
      { method: 'chat.conversations.readCursor.update', body: { readSeq: 42 } },
      { method: 'chat.conversations.preferences.update', body: { isMarkedUnread: false } },
    ],
    'device sync feed read/message sequence state must drive the standard read cursor update',
  );

  console.log('sdkwork-chat-pc device sync feed contract passed');
}

void main();
