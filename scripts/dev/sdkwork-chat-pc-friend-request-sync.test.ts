import assert from 'node:assert/strict';
import type { FriendRequest, ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ContactService';

type FriendRequestDirection = 'incoming' | 'outgoing';

type FriendRequestListParams = {
  cursor?: string;
  direction: FriendRequestDirection;
  limit?: number;
  status?: 'pending' | 'accepted' | 'declined' | 'canceled' | 'expired' | 'all';
};

const friendRequestCalls: FriendRequestListParams[] = [];
const userSearchCalls: Array<{ limit?: number; q?: string }> = [];

function createFriendRequest(
  requestId: string,
  direction: FriendRequestDirection,
): FriendRequest {
  const currentUserId = 'current-user';
  const peerUserId = `${direction}-peer-${requestId}`;
  return {
    createdAt: '2026-06-04T00:00:00.000Z',
    requestId,
    requestMessage: `request ${requestId}`,
    requesterUserId: direction === 'incoming' ? peerUserId : currentUserId,
    status: 'pending',
    targetUserId: direction === 'incoming' ? currentUserId : peerUserId,
    tenantId: 'tenant-1',
    updatedAt: '2026-06-04T00:00:00.000Z',
  };
}

function pageFriendRequests(params: FriendRequestListParams): {
  items: FriendRequest[];
  nextCursor?: string;
} {
  const page = params.cursor ?? '0';
  if (page === '0') {
    return {
      items: [createFriendRequest(`${params.direction}-1`, params.direction)],
      nextCursor: '1',
    };
  }
  if (page === '1') {
    return {
      items: [createFriendRequest(`${params.direction}-2`, params.direction)],
    };
  }
  return { items: [] };
}

const fakeClient = {
  social: {
    users: {
      async list(params: { limit?: number; q?: string }) {
        userSearchCalls.push(params);
        const userId = params.q;
        if (!userId) {
          return { items: [], hasMore: false };
        }
        return {
          items: [
            {
              avatarUrl: `https://cdn.example.test/${encodeURIComponent(userId)}.png`,
              displayName: `Profile ${userId}`,
              relationshipState: 'none',
              tenantId: 'tenant-1',
              userId,
            },
          ],
          hasMore: false,
        };
      },
    },
    friendRequests: {
      async list(params: FriendRequestListParams) {
        friendRequestCalls.push(params);
        return pageFriendRequests(params);
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkContactService(() => fakeClient);
  const requests = await service.getFriendRequests();

  const incomingCalls = friendRequestCalls.filter((call) => call.direction === 'incoming');
  const outgoingCalls = friendRequestCalls.filter((call) => call.direction === 'outgoing');

  assert.deepEqual(
    incomingCalls,
    [
      { direction: 'incoming', status: 'all', limit: 100 },
      { direction: 'incoming', status: 'all', limit: 100, cursor: '1' },
    ],
    'incoming friend request sync must continue until nextCursor is exhausted',
  );
  assert.deepEqual(
    outgoingCalls,
    [
      { direction: 'outgoing', status: 'all', limit: 100 },
      { direction: 'outgoing', status: 'all', limit: 100, cursor: '1' },
    ],
    'outgoing friend request sync must continue until nextCursor is exhausted',
  );
  assert.deepEqual(
    requests.map((request) => request.name),
    [
      'Profile incoming-peer-incoming-1',
      'Profile incoming-peer-incoming-2',
      'Profile outgoing-peer-outgoing-1',
      'Profile outgoing-peer-outgoing-2',
    ],
    'friend request list must resolve peer names through the real social user search endpoint',
  );
  assert.deepEqual(
    requests.map((request) => request.avatar),
    [
      'https://cdn.example.test/incoming-peer-incoming-1.png',
      'https://cdn.example.test/incoming-peer-incoming-2.png',
      'https://cdn.example.test/outgoing-peer-outgoing-1.png',
      'https://cdn.example.test/outgoing-peer-outgoing-2.png',
    ],
    'friend request list must resolve peer avatars through the real social user search endpoint',
  );
  assert.deepEqual(
    userSearchCalls,
    [
      { q: 'incoming-peer-incoming-1', limit: 20 },
      { q: 'incoming-peer-incoming-2', limit: 20 },
      { q: 'outgoing-peer-outgoing-1', limit: 20 },
      { q: 'outgoing-peer-outgoing-2', limit: 20 },
    ],
  );

  console.log('sdkwork-chat-pc friend request sync contract passed');
}

void main();
