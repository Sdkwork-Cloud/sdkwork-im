import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ChatService';

const calls: Array<{
  body?: Record<string, unknown>;
  conversationId?: string;
  messageId?: string;
  method: string;
  params?: {
    afterSeq?: number;
    limit?: number;
  };
}> = [];

const timelinePages = [
  {
    items: [
      {
        conversationId: 'chat-1',
        messageId: 'message-1',
        messageSeq: 1,
        summary: 'first page message',
        sender: {
          id: 'u_alice',
          kind: 'user',
          metadata: {},
        },
        body: {
          summary: 'first page message',
          parts: [{ kind: 'text', text: 'first page message' }],
          renderHints: {
            sdkworkChatPcType: 'text',
          },
        },
        messageType: 'standard',
        deliveryMode: 'discrete',
        occurredAt: '2026-06-04T10:00:00.000Z',
      },
    ],
    nextAfterSeq: 1,
    hasMore: true,
  },
  {
    items: [
      {
        conversationId: 'chat-1',
        messageId: 'message-2',
        messageSeq: 2,
        summary: 'second page message',
        sender: {
          id: 'u_bob',
          kind: 'user',
          metadata: {},
        },
        body: {
          summary: 'second page message',
          parts: [{ kind: 'text', text: 'second page body text' }],
          renderHints: {
            sdkworkChatPcType: 'text',
          },
          replyTo: {
            messageId: 'message-1',
            senderDisplayName: 'Alice',
            contentPreview: 'first page message',
          },
        },
        messageType: 'standard',
        deliveryMode: 'discrete',
        occurredAt: '2026-06-04T10:00:05.000Z',
      },
    ],
    hasMore: false,
  },
];

const fakeClient = {
  conversations: {
    async postText(
      conversationId: string,
      text: string,
      body: Record<string, unknown>,
    ) {
      calls.push({ method: 'chat.conversations.messages.create', conversationId, body });
      return {
        messageId: 'message-3',
        messageSeq: 3,
        eventId: 'event-3',
      };
    },
    async postMessage(
      conversationId: string,
      body: Record<string, unknown>,
    ) {
      calls.push({ method: 'chat.conversations.messages.create', conversationId, body });
      return {
        messageId: 'message-4',
        messageSeq: 4,
        eventId: 'event-4',
      };
    },
    async listMessages(
      conversationId: string,
      params?: {
        afterSeq?: number;
        limit?: number;
      },
    ) {
      calls.push({ method: 'chat.conversations.messages.list', conversationId, params });
      return timelinePages[calls.filter((call) => call.method === 'chat.conversations.messages.list').length - 1];
    },
  },
  messages: {
    async deleteForMe(messageId: string) {
      calls.push({ method: 'chat.messages.visibility.delete', messageId });
      return {
        tenantId: 't-demo',
        conversationId: 'chat-1',
        messageId,
        messageSeq: 1,
        principalKind: 'user',
        principalId: 'u_alice',
        isDeleted: true,
        updatedAt: '2026-06-04T10:00:10.000Z',
      };
    },
  },
} as unknown as ImSdkClient;

function assertLastMediaPost({
  content,
  coverUrl,
  duration,
  fileName,
  fileSize,
  mediaKind,
  messageType,
  sizeBytes,
}: {
  content: string;
  coverUrl?: string;
  duration?: number;
  fileName: string;
  fileSize?: string;
  mediaKind: string;
  messageType: 'file' | 'image' | 'video' | 'voice';
  sizeBytes?: string;
}): void {
  const body = calls.at(-1)?.body as Record<string, unknown>;
  const parts = body.parts as Array<Record<string, unknown>>;
  const part = parts[0];
  const drive = part.drive as Record<string, unknown>;
  const resource = part.resource as Record<string, unknown>;
  assert.equal(
    body.text,
    undefined,
    'PC media send must not persist local object URLs as plain text message bodies',
  );
  assert.equal(part.kind, 'media');
  assert.equal(part.mediaRole, 'attachment');
  assert.equal(drive.driveUri, resource.uri);
  assert.equal(resource.kind, mediaKind);
  assert.equal(resource.source, 'drive');
  assert.equal(resource.publicUrl, content);
  assert.equal(resource.fileName, fileName);
  if (sizeBytes !== undefined) {
    assert.equal(resource.sizeBytes, sizeBytes);
  }
  if (duration !== undefined) {
    assert.equal(resource.durationSeconds, duration);
  }
  assert.deepEqual(
    body.renderHints,
    {
      ...(coverUrl ? { coverUrl } : {}),
      ...(duration ? { duration: String(duration) } : {}),
      ...(fileName ? { fileName } : {}),
      ...(fileSize ? { fileSize } : {}),
      sdkworkChatPcType: messageType,
    },
    'PC media send must preserve UI metadata through render hints without changing visual components',
  );
}

