import type {
  InboxResponse,
  ConversationMember,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/clawchat-pc-core/sdk/imSdkClient';
import type { Chat, Message, User } from '@sdkwork/clawchat-pc-types';
import { chatService, createSdkworkChatService, type ChatService } from './ChatService';
import { contactService } from './ContactService';

export interface GroupService {
  createGroup(name: string, members: string[]): Promise<Chat>;
  getGroups(): Promise<Chat[]>;
  updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat>;
  addMembers(groupId: string, memberIds: string[]): Promise<void>;
  inviteUserToGroup(group: Chat, targetUser: User): Promise<Message>;
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
export const GROUP_INVITE_DESCRIPTOR_PREFIX = 'group-invite:';

export interface GroupInviteDescriptor {
  groupAvatar?: string;
  groupId: string;
  groupName?: string;
  inviterId?: string;
  kind: 'group_invite';
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function parseJsonRecord(value: unknown): Record<string, unknown> | undefined {
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(value);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : undefined;
  } catch {
    return undefined;
  }
}

function buildGroupInviteUrl(groupId: string): string {
  return `sdkwork-chat://groups/${encodeURIComponent(groupId)}`;
}

function readGroupIdFromInviteUrl(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }
  const match = /^sdkwork-chat:\/\/groups\/([^/?#]+)/u.exec(value.trim());
  if (!match?.[1]) {
    return undefined;
  }
  try {
    return decodeURIComponent(match[1]);
  } catch {
    return match[1];
  }
}

function buildGroupInviteDescriptor(group: Chat, inviterId: string): string {
  const descriptor: GroupInviteDescriptor = {
    groupId: group.id,
    kind: 'group_invite',
    ...(group.avatar ? { groupAvatar: group.avatar } : {}),
    ...(group.name ? { groupName: group.name } : {}),
    ...(inviterId ? { inviterId } : {}),
  };
  return `${GROUP_INVITE_DESCRIPTOR_PREFIX}${encodeURIComponent(JSON.stringify(descriptor))}`;
}

export function parseGroupInviteDescriptor(message: Message): GroupInviteDescriptor | undefined {
  if (message.type !== 'card') {
    return undefined;
  }

  if (message.desc?.startsWith(GROUP_INVITE_DESCRIPTOR_PREFIX)) {
    const payload = message.desc.slice(GROUP_INVITE_DESCRIPTOR_PREFIX.length);
    try {
      const parsed = parseJsonRecord(decodeURIComponent(payload));
      const groupId = pickString(parsed?.groupId);
      if (groupId) {
        return {
          groupId,
          kind: 'group_invite',
          ...(pickString(parsed?.groupAvatar) ? { groupAvatar: pickString(parsed?.groupAvatar) } : {}),
          ...(pickString(parsed?.groupName) ? { groupName: pickString(parsed?.groupName) } : {}),
          ...(pickString(parsed?.inviterId) ? { inviterId: pickString(parsed?.inviterId) } : {}),
        };
      }
    } catch {
      return undefined;
    }
  }

  const groupId = readGroupIdFromInviteUrl(message.content);
  return groupId
    ? {
        groupId,
        kind: 'group_invite',
      }
    : undefined;
}

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

function isGeneratedGroupName(group: Chat): boolean {
  return group.name === `Group ${group.id}`;
}

function mergeCachedGroupViewState(group: Chat, state: GroupViewState | undefined): Chat {
  return {
    ...group,
    ...(group.activeCount === undefined && state?.activeCount !== undefined ? { activeCount: state.activeCount } : {}),
    ...(group.avatar === undefined && state?.avatar !== undefined ? { avatar: state.avatar } : {}),
    ...(group.memberCount === undefined && state?.memberCount !== undefined ? { memberCount: state.memberCount } : {}),
    ...(group.members === undefined && state?.members !== undefined ? { members: state.members } : {}),
    ...(isGeneratedGroupName(group) && state?.name !== undefined ? { name: state.name } : {}),
    ...(group.notice === undefined && state?.notice !== undefined ? { notice: state.notice } : {}),
  };
}

function mapConversationEntryToGroup(entry: ConversationListEntry): Chat {
  const updatedAt = new Date(entry.lastActivityAt).getTime();
  return {
    id: entry.conversationId,
    name: `Group ${entry.conversationId}`,
    avatar: createGroupAvatar(entry.conversationId),
    type: 'group',
    unreadCount: entry.unreadCount,
    updatedAt: Number.isFinite(updatedAt) ? updatedAt : Date.now(),
  };
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

  private async hydrateConversationEntryGroup(entry: ConversationListEntry): Promise<Chat | null> {
    const group = mapConversationEntryToGroup(entry);
    try {
      const preferences = await this.client().conversations.getPreferences(entry.conversationId);
      if (preferences.isHidden) {
        return null;
      }
    } catch {
      // Keep newly available groups visible when preference hydration is temporarily unavailable.
    }

    try {
      const profile = await this.client().conversations.getProfile(entry.conversationId);
      return {
        ...group,
        ...(profile.displayName ? { name: profile.displayName } : {}),
        ...(profile.avatarUrl ? { avatar: profile.avatarUrl } : {}),
        notice: profile.notice,
      };
    } catch {
      return group;
    }
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

    const groupName = normalizeGroupName(name, members.length);
    const groupAvatar = createGroupAvatar(boundGroupId);
    await this.client().conversations.updateProfile(boundGroupId, {
      avatarUrl: groupAvatar,
      displayName: groupName,
    });
    await this.client().conversations.updatePreferences(boundGroupId, { isHidden: false });

    for (const memberId of invitedMemberIds) {
      await this.client().conversations.addMember(boundGroupId, {
        principalId: memberId,
        principalKind: 'user',
        role: 'member',
      });
    }

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
    const groupsById = new Map(chats
      .filter((chat) => chat.type === 'group')
      .map((group) => [group.id, group]));
    const entries = await this.listAllConversationEntries().catch(() => []);
    for (const entry of entries) {
      if (entry.conversationType.toLowerCase() !== 'group' || groupsById.has(entry.conversationId)) {
        continue;
      }
      const group = await this.hydrateConversationEntryGroup(entry);
      if (group) {
        groupsById.set(entry.conversationId, group);
      }
    }
    const groups = await Promise.all(Array.from(groupsById.values()).map((group) => this.withMemberState(group)));
    return groups.sort((left, right) => right.updatedAt - left.updatedAt);
  }

  async updateGroupInfo(groupId: string, updates: Partial<Chat>): Promise<Chat> {
    const updatedGroup = await this.chatClient.updateChat(groupId, {
      ...updates,
      type: 'group',
    });
    const existingState = this.groupViewState.get(groupId) ?? {};
    this.groupViewState.set(groupId, {
      ...existingState,
      activeCount: updatedGroup.activeCount ?? updates.activeCount ?? existingState.activeCount,
      avatar: updatedGroup.avatar ?? updates.avatar ?? existingState.avatar,
      memberCount: updatedGroup.memberCount ?? updates.memberCount ?? existingState.memberCount,
      members: updatedGroup.members ?? updates.members ?? existingState.members,
      name: updatedGroup.name ?? updates.name ?? existingState.name,
      notice: updatedGroup.notice ?? updates.notice ?? existingState.notice,
    });
    return updatedGroup;
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

  async inviteUserToGroup(group: Chat, targetUser: User): Promise<Message> {
    const targetUserId = targetUser.id.trim();
    if (!targetUserId) {
      throw new Error('Group invite target user id is required');
    }

    await this.addMembers(group.id, [targetUserId]);
    const directChat = await this.chatClient.startDirectChat({
      avatar: targetUser.avatar,
      conversationId: targetUser.conversationId,
      directChatId: targetUser.directChatId,
      id: targetUserId,
      name: targetUser.name,
    });
    const currentUserId = contactService.getCurrentUser().id;
    return this.chatClient.sendMessage(
      directChat.id,
      buildGroupInviteUrl(group.id),
      'card',
      undefined,
      {
        appIcon: group.avatar,
        desc: buildGroupInviteDescriptor(group, currentUserId),
        fileName: '邀请你加入群聊',
      },
    );
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
    await this.chatClient.deleteChat(groupId).catch(() => undefined);
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
        ...mergeCachedGroupViewState(group, this.groupViewState.get(group.id)),
        activeCount: memberState.activeCount,
        memberCount: memberState.memberCount,
        members: memberState.members,
      };
    } catch {
      return mergeCachedGroupViewState(group, this.groupViewState.get(group.id));
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

export function createSdkworkGroupService(getClient?: () => ImSdkClient, chatClient?: ChatService): GroupService {
  return new SdkworkGroupService(getClient, chatClient);
}

export const groupService = createSdkworkGroupService();
