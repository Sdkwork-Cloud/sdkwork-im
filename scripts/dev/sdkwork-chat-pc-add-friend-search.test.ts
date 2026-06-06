import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ContactService';

const calls: Array<{ body?: Record<string, unknown>; method: string; params?: Record<string, unknown> }> = [];

const fakeClient = {
  chat: {
    contacts: {
      async list() {
        calls.push({ method: 'chat.contacts.list' });
        return { items: [], hasMore: false };
      },
    },
  },
  social: {
    users: {
      async list(params: Record<string, unknown>) {
        calls.push({ method: 'social.users.list', params });
        if (params.q === 'alice') {
          return {
            items: [
              {
                userId: 'u_alice',
                displayName: 'Alice',
                avatarUrl: 'https://example.com/alice.png',
                email: 'alice@example.com',
                phone: '+12025550100',
                relationshipState: 'none',
              },
            ],
            hasMore: false,
          };
        }
        return {
          items: [],
          hasMore: false,
        };
      },
    },
    friendRequests: {
      async create(body: Record<string, unknown>) {
        calls.push({ method: 'social.friendRequests.create', body });
        return {
          friendRequest: {
            requestId: 'fr_alice',
            requesterUserId: 'current-user',
            targetUserId: body.targetUserId,
            status: 'pending',
          },
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkContactService(() => fakeClient);

  const results = await service.searchContacts(' alice ');

  assert.deepEqual(
    calls.at(-1),
    {
      method: 'social.users.list',
      params: { q: 'alice', limit: 20 },
    },
    'add-friend search must query the generated IM SDK social user search endpoint',
  );
  assert.deepEqual(
    results.map((user) => [user.id, user.name, user.email, user.phone]),
    [['u_alice', 'Alice', 'alice@example.com', '+12025550100']],
    'add-friend search must map backend user search results into selectable contacts',
  );

  const missing = await service.searchContacts('does-not-exist');
  assert.deepEqual(
    missing,
    [],
    'add-friend search must not synthesize mock users when the backend returns no matches',
  );
  assert.equal(
    await service.getUserById('does-not-exist'),
    null,
    'getUserById must not synthesize mock users when the backend user search has no match',
  );
  assert.equal(
    calls.filter((call) => call.method === 'social.friendRequests.create').length,
    0,
    'searching for a missing user must not create a friend request',
  );

  const added = await service.addFriendBySearchQuery(' alice ');
  assert.equal(added.id, 'u_alice', 'direct add-by-input must use the backend search result user id');
  assert.deepEqual(
    calls.slice(-2),
    [
      {
        method: 'social.users.list',
        params: { q: 'alice', limit: 20 },
      },
      {
        method: 'social.friendRequests.create',
        body: { targetUserId: 'u_alice' },
      },
    ],
    'direct add-by-input must search real users before creating a friend request',
  );

  await assert.rejects(
    () => service.addFriendBySearchQuery('does-not-exist'),
    /not found/i,
    'direct add-by-input must reject missing users instead of treating the raw input as a target user id',
  );

  console.log('sdkwork-chat-pc add-friend search contract passed');
}

void main();
