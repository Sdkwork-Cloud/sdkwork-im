import type {
  ContactPreferencesView,
  ContactTagView,
  ContactView,
  CreateContactTagRequest,
  FriendRequest as ImFriendRequest,
  ImSdkClient,
  SocialUserSearchResult,
  UpdateContactTagRequest,
} from '@sdkwork/im-sdk';
import {
  getImSdkClientWithSession,
  readAppSdkSessionTokens,
} from '@sdkwork/clawchat-pc-core';
import type { User } from '@sdkwork/clawchat-pc-types';
import {
  parseDeviceSyncPayload,
  pickDeviceSyncString,
  resolveSdkworkChatPcDeviceId,
  retrieveDeviceSyncFeedWindow,
} from './DeviceSyncFeedService';
import {
  organizationDirectoryService,
  type OrganizationDirectoryService,
} from './OrganizationDirectoryService';

export interface OrgDepartment {
  id: string;
  name: string;
  parentId: string | null;
  order: number;
}

export interface FriendRequest {
  avatar?: string;
  id: number;
  name: string;
  msg: string;
  status: 'pending' | 'added' | 'rejected';
}

export interface ContactTag {
  id: string;
  name: string;
  color: string;
  count: number;
  bg: string;
  border: string;
}

export interface ContactDeviceSyncResult {
  added: User[];
  deviceId: string;
  nextAfterSeq: number;
  removedUserIds: string[];
  trimmedThroughSeq: number;
}

export interface ContactService {
  getContacts(): Promise<User[]>;
  searchContacts(query: string): Promise<User[]>;
  addFriend(userId: string): Promise<void>;
  addFriendBySearchQuery(query: string): Promise<User>;
  getStarredContacts(): Promise<User[]>;
  getDepartments(): Promise<OrgDepartment[]>;
  getUsersByDepartment(departmentId: string): Promise<User[]>;
  getCurrentUser(): User;
  getUserById(id: string): Promise<User | null>;
  getFriendRequests(): Promise<FriendRequest[]>;
  getTags(): Promise<ContactTag[]>;
  addTag(tag: Omit<ContactTag, 'id'>): Promise<ContactTag>;
  updateTag(id: string, updates: Partial<ContactTag>): Promise<ContactTag>;
  removeTag(id: string): Promise<void>;
  updateProfile(update: Partial<User>): Promise<User>;
  deleteContact(userId: string): Promise<void>;
  handleFriendRequest(requestId: number, action: 'accept' | 'reject'): Promise<void>;
  toggleStarContact(userId: string, isStarred: boolean): Promise<void>;
  setContactRemark(userId: string, remark: string): Promise<void>;
  addToBlacklist(userId: string): Promise<void>;
  recommendToFriend(userId: string): Promise<void>;
  syncContactsFromDeviceFeed(deviceId?: string): Promise<ContactDeviceSyncResult>;
}

const CONTACT_DEVICE_SYNC_NAMESPACE = 'contacts';
const CONTACTS_PAGE_LIMIT = 100;
const CONTACT_PREFERENCES_BATCH_SIZE = 20;
const CONTACT_PROFILE_BATCH_SIZE = 20;
const CONTACT_TAGS_PAGE_LIMIT = 100;
const FRIEND_REQUESTS_PAGE_LIMIT = 100;
const SOCIAL_USER_SEARCH_LIMIT = 20;

function normalizeString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    const normalized = normalizeString(value);
    if (normalized) {
      return normalized;
    }
  }
  return undefined;
}