async function main(): Promise<void> {
  const service = createSdkworkChatService(() => fakeClient);
  const messages = await service.getMessages('chat-1');

  assert.equal(calls.length, 2, 'chat message history must continue until the IM SDK timeline window is exhausted');
  assert.deepEqual(
    calls,
    [
      { method: 'chat.conversations.messages.list', conversationId: 'chat-1', params: { afterSeq: 0, limit: 100 } },
      { method: 'chat.conversations.messages.list', conversationId: 'chat-1', params: { afterSeq: 1, limit: 100 } },
    ],
  );
  assert.deepEqual(
    messages.map((message) => [
      message.id,
      message.senderId,
      message.content,
      message.type,
      message.timestamp,
      message.replyTo,
    ]),
    [
      ['message-1', 'u_alice', 'first page message', 'text', Date.parse('2026-06-04T10:00:00.000Z'), undefined],
      [
        'message-2',
        'u_bob',
        'second page body text',
        'text',
        Date.parse('2026-06-04T10:00:05.000Z'),
        { id: 'message-1', senderName: 'Alice', content: 'first page message' },
      ],
    ],
  );

  const replyTo = { id: 'message-2', senderName: 'Bob', content: 'second page body text' };
  const postedReply = await service.sendMessage('chat-1', 'reply text', 'text', replyTo);
  assert.deepEqual(
    calls.at(-1),
    {
      method: 'chat.conversations.messages.create',
      conversationId: 'chat-1',
      body: {
        clientMsgId: (calls.at(-1) as { body?: { clientMsgId?: string } }).body?.clientMsgId,
        summary: 'reply text',
        replyTo: {
          messageId: 'message-2',
          senderDisplayName: 'Bob',
          contentPreview: 'second page body text',
        },
      },
    },
    'PC sendMessage must persist reply references through the standard IM message body',
  );
  assert.deepEqual(
    postedReply.replyTo,
    replyTo,
    'PC sendMessage must keep reply preview in the local optimistic message',
  );

  const postedImage = await service.sendMessage('chat-1', 'blob://local-image-1', 'image', undefined, {
    fileName: 'local-image.png',
    fileSize: '4.0 KB',
    coverUrl: 'blob://local-image-cover',
  });
  assertLastMediaPost({
    content: 'blob://local-image-1',
    coverUrl: 'blob://local-image-cover',
    fileName: 'local-image.png',
    fileSize: '4.0 KB',
    mediaKind: 'image',
    messageType: 'image',
    sizeBytes: '4096',
  });
  assert.equal(postedImage.id, 'message-4');
  assert.equal(postedImage.type, 'image');

  const postedVoice = await service.sendMessage('chat-1', 'blob://local-voice-1', 'voice', undefined, {
    duration: 12,
    fileName: 'voice-message.ogg',
    fileSize: '8192',
  });
  assertLastMediaPost({
    content: 'blob://local-voice-1',
    duration: 12,
    fileName: 'voice-message.ogg',
    fileSize: '8192',
    mediaKind: 'voice',
    messageType: 'voice',
    sizeBytes: '8192',
  });
  assert.equal(postedVoice.type, 'voice');

  const postedFile = await service.sendMessage('chat-1', 'blob://local-file-1', 'file', undefined, {
    fileName: 'quarterly-report.pdf',
    fileSize: '1.5 MB',
  });
  assertLastMediaPost({
    content: 'blob://local-file-1',
    fileName: 'quarterly-report.pdf',
    fileSize: '1.5 MB',
    mediaKind: 'file',
    messageType: 'file',
    sizeBytes: '1572864',
  });
  assert.equal(postedFile.type, 'file');

  const postedVideo = await service.sendMessage('chat-1', 'blob://local-video-1', 'video', undefined, {
    coverUrl: 'blob://local-video-cover',
    duration: 42,
    fileName: 'demo-video.mp4',
    fileSize: '3 MB',
  });
  assertLastMediaPost({
    content: 'blob://local-video-1',
    coverUrl: 'blob://local-video-cover',
    duration: 42,
    fileName: 'demo-video.mp4',
    fileSize: '3 MB',
    mediaKind: 'video',
    messageType: 'video',
    sizeBytes: '3145728',
  });
  assert.equal(postedVideo.type, 'video');

  await service.deleteMessage('chat-1', 'message-1');
  assert.deepEqual(
    calls.at(-1),
    { method: 'chat.messages.visibility.delete', messageId: 'message-1' },
    'PC deleteMessage must persist current-user message deletion through the standard IM SDK visibility API',
  );

  console.log('sdkwork-chat-pc message sync contract passed');
}

void main();
