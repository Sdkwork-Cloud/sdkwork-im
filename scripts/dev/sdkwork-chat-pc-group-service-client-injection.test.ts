import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/GroupService';

const calls: Array<{
  conversationId?: string;
  method: string;
  params?: Record<string, unknown>;
}> = [];

function createMember(conversationId: string, principalId: string): ConversationMember {
  return {
    attributes: {},
    conversationId,
    joinedAt: '2026-06-04T00:00:00.000Z',
    memberId: `member-${principalId}`,
    principalId,
    principalKind: 'user',
    role: principalId === 'current-user' ? 'owner' : 'member',
    state: 'joined',
    tenantId: 'tenant-1',
  };
}

const fakeClient = {
  chat: {
    inbox: {
      async retrieve(params?: Record<string, unknown>) {
        calls.push({ method: 'chat.inbox.retrieve', params });
        return {
          hasMore: false,
          items: [
            {
              conversationId: 'group-1',
              conversationType: 'group',
              lastActivityAt: '2026-06-04T10:00:00.000Z',
              lastMessageSeq: 2,
              unreadCount: 0,
            },
            {
              conversationId: 'single-1',
              conversationType: 'single',
              lastActivityAt: '2026-06-04T09:00:00.000Z',
              lastMessageSeq: 1,
              unreadCount: 0,
            },
          ],
        };
      },
    },
  },
  conversations: {
    async getPreferences(conversationId: string) {
      calls.push({ method: 'conversations.getPreferences', conversationId });
      return {
        conversationId,
        isHidden: false,
        isMarkedUnread: false,
        isMuted: false,
        isPinned: false,
        principalId: 'current-user',
        principalKind: 'user',
        tenantId: 'tenant-1',
        updatedAt: '2026-06-04T10:00:00.000Z',
      };
    },
    async getProfile(conversationId: string) {
      calls.push({ method: 'conversations.getProfile', conversationId });
      return {
        avatarUrl: `https://cdn.example.test/${conversationId}.png`,
        conversationId,
        displayName: conversationId === 'group-1' ? 'Backend Group' : 'Direct Chat',
        notice: '',
        tenantId: 'tenant-1',
        updatedAt: '2026-06-04T10:00:00.000Z',
      };
    },
    async listMembers(conversationId: string, params?: Record<string, unknown>) {
      calls.push({ method: 'conversations.listMembers', conversationId, params });
      return {
        hasMore: false,
        items: [
          createMember(conversationId, 'current-user'),
          createMember(conversationId, 'u_alice'),
        ],
      };
    },
    async updateChat() {
      throw new Error('GroupService must not call non-standard updateChat on the IM SDK client');
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient);

  const groups = await service.getGroups();

  assert.deepEqual(
    groups.map((group) => [
      group.id,
      group.name,
      group.memberCount,
      group.activeCount,
      group.members,
    ]),
    [
      ['group-1', 'Backend Group', 2, 2, ['current-user', 'u_alice']],
    ],
    'group service must hydrate groups and member state through the same injected IM SDK client',
  );
  assert.deepEqual(
    calls.map((call) => call.method),
    [
      'chat.inbox.retrieve',
      'conversations.getPreferences',
      'conversations.getPreferences',
      'conversations.getProfile',
      'conversations.getProfile',
      'conversations.listMembers',
    ],
    'group service must not bypass its injected IM SDK client through the global chatService singleton',
  );

  console.log('sdkwork-chat-pc group service client injection contract passed');
}

void main();
