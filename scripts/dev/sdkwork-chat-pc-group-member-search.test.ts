import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/GroupService';

const calls: Array<{
  body?: Record<string, unknown>;
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
    role: 'member',
    state: 'joined',
    tenantId: 'tenant-1',
  };
}

const fakeClient = {
  conversations: {
    async listMembers(conversationId: string, params?: Record<string, unknown>) {
      calls.push({ method: 'conversations.listMembers', conversationId, params });
      return {
        hasMore: false,
        items: [
          createMember(conversationId, 'current-user'),
          createMember(conversationId, 'u_existing'),
        ],
      };
    },
    async addMember(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.addMember', conversationId, body });
      return createMember(conversationId, String(body.principalId));
    },
  },
  social: {
    users: {
      async list(params: Record<string, unknown>) {
        calls.push({ method: 'social.users.list', params });
        if (params.q === 'alice') {
          return {
            hasMore: false,
            items: [
              {
                avatarUrl: 'https://cdn.example.test/alice.png',
                displayName: 'Alice Chen',
                relationshipState: 'none',
                tenantId: 'tenant-1',
                userId: 'u_alice',
              },
            ],
          };
        }
        if (params.q === 'existing') {
          return {
            hasMore: false,
            items: [
              {
                avatarUrl: 'https://cdn.example.test/existing.png',
                displayName: 'Existing User',
                relationshipState: 'active',
                tenantId: 'tenant-1',
                userId: 'u_existing',
              },
            ],
          };
        }
        return {
          hasMore: false,
          items: [],
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient);

  await service.addMembersBySearchQuery('group-1', [' alice ', 'existing', 'missing', 'alice']);

  assert.deepEqual(
    calls.filter((call) => call.method === 'social.users.list'),
    [
      { method: 'social.users.list', params: { q: 'alice', limit: 20 } },
      { method: 'social.users.list', params: { q: 'existing', limit: 20 } },
      { method: 'social.users.list', params: { q: 'missing', limit: 20 } },
    ],
    'group add-member input must search the real backend user directory before inviting members',
  );
  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.addMember'),
    [
      {
        body: {
          principalId: 'u_alice',
          principalKind: 'user',
          role: 'member',
        },
        conversationId: 'group-1',
        method: 'conversations.addMember',
      },
    ],
    'group add-member input must invite only resolved non-member user ids through the IM SDK',
  );

  await assert.rejects(
    () => service.addMembersBySearchQuery('group-1', ['missing']),
    /not found/i,
    'group add-member input must reject unresolved users instead of treating raw text as member ids',
  );

  console.log('sdkwork-chat-pc group member search contract passed');
}

void main();