function pickNumber(...values: unknown[]): number | undefined {
  for (const value of values) {
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const parsed = Number(value);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return undefined;
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function createAvatar(seed: string): string {
  return `https://api.dicebear.com/7.x/avataaars/svg?seed=${encodeURIComponent(seed)}`;
}

function createSearchKey(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .normalize('NFKD')
    .replace(/[^\da-z]+/gu, '');
}

function toRequestUiId(requestId: string): number {
  if (/^\d+$/u.test(requestId)) {
    const numericId = Number(requestId);
    if (Number.isSafeInteger(numericId) && numericId > 0) {
      return numericId;
    }
  }

  let hash = 2166136261;
  for (const char of requestId) {
    hash ^= char.codePointAt(0) ?? 0;
    hash = Math.imul(hash, 16777619);
  }
  return (hash >>> 0) || 1;
}

function normalizeRequestStatus(status: ImFriendRequest['status']): FriendRequest['status'] {
  if (status === 'pending') {
    return 'pending';
  }
  if (status === 'accepted') {
    return 'added';
  }
  return 'rejected';
}

class SdkworkContactService implements ContactService {
  private readonly contactByUserId = new Map<string, ContactView>();
  private readonly contactDeviceSyncAfterSeq = new Map<string, number>();
  private readonly preferenceByUserId = new Map<string, ContactPreferencesView>();
  private readonly requestIdByUiId = new Map<number, string>();
  private readonly requestUiIdByBackendId = new Map<string, number>();
  private readonly userCache = new Map<string, User>();
  private currentUserOverrides: Partial<User> = {};

  constructor(
    private readonly getClient: () => ImSdkClient = getImSdkClientWithSession,
    _getAppClient?: () => unknown,
    private readonly getOrganizationDirectoryService: () => OrganizationDirectoryService = () => organizationDirectoryService,
  ) {}

  private client(): ImSdkClient {
    return this.getClient();
  }

  private organizationDirectory(): OrganizationDirectoryService {
    return this.getOrganizationDirectoryService();
  }

  private async listAllContacts(): Promise<ContactView[]> {
    const items: ContactView[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().chat.contacts.list({
        limit: CONTACTS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  private async listAllFriendRequests(direction: 'incoming' | 'outgoing'): Promise<ImFriendRequest[]> {
    const items: ImFriendRequest[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().social.friendRequests.list({
        direction,
        status: 'all',
        limit: FRIEND_REQUESTS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      if (!response.nextCursor || response.nextCursor === cursor) {
        break;
      }
      cursor = response.nextCursor;
    } while (cursor);

    return items;
  }

  private async listAllContactTags(): Promise<ContactTagView[]> {
    const items: ContactTagView[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().social.contacts.tags.list({
        limit: CONTACT_TAGS_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      if (!response.nextCursor || response.nextCursor === cursor) {
        break;
      }
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  async getContacts(): Promise<User[]> {
    const contacts = await this.listAllContacts();
    const preferences = await this.loadContactPreferences(contacts);
    await this.loadContactPeerProfiles(contacts);
    const users = contacts
      .map((contact) => this.mapContactViewToUser(contact, preferences.get(contact.targetUserId)))
      .filter((user) => !(preferences.get(user.id)?.isBlocked ?? false));
    return users.sort((left, right) => left.name.localeCompare(right.name));
  }

  async searchContacts(query: string): Promise<User[]> {
    const normalizedQuery = query.trim();
    if (!normalizedQuery) {
      return [];
    }

    const response = await this.client().social.users.list({
      q: normalizedQuery,
      limit: SOCIAL_USER_SEARCH_LIMIT,
    });
    return response.items.map((item) => this.mapSocialUserSearchResultToUser(item));
  }

  async addFriend(userId: string): Promise<void> {
    const targetUserId = userId.trim();
    if (!targetUserId) {
      throw new Error('Friend user id is required');
    }
    await this.client().social.friendRequests.create({ targetUserId });
  }

  async addFriendBySearchQuery(query: string): Promise<User> {
    const normalizedQuery = query.trim();
    if (!normalizedQuery) {
      throw new Error('Friend search query is required');
    }

    const [targetUser] = await this.searchContacts(normalizedQuery);
    if (!targetUser) {
      throw new Error('Friend search target not found');
    }

    await this.addFriend(targetUser.id);
    return targetUser;
  }

  async getStarredContacts(): Promise<User[]> {
    const contacts = await this.getContacts();
    return contacts.filter((user) => this.preferenceByUserId.get(user.id)?.isStarred ?? false);
  }

  async getDepartments(): Promise<OrgDepartment[]> {
    return this.organizationDirectory().getDepartments();
  }

  async getUsersByDepartment(departmentId: string): Promise<User[]> {
    return this.organizationDirectory().getUsersByDepartment(departmentId);
  }

  getCurrentUser(): User {
    const session = readAppSdkSessionTokens();
    const sessionUser = session?.user;
    const id = pickString(
      sessionUser?.userId,
      sessionUser?.id,
      session?.context?.userId,
      this.currentUserOverrides.id,
      'current-user',
    ) ?? 'current-user';
    const name = pickString(
      this.currentUserOverrides.name,
      sessionUser?.displayName,
      sessionUser?.nickname,
      sessionUser?.name,
      sessionUser?.username,
      id,
    ) ?? id;
    return {
      id,
      name,
      avatar: pickString(this.currentUserOverrides.avatar, sessionUser?.avatar) ?? createAvatar(id),
      status: this.currentUserOverrides.status ?? 'online',
      email: pickString(this.currentUserOverrides.email, sessionUser?.email),
      phone: pickString(this.currentUserOverrides.phone, sessionUser?.phone),
      py: createSearchKey(name),
    };
  }

  async getUserById(id: string): Promise<User | null> {
    const normalizedId = id.trim();
    if (!normalizedId) {
      return null;
    }
    const currentUser = this.getCurrentUser();
    if (currentUser.id === normalizedId) {
      return currentUser;
    }
    const cached = this.userCache.get(normalizedId);
    if (cached) {
      return { ...cached };
    }
    const contacts = await this.getContacts();
    const contact = contacts.find((user) => user.id === normalizedId);
    if (contact) {
      return contact;
    }
    const [searchedUser] = await this.searchContacts(normalizedId);
    return searchedUser?.id === normalizedId ? searchedUser : null;
  }

  async getFriendRequests(): Promise<FriendRequest[]> {
    const [incoming, outgoing] = await Promise.all([
      this.listAllFriendRequests('incoming'),
      this.listAllFriendRequests('outgoing'),
    ]);
    const requests = [...incoming, ...outgoing];
    await this.loadFriendRequestPeerProfiles(requests);
    return requests.map((request) => this.mapFriendRequest(request));
  }

  async getTags(): Promise<ContactTag[]> {
    const tags = await this.listAllContactTags();
    return tags.map((tag) => this.mapContactTagViewToContactTag(tag));
  }

  async addTag(tag: Omit<ContactTag, 'id'>): Promise<ContactTag> {
    const created = await this.client().social.contacts.tags.create(
      this.mapContactTagInputToCreateRequest(tag),
    );
    return this.mapContactTagViewToContactTag(created);
  }

  async updateTag(id: string, updates: Partial<ContactTag>): Promise<ContactTag> {
    const tagId = this.normalizeContactTagId(id);
    const updated = await this.client().social.contacts.tags.update(
      tagId,
      this.mapContactTagUpdateToRequest(updates),
    );
    return this.mapContactTagViewToContactTag(updated);
  }

  async removeTag(id: string): Promise<void> {
    await this.client().social.contacts.tags.delete(this.normalizeContactTagId(id));
  }

  async updateProfile(update: Partial<User>): Promise<User> {
    this.currentUserOverrides = {
      ...this.currentUserOverrides,
      ...update,
    };
    return this.getCurrentUser();
  }

  async deleteContact(userId: string): Promise<void> {
    const normalizedUserId = userId.trim();
    if (!normalizedUserId) {
      throw new Error('Contact user id is required');
    }

    let contact = this.contactByUserId.get(normalizedUserId);
    if (!contact) {
      await this.getContacts();
      contact = this.contactByUserId.get(normalizedUserId);
    }
    if (!contact?.friendshipId) {
      throw new Error('Contact friendship is not available');
    }

    await this.client().social.friendships.remove(contact.friendshipId);
    this.contactByUserId.delete(normalizedUserId);
    this.userCache.delete(normalizedUserId);
    this.preferenceByUserId.delete(normalizedUserId);
  }

  async handleFriendRequest(requestId: number, action: 'accept' | 'reject'): Promise<void> {
    const backendRequestId = this.requestIdByUiId.get(requestId) ?? String(requestId);
    if (action === 'accept') {
      const result = await this.client().social.friendRequests.accept(backendRequestId);
      this.requestIdByUiId.delete(requestId);
      this.requestUiIdByBackendId.delete(backendRequestId);
      const userId = this.resolveFriendshipPeerId(result.friendship);
      if (userId) {
        await this.loadUserProfile(userId);
      }
      return;
    }

    await this.client().social.friendRequests.decline(backendRequestId);
    this.requestIdByUiId.delete(requestId);
    this.requestUiIdByBackendId.delete(backendRequestId);
  }

  async toggleStarContact(userId: string, isStarred: boolean): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const preferences = await this.client().social.contacts.preferences.update(normalizedUserId, {
      isStarred,
    });
    this.preferenceByUserId.set(normalizedUserId, preferences);
  }

  async setContactRemark(userId: string, remark: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const normalizedRemark = remark.trim();
    const preferences = await this.client().social.contacts.preferences.update(normalizedUserId, {
      remark: normalizedRemark,
    });
    this.preferenceByUserId.set(normalizedUserId, preferences);
    const cached = this.userCache.get(normalizedUserId);
    if (cached) {
      this.userCache.set(normalizedUserId, {
        ...cached,
        name: preferences.remark || normalizedUserId,
        py: createSearchKey(preferences.remark || normalizedUserId),
      });
    }
  }

  async addToBlacklist(userId: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    const preferences = await this.client().social.contacts.preferences.update(normalizedUserId, {
      isBlocked: true,
      isStarred: false,
    });
    this.preferenceByUserId.set(normalizedUserId, preferences);
  }

  async recommendToFriend(userId: string): Promise<void> {
    const normalizedUserId = this.normalizeContactUserId(userId);
    await this.client().social.contacts.recommendations.create(normalizedUserId, {});
  }

  async syncContactsFromDeviceFeed(deviceId = resolveSdkworkChatPcDeviceId()): Promise<ContactDeviceSyncResult> {
    const window = await retrieveDeviceSyncFeedWindow(
      this.client(),
      CONTACT_DEVICE_SYNC_NAMESPACE,
      deviceId,
      this.contactDeviceSyncAfterSeq,
    );
    const added: User[] = [];
    const removedUserIds: string[] = [];

    for (const entry of window.entries) {
      if (entry.originEventType === 'friendship.activated') {
        const payload = parseDeviceSyncPayload(entry);
        const targetUserId = this.resolveFriendshipPeerIdFromPayload(payload);
        if (!targetUserId) {
          continue;
        }
        const contact = this.buildContactViewFromFriendshipPayload(entry.tenantId, payload, targetUserId);
        const user = this.mapContactViewToUser(contact, this.preferenceByUserId.get(targetUserId));
        added.push(user);
        continue;
      }

      if (entry.originEventType === 'friendship.removed') {
        const targetUserId = this.resolveFriendshipPeerIdFromPayload(parseDeviceSyncPayload(entry));
        if (!targetUserId) {
          continue;
        }
        this.contactByUserId.delete(targetUserId);
        this.userCache.delete(targetUserId);
        this.preferenceByUserId.delete(targetUserId);
        removedUserIds.push(targetUserId);
      }
    }

    return {
      added,
      deviceId: window.deviceId,
      nextAfterSeq: window.nextAfterSeq,
      removedUserIds,
      trimmedThroughSeq: window.trimmedThroughSeq,
    };
  }

  private async loadContactPreferences(contacts: ContactView[]): Promise<Map<string, ContactPreferencesView>> {
    const entries: Array<readonly [string, ContactPreferencesView]> = [];
    for (let offset = 0; offset < contacts.length; offset += CONTACT_PREFERENCES_BATCH_SIZE) {
      const batch = contacts.slice(offset, offset + CONTACT_PREFERENCES_BATCH_SIZE);
      entries.push(...await Promise.all(batch.map(async (contact) => {
        const preferences = await this.client().social.contacts.preferences.retrieve(contact.targetUserId);
        return [contact.targetUserId, preferences] as const;
      })));
    }
    const preferencesByUserId = new Map(entries);
    for (const [userId, preferences] of preferencesByUserId) {
      this.preferenceByUserId.set(userId, preferences);
    }
    return preferencesByUserId;
  }

  private async loadContactPeerProfiles(contacts: ContactView[]): Promise<void> {
    const userIds = [...new Set(contacts.map((contact) => contact.targetUserId))]
      .filter((userId) => !this.userCache.has(userId));

    for (let offset = 0; offset < userIds.length; offset += CONTACT_PROFILE_BATCH_SIZE) {
      const batch = userIds.slice(offset, offset + CONTACT_PROFILE_BATCH_SIZE);
      await Promise.all(batch.map((userId) => this.loadUserProfile(userId)));
    }
  }

  private async loadUserProfile(userId: string): Promise<User | null> {
    const [profile] = await this.searchContacts(userId);
    return profile?.id === userId ? profile : null;
  }

  private normalizeContactUserId(userId: string): string {
    const normalizedUserId = userId.trim();
    if (!normalizedUserId) {
      throw new Error('Contact user id is required');
    }
    return normalizedUserId;
  }

  private normalizeContactTagId(tagId: string): string {
    const normalizedTagId = tagId.trim();
    if (!normalizedTagId) {
      throw new Error('Contact tag id is required');
    }
    return normalizedTagId;
  }

  private mapContactTagViewToContactTag(tag: ContactTagView): ContactTag {
    return {
      id: tag.tagId,
      name: tag.name,
      color: tag.color,
      count: tag.count,
      bg: tag.bg,
      border: tag.border,
    };
  }

  private mapContactTagInputToCreateRequest(tag: Omit<ContactTag, 'id'>): CreateContactTagRequest {
    return {
      name: tag.name,
      color: tag.color,
      count: tag.count,
      bg: tag.bg,
      border: tag.border,
    };
  }

  private mapContactTagUpdateToRequest(updates: Partial<ContactTag>): UpdateContactTagRequest {
    const request: UpdateContactTagRequest = {};
    if (updates.name !== undefined) {
      request.name = updates.name;
    }
    if (updates.color !== undefined) {
      request.color = updates.color;
    }
    if (updates.count !== undefined) {
      request.count = updates.count;
    }
    if (updates.bg !== undefined) {
      request.bg = updates.bg;
    }
    if (updates.border !== undefined) {
      request.border = updates.border;
    }
    return request;
  }

  private mapContactViewToUser(contact: ContactView, preferences?: ContactPreferencesView): User {
    this.contactByUserId.set(contact.targetUserId, contact);
    const user = this.createUserFromId(contact.targetUserId, preferences);
    this.userCache.set(user.id, user);
    return user;
  }

  private mapSocialUserSearchResultToUser(result: SocialUserSearchResult): User {
    const name = result.displayName || result.userId;
    const user: User = {
      id: result.userId,
      name,
      avatar: result.avatarUrl ?? createAvatar(result.userId),
      status: result.relationshipState === 'active' ? 'online' : 'offline',
      email: result.email,
      phone: result.phone,
      departmentId: pickString(
        toRecord(result).departmentId,
        toRecord(result).department_id,
        toRecord(result).orgUnitId,
        toRecord(result).org_unit_id,
      ),
      py: createSearchKey(name),
    };
    this.userCache.set(user.id, user);
    return user;
  }

  private createUserFromId(userId: string, preferences = this.preferenceByUserId.get(userId)): User {
    const cached = this.userCache.get(userId);
    const name = preferences?.remark || cached?.name || userId;
    return {
      id: userId,
      name,
      avatar: cached?.avatar ?? createAvatar(userId),
      status: cached?.status ?? 'offline',
      ...(cached?.departmentId ? { departmentId: cached.departmentId } : {}),
      py: createSearchKey(name),
    };
  }

  private async loadFriendRequestPeerProfiles(requests: ImFriendRequest[]): Promise<void> {
    const currentUserId = this.getCurrentUser().id;
    const peerUserIds = [...new Set(requests.map((request) => (
      request.requesterUserId === currentUserId
        ? request.targetUserId
        : request.requesterUserId
    )))];

    await Promise.all(peerUserIds.map(async (peerUserId) => {
      if (this.userCache.has(peerUserId)) {
        return;
      }
      try {
        const [profile] = await this.searchContacts(peerUserId);
        if (profile?.id === peerUserId) {
          this.userCache.set(peerUserId, profile);
        }
      } catch {
        // Keep the friend-request list usable when profile lookup is temporarily unavailable.
      }
    }));
  }

  private mapFriendRequest(request: ImFriendRequest): FriendRequest {
    const uiId = this.getOrCreateRequestUiId(request.requestId);
    const currentUserId = this.getCurrentUser().id;
    const peerUserId = request.requesterUserId === currentUserId
      ? request.targetUserId
      : request.requesterUserId;
    const peerUser = this.userCache.get(peerUserId);
    const name = this.preferenceByUserId.get(peerUserId)?.remark || peerUser?.name || peerUserId;
    return {
      avatar: peerUser?.avatar,
      id: uiId,
      name,
      msg: request.requestMessage ?? '',
      status: normalizeRequestStatus(request.status),
    };
  }

  private getOrCreateRequestUiId(requestId: string): number {
    const existing = this.requestUiIdByBackendId.get(requestId);
    if (existing) {
      return existing;
    }

    let uiId = toRequestUiId(requestId);
    while (this.requestIdByUiId.has(uiId) && this.requestIdByUiId.get(uiId) !== requestId) {
      uiId += 1;
    }
    this.requestIdByUiId.set(uiId, requestId);
    this.requestUiIdByBackendId.set(requestId, uiId);
    return uiId;
  }

  private resolveFriendshipPeerId(friendship: {
    initiatorUserId: string;
    userHighId: string;
    userLowId: string;
  }): string | undefined {
    const currentUserId = this.getCurrentUser().id;
    if (friendship.userLowId === currentUserId) {
      return friendship.userHighId;
    }
    if (friendship.userHighId === currentUserId) {
      return friendship.userLowId;
    }
    return friendship.initiatorUserId === currentUserId
      ? undefined
      : friendship.initiatorUserId;
  }

  private resolveFriendshipPeerIdFromPayload(payload: Record<string, unknown>): string | undefined {
    const currentUserId = this.getCurrentUser().id;
    const userLowId = pickDeviceSyncString(payload.userLowId, payload.lowUserId);
    const userHighId = pickDeviceSyncString(payload.userHighId, payload.highUserId);
    if (userLowId === currentUserId) {
      return userHighId;
    }
    if (userHighId === currentUserId) {
      return userLowId;
    }
    return pickDeviceSyncString(payload.targetUserId, payload.peerUserId);
  }

  private buildContactViewFromFriendshipPayload(
    tenantId: string,
    payload: Record<string, unknown>,
    targetUserId: string,
  ): ContactView {
    const currentUserId = this.getCurrentUser().id;
    const establishedAt = pickDeviceSyncString(
      payload.activatedAt,
      payload.establishedAt,
      payload.createdAt,
    ) ?? new Date().toISOString();
    return {
      contactType: 'friendship',
      conversationId: pickDeviceSyncString(payload.conversationId),
      directChatId: pickDeviceSyncString(payload.directChatId),
      establishedAt,
      friendshipId: pickDeviceSyncString(payload.friendshipId) ?? `friendship-${targetUserId}`,
      lastInteractionAt: pickDeviceSyncString(payload.lastInteractionAt, payload.boundAt) ?? establishedAt,
      ownerUserId: currentUserId,
      relationshipState: 'active',
      targetUserId,
      tenantId,
    };
  }
}

export function createSdkworkContactService(
  getClient?: () => ImSdkClient,
  getAppClient?: () => unknown,
  getOrganizationDirectoryService?: () => OrganizationDirectoryService,
): ContactService {
  return new SdkworkContactService(getClient, getAppClient, getOrganizationDirectoryService);
}

export const contactService = createSdkworkContactService();
