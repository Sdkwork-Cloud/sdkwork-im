import type {
  InboxResponse,
  ConversationMember,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/clawchat-pc-core/sdk/imSdkClient';
import type { Chat } from '@sdkwork/clawchat-pc-types';
import { chatService, createSdkworkChatService, type ChatService } from './ChatService';
import { contactService } from './ContactService';

export interface GroupService {
  createGroup(name: string, members: string[]): Promise<Chat>;
  getGroups(): Promise<Chat[]>;
  updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat>;
  addMembers(groupId: string, memberIds: string[]): Promise<void>;
  addMembersBySearchQuery(groupId: string, memberQueries: string[]): Promise<string[]>;
  removeMember(groupId: string, memberId: string): Promise<void>;
  deleteGroup(groupId: string): Promise<void>;
  syncGroupMembers(): Promise<GroupMemberSyncChange[]>;
}

type GroupViewState = Partial<Pick<Chat, 'activeCount' | 'avatar' | 'memberCount' | 'members' | 'name' | 'notice'>>;
export type GroupMemberSyncChange = Required<Pick<GroupViewState, 'activeCount' | 'memberCount' | 'members'>> & {
  groupId: string;
};
type ConversationListEntry = InboxResponse['items'][number];
const GROUP_MEMBERS_PAGE_LIMIT = 100;
const GROUPS_PAGE_LIMIT = 100;
const SOCIAL_USER_SEARCH_LIMIT = 20;

function createGroupId(): string {
  const requestId =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `pc-group-${requestId}`;
}

function createGroupAvatar(groupId: string): string {
  return `https://api.dicebear.com/7.x/identicon/svg?seed=${encodeURIComponent(groupId)}`;
}

function normalizeGroupName(name: string, memberCount: number): string {
  const trimmedName = name.trim();
  return trimmedName || `群聊(${memberCount}人)`;
}

function uniqueMemberIds(memberIds: string[]): string[] {
  const result: string[] = [];
  const seen = new Set<string>();
  for (const memberId of memberIds) {
    const normalizedMemberId = memberId.trim();
    if (!normalizedMemberId || seen.has(normalizedMemberId)) {
      continue;
    }
    seen.add(normalizedMemberId);
    result.push(normalizedMemberId);
  }
  return result;
}

function mapActiveMemberIds(members: ConversationMember[]): string[] {
  return members
    .filter((member) => member.state === 'joined' || member.state === 'invited')
    .map((member) => member.principalId);
}

class SdkworkGroupService implements GroupService {
  private readonly groupViewState = new Map<string, GroupViewState>();
  private readonly chatClient: ChatService;

  constructor(
    private readonly getClient: () => ImSdkClient = getImSdkClientWithSession,
    chatClient?: ChatService,
  ) {
    this.chatClient = chatClient ?? (
      getClient === getImSdkClientWithSession
        ? chatService
        : createSdkworkChatService(getClient)
    );
  }

  private client(): ImSdkClient {
    return this.getClient();
  }

  private async listAllConversationMembers(groupId: string): Promise<ConversationMember[]> {
    const items: ConversationMember[] = [];
    let cursor: string | undefined;

    while (true) {
      const response = await this.client().conversations.listMembers(groupId, {
        limit: GROUP_MEMBERS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);

      if (!response.hasMore || !response.nextCursor || response.nextCursor === cursor) {
        break;
      }

      cursor = response.nextCursor;
    }

    return items;
  }

  private async listAllConversationEntries(): Promise<ConversationListEntry[]> {
    const items: ConversationListEntry[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().conversations.list({
        limit: GROUPS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  async createGroup(name: string, memberIds: string[]): Promise<Chat> {
    const currentUserId = contactService.getCurrentUser().id;
    const invitedMemberIds = uniqueMemberIds(memberIds).filter((memberId) => memberId !== currentUserId);
    const members = [currentUserId, ...invitedMemberIds];
    const groupId = createGroupId();

    const result = await this.client().conversations.create({
      conversationId: groupId,
      conversationType: 'group',
    });
    const boundGroupId = result.conversationId;

    for (const memberId of invitedMemberIds) {
      await this.client().conversations.addMember(boundGroupId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
    }

    const groupName = normalizeGroupName(name, members.length);
    const groupAvatar = createGroupAvatar(boundGroupId);
    await this.client().conversations.updateProfile(boundGroupId, {
      avatarUrl: groupAvatar,
      displayName: groupName,
    });
    await this.client().conversations.updatePreferences(boundGroupId, { isHidden: false });

    const group: Chat = {
      id: boundGroupId,
      name: groupName,
      avatar: groupAvatar,
      type: 'group',
      unreadCount: 0,
      updatedAt: Date.now(),
      memberCount: members.length,
      members,
      activeCount: members.length,
    };

    this.groupViewState.set(boundGroupId, {
      activeCount: group.activeCount,
      avatar: group.avatar,
      memberCount: group.memberCount,
      members: group.members,
      name: group.name,
      notice: group.notice,
    });
    return group;
  }

  async getGroups(): Promise<Chat[]> {
    const chats = await this.chatClient.getChats();
    const groups = chats.filter((chat) => chat.type === 'group');
    return Promise.all(groups.map((group) => this.withMemberState(group)));
  }

  async updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat> {
    const existingState = this.groupViewState.get(groupId) ?? {};
    this.groupViewState.set(groupId, {
      ...existingState,
      activeCount: updates.activeCount ?? existingState.activeCount,
      avatar: updates.avatar ?? existingState.avatar,
      memberCount: updates.memberCount ?? existingState.memberCount,
      members: updates.members ?? existingState.members,
      name: updates.name ?? existingState.name,
      notice: updates.notice ?? existingState.notice,
    });
    return this.chatClient.updateChat(groupId, {
      ...updates,
      type: 'group',
    });
  }

  async addMembers(groupId: string, memberIds: string[]): Promise<void> {
    const existingMembers = await this.listAllConversationMembers(groupId);
    const activeMemberIds = new Set(mapActiveMemberIds(existingMembers));
    const membersToAdd = uniqueMemberIds(memberIds).filter((memberId) => !activeMemberIds.has(memberId));

    for (const memberId of membersToAdd) {
      await this.client().conversations.addMember(groupId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
      activeMemberIds.add(memberId);
    }

    await this.syncMemberViewState(groupId);
  }

  async addMembersBySearchQuery(groupId: string, memberQueries: string[]): Promise<string[]> {
    const uniqueQueries = uniqueMemberIds(memberQueries);
    if (uniqueQueries.length === 0) {
      throw new Error('Group member search query is required');
    }

    const resolvedMemberIds: string[] = [];
    const unresolvedQueries: string[] = [];
    const seenMemberIds = new Set<string>();

    for (const query of uniqueQueries) {
      const response = await this.client().social.users.list({
        q: query,
        limit: SOCIAL_USER_SEARCH_LIMIT,
      });
      const [targetUser] = response.items;
      if (!targetUser) {
        unresolvedQueries.push(query);
        continue;
      }
      if (seenMemberIds.has(targetUser.userId)) {
        continue;
      }
      seenMemberIds.add(targetUser.userId);
      resolvedMemberIds.push(targetUser.userId);
    }

    if (resolvedMemberIds.length === 0) {
      throw new Error(`Group member search target not found: ${unresolvedQueries.join(', ')}`);
    }

    await this.addMembers(groupId, resolvedMemberIds);
    return resolvedMemberIds;
  }

  async removeMember(groupId: string, memberId: string): Promise<void> {
    const normalizedMemberId = memberId.trim();
    if (!normalizedMemberId) {
      throw new Error('Group member id is required');
    }

    const members = await this.listAllConversationMembers(groupId);
    const targetMember = members.find((member) => (
      member.memberId === normalizedMemberId
      || member.principalId === normalizedMemberId
    ));
    if (!targetMember) {
      throw new Error('Group member is not available');
    }

    await this.client().conversations.removeMember(groupId, {
      memberId: targetMember.memberId,
    });
    await this.syncMemberViewState(groupId);
  }

  async deleteGroup(groupId: string): Promise<void> {
    await this.client().conversations.leave(groupId);
    this.groupViewState.delete(groupId);
  }

  async syncGroupMembers(): Promise<GroupMemberSyncChange[]> {
    const entries = await this.listAllConversationEntries();
    const groupIds = entries
      .filter((entry) => entry.conversationType.toLowerCase() === 'group')
      .map((entry) => entry.conversationId);
    const changes: GroupMemberSyncChange[] = [];

    for (const groupId of groupIds) {
      const state = await this.syncMemberViewState(groupId, false);
      changes.push({
        ...state,
        groupId,
      });
    }

    return changes.sort((left, right) => left.groupId.localeCompare(right.groupId));
  }

  private async withMemberState(group: Chat): Promise<Chat> {
    try {
      const memberState = await this.syncMemberViewState(group.id, false);
      return {
        ...group,
        activeCount: memberState.activeCount,
        memberCount: memberState.memberCount,
        members: memberState.members,
        ...this.groupViewState.get(group.id),
      };
    } catch {
      return {
        ...group,
        ...this.groupViewState.get(group.id),
      };
    }
  }

  private async syncMemberViewState(
    groupId: string,
    syncChatView = true,
  ): Promise<Required<Pick<GroupViewState, 'activeCount' | 'memberCount' | 'members'>>> {
    const members = mapActiveMemberIds(await this.listAllConversationMembers(groupId));
    const existingState = this.groupViewState.get(groupId) ?? {};
    const nextState = {
      ...existingState,
      activeCount: members.length,
      memberCount: members.length,
      members,
    };
    this.groupViewState.set(groupId, nextState);
    if (syncChatView) {
      await this.chatClient.updateChat(groupId, {
        activeCount: nextState.activeCount,
        memberCount: nextState.memberCount,
        members: nextState.members,
        type: 'group',
      }).catch(() => undefined);
    }
    return {
      activeCount: nextState.activeCount,
      memberCount: nextState.memberCount,
      members: nextState.members,
    };
  }

}

export function createSdkworkGroupService(getClient?: () => ImSdkClient): GroupService {
  return new SdkworkGroupService(getClient);
}

export const groupService = createSdkworkGroupService();
