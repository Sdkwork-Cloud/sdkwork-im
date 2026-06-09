import assert from 'node:assert/strict';
import type { ConversationMember, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkGroupService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/GroupService';
import type { ChatService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ChatService';

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
    async removeMember(conversationId: string, body: Record<string, unknown>) {
      calls.push({ method: 'conversations.removeMember', conversationId, body });
      return createMember(conversationId, 'u_bob');
    },
    async leave(conversationId: string) {
      calls.push({ method: 'conversations.leave', conversationId });
      return {};
    },
  },
  social: {
    users: {
      async list(params: Record<string, unknown>) {
        calls.push({ method: 'social.users.list', params });
        throw new Error('group membership must not use arbitrary social user search');
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkGroupService(() => fakeClient);

  await service.addMembers('group-1', [' u_alice ', 'u_existing', 'u_bob', 'u_alice', '']);

  assert.deepEqual(
    calls.filter((call) => call.method === 'social.users.list'),
    [],
    'group member operations must use selected address-book user ids without arbitrary user search',
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
      {
        body: {
          principalId: 'u_bob',
          principalKind: 'user',
          role: 'member',
        },
        conversationId: 'group-1',
        method: 'conversations.addMember',
      },
    ],
    'group add-member flow must invite only selected non-member contact user ids through the IM SDK',
  );

  calls.length = 0;
  await service.removeMember('group-1', 'u_existing');

  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.removeMember'),
    [
      {
        body: {
          memberId: 'member-u_existing',
        },
        conversationId: 'group-1',
        method: 'conversations.removeMember',
      },
    ],
    'group remove-member flow must resolve the backend conversation member id before removing the selected member',
  );

  const failingChatService = {
    async getChats() {
      return [
        {
          id: 'group-1',
          name: 'Original Group',
          type: 'group',
          unreadCount: 0,
          updatedAt: 1,
        },
      ];
    },
    async updateChat() {
      throw new Error('profile update failed');
    },
  } as unknown as ChatService;
  const updateService = createSdkworkGroupService(() => fakeClient, failingChatService);
  await assert.rejects(
    () => updateService.updateGroupInfo('group-1', { name: 'Failed Name' }),
    /profile update failed/u,
    'group profile updates must surface SDK failures',
  );
  const groupsAfterFailedUpdate = await updateService.getGroups();
  assert.equal(
    groupsAfterFailedUpdate[0]?.name,
    'Original Group',
    'failed group profile updates must not pollute the GroupService group projection cache',
  );

  let backendGroupName = 'Cached Group Name';
  const profileRefreshChatService = {
    async getChats() {
      return [
        {
          id: 'group-1',
          name: backendGroupName,
          type: 'group',
          unreadCount: 0,
          updatedAt: 1,
        },
      ];
    },
    async updateChat(_groupId: string, updates: Record<string, unknown>) {
      return {
        id: 'group-1',
        name: String(updates.name ?? 'Cached Group Name'),
        type: 'group',
        unreadCount: 0,
        updatedAt: 1,
      };
    },
  } as unknown as ChatService;
  const profileRefreshService = createSdkworkGroupService(() => fakeClient, profileRefreshChatService);
  await profileRefreshService.updateGroupInfo('group-1', { name: 'Cached Group Name' });
  backendGroupName = 'Backend Renamed Group';
  const groupsAfterBackendProfileRefresh = await profileRefreshService.getGroups();
  assert.equal(
    groupsAfterBackendProfileRefresh[0]?.name,
    'Backend Renamed Group',
    'group member projection refresh must not let stale cached group profile fields override the latest SDK chat profile',
  );

  backendGroupName = 'Group group-1';
  const groupsAfterFallbackProfileRefresh = await profileRefreshService.getGroups();
  assert.equal(
    groupsAfterFallbackProfileRefresh[0]?.name,
    'Cached Group Name',
    'group member projection refresh should keep the last successful group profile when the SDK chat profile falls back to a generated name',
  );

  calls.length = 0;
  const groupDeleteChatCalls: string[] = [];
  const groupDeleteChatService = {
    async deleteChat(chatId: string) {
      groupDeleteChatCalls.push(chatId);
    },
  } as unknown as ChatService;
  const deleteService = createSdkworkGroupService(() => fakeClient, groupDeleteChatService);
  await deleteService.deleteGroup('group-1');
  assert.deepEqual(
    calls.filter((call) => call.method === 'conversations.leave'),
    [
      {
        conversationId: 'group-1',
        method: 'conversations.leave',
      },
    ],
    'group leave flow must leave the SDK-backed conversation',
  );
  assert.deepEqual(
    groupDeleteChatCalls,
    ['group-1'],
    'group leave flow must also clear ChatService local view and message caches so stale group messages cannot resurrect the left group',
  );

  console.log('sdkwork-chat-pc group member contacts contract passed');
}

void main();
