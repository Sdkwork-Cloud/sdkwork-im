import type {
  ContentPart,
  DriveReference,
  InboxResponse,
  ConversationProfileView,
  ImDecodedMessage,
  ImLiveConnection,
  ImMessageContext,
  ImSdkClient,
  MediaKind,
  MediaResource,
  ImSubscription,
  MessageInteractionSummaryView,
  MessageReplyReference,
  TimelineResponse,
  UpdateConversationProfileRequest,
} from '@sdkwork/im-sdk';
import type {
  DriveUploaderBlobLike,
  DriveUploaderClient,
  DriveUploaderProfile,
  DriveUploaderRequest,
  DriveUploaderUploadResult,
} from '@sdkwork/drive-app-sdk';
import { getDriveAppSdkClientWithSession } from '@sdkwork/clawchat-pc-core/sdk/driveAppSdkClient';
import {
  getImSdkClientWithSession,
} from '@sdkwork/clawchat-pc-core/sdk/imSdkClient';
import {
  SDKWORK_CHAT_SESSION_CHANGED_EVENT,
  readAppSdkSessionTokens,
  resolveAppSdkOrganizationId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from '@sdkwork/clawchat-pc-core/sdk/session';
import type { Chat, Message } from '@sdkwork/clawchat-pc-types';
import { resolveSdkworkChatPcClientId } from './ClientIdentityService';
import { contactService } from './ContactService';

type ConversationInboxEntry = InboxResponse['items'][number];
type TimelineViewEntry = TimelineResponse['items'][number];
type MessageHandler = (message: Message) => void;
type SendableMediaMessageType = Extract<Message['type'], 'file' | 'image' | 'video' | 'voice'>;
type SendableStructuredMessageType = Extract<Message['type'], 'applet' | 'card' | 'link' | 'music' | 'system' | 'video_call'>;

type ChatMessageExtraInfo = Partial<Message> & {
  file?: DriveUploaderBlobLike;
  mimeType?: string;
};

interface ChatMediaUploadResult {
  content: string;
  drive: DriveReference;
  resource: MediaResource;
}

interface ChatServiceDependencies {
  getClient?: () => ImSdkClient;
  getDriveUploader?: () => Pick<
    DriveUploaderClient,
    'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
  >;
  getSession?: () => SdkworkChatSession | null;
}

export interface ChatOfflineSyncResult {
  appliedMessages: number;
  refreshedChats: number;
}

interface ConversationLiveSubscription {
  closeLifecycleStream?: ImSubscription;
  closeMessageStream?: ImSubscription;
  closeErrorStream?: ImSubscription;
  connection?: ImLiveConnection;
  handlers: Set<MessageHandler>;
  isClosed: boolean;
  reconnectTimer?: ReturnType<typeof setTimeout>;
  started: Promise<void>;
}

export interface ChatService {
  getChats(): Promise<Chat[]>;
  getMessages(chatId: string): Promise<Message[]>;
  subscribeMessages(chatId: string, handler: MessageHandler): () => void;
  sendMessage(
    chatId: string,
    content: string,
    type?: Message['type'],
    replyTo?: Message['replyTo'],
    extraInfo?: ChatMessageExtraInfo
  ): Promise<Message>;
  forwardMessages(targetChatIds: string[], messages: Message[]): Promise<void>;
  markAsRead(chatId: string): Promise<void>;
  markAsUnread(chatId: string): Promise<void>;
  deleteMessage(chatId: string, messageId: string): Promise<void>;
  deleteChat(chatId: string): Promise<void>;
  pinChat(chatId: string, isPinned: boolean): Promise<void>;
  muteChat(chatId: string, isMuted: boolean): Promise<void>;
  addReaction(chatId: string, messageId: string, emoji: string): Promise<void>;
  removeReaction(chatId: string, messageId: string, emoji: string): Promise<void>;
  updateChat(chatId: string, updates: Partial<Chat>): Promise<Chat>;
  createChat(chat: Chat): Promise<void>;
  startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat>;
  startAgentChat(agent: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat>;
  startEnterpriseChat(enterprise: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat>;
  syncOfflineMessages(): Promise<ChatOfflineSyncResult>;
}

type ConversationViewState = Partial<Pick<Chat, 'activeCount' | 'avatar' | 'isMarkedUnread' | 'isMuted' | 'isPinned' | 'memberCount' | 'members' | 'name' | 'notice' | 'type'>> & {
  isHidden?: boolean;
};
const INBOX_PAGE_LIMIT = 100;
const MESSAGE_PAGE_LIMIT = 100;
const CHAT_APP_ID = 'chat';
const CHAT_DRIVE_SCENE = 'im';
const CHAT_DRIVE_SOURCE = 'chat_message';
const CHAT_DRIVE_APP_RESOURCE_TYPE = 'im_conversation';
const CHAT_MESSAGE_TYPES = new Set<Message['type']>([
  'applet',
  'card',
  'file',
  'image',
  'link',
  'music',
  'system',
  'text',
  'video',
  'video_call',
  'voice',
]);
const MEDIA_MESSAGE_TYPES = new Set<Message['type']>(['file', 'image', 'video', 'voice']);
const STRUCTURED_MESSAGE_SCHEMA_BY_TYPE: Record<SendableStructuredMessageType, string> = {
  applet: 'urn:sdkwork:craw-chat:message:applet',
  card: 'urn:sdkwork:craw-chat:message:card',
  link: 'urn:sdkwork:craw-chat:message:link',
  music: 'urn:sdkwork:craw-chat:message:music',
  system: 'urn:sdkwork:craw-chat:message:system',
  video_call: 'urn:sdkwork:craw-chat:message:video_call',
};

let driveUploaderClient: Pick<
  DriveUploaderClient,
  'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
> | null = null;

function parseTimestamp(value: string | undefined): number {
  if (!value) {
    return Date.now();
  }
  const timestamp = new Date(value).getTime();
  return Number.isFinite(timestamp) ? timestamp : Date.now();
}

function normalizeConversationType(value: string | undefined): Chat['type'] {
  return value?.toLowerCase() === 'group' ? 'group' : 'single';
}

function createConversationAvatar(conversationId: string): string {
  return `https://api.dicebear.com/7.x/shapes/svg?seed=${encodeURIComponent(conversationId)}`;
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
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

function isLocalPreviewUrl(value: string | undefined): boolean {
  return Boolean(value && /^(?:blob:|data:)/iu.test(value.trim()));
}

function pickDurableDeliveryUrl(value: string | undefined): string | undefined {
  return value && !isLocalPreviewUrl(value) ? value : undefined;
}

function resolveChatMessageType(value: unknown): Message['type'] | undefined {
  return typeof value === 'string' && CHAT_MESSAGE_TYPES.has(value as Message['type'])
    ? value as Message['type']
    : undefined;
}

function isMediaMessageType(value: Message['type']): value is SendableMediaMessageType {
  return MEDIA_MESSAGE_TYPES.has(value);
}

function isStructuredMessageType(value: Message['type']): value is SendableStructuredMessageType {
  return Object.prototype.hasOwnProperty.call(STRUCTURED_MESSAGE_SCHEMA_BY_TYPE, value);
}

function resolveDecodedMessageType(message: ImDecodedMessage): Message['type'] {
  const hintedType = resolveChatMessageType(message.renderHints?.sdkworkChatPcType);
  if (hintedType) {
    return hintedType;
  }

  switch (message.type) {
    case 'image':
    case 'video':
    case 'file':
    case 'link':
    case 'card':
    case 'music':
    case 'voice':
      return message.type;
    case 'audio':
      return 'voice';
    case 'contact':
      return 'card';
    case 'data':
    case 'signal':
    case 'stream_ref':
      return 'system';
    default:
      return 'text';
  }
}

function resolveResourceUrl(resource: ImDecodedMessage['attachments'][number]['resource'] | undefined): string | undefined {
  return pickString(resource?.publicUrl, resource?.url, resource?.uri);
}

function resolveRenditionUrl(value: unknown): string | undefined {
  const rendition = toRecord(value);
  return pickString(rendition.publicUrl, rendition.url, rendition.uri);
}

function resolveAttachmentUrl(message: ImDecodedMessage): string | undefined {
  return resolveResourceUrl(message.attachments[0]?.resource);
}

function firstTimelinePart(entry: TimelineViewEntry): Record<string, unknown> {
  const parts = Array.isArray(entry.body?.parts) ? entry.body.parts : [];
  return toRecord(parts[0]);
}

function resolvePartMessageType(part: Record<string, unknown>, renderHints: Record<string, unknown>): Message['type'] {
  const hintedType = resolveChatMessageType(renderHints.sdkworkChatPcType);
  if (hintedType) {
    return hintedType;
  }

  switch (part.kind) {
    case 'media': {
      const resource = toRecord(part.resource);
      const mediaKind = pickString(resource.kind, resource.mediaKind, resource.type);
      if (mediaKind === 'image' || mediaKind === 'video' || mediaKind === 'file') {
        return mediaKind;
      }
      if (mediaKind === 'audio' || mediaKind === 'voice') {
        return 'voice';
      }
      return 'file';
    }
    case 'data':
    case 'signal':
    case 'stream_ref':
      return 'system';
    default:
      return 'text';
  }
}

function resolveTimelineMessageType(entry: TimelineViewEntry): Message['type'] {
  const renderHints = toRecord(entry.body?.renderHints);
  return resolvePartMessageType(firstTimelinePart(entry), renderHints);
}

function resolveTimelineResource(entry: TimelineViewEntry): Record<string, unknown> {
  const part = firstTimelinePart(entry);
  return part.kind === 'media' ? toRecord(part.resource) : {};
}

function resolveTimelineResourceUrl(entry: TimelineViewEntry): string | undefined {
  const resource = resolveTimelineResource(entry);
  return pickString(resource.publicUrl, resource.url, resource.uri);
}

function resolveTimelineMessageContent(entry: TimelineViewEntry, type: Message['type']): string {
  const part = firstTimelinePart(entry);
  const resourceUrl = resolveTimelineResourceUrl(entry);
  switch (type) {
    case 'image':
    case 'video':
    case 'voice':
    case 'file':
      return pickString(resourceUrl, entry.body?.summary, entry.summary) ?? '';
    default:
      return pickString(part.text, entry.body?.summary, entry.summary) ?? '';
  }
}

function resolveDecodedMessageContent(message: ImDecodedMessage, type: Message['type']): string {
  const content = toRecord(message.content);
  const attachmentUrl = resolveAttachmentUrl(message);
  switch (type) {
    case 'image':
    case 'video':
    case 'voice':
    case 'file':
      return pickString(attachmentUrl, message.text, message.summary) ?? '';
    case 'link':
      return pickString(content.url, message.text, message.summary) ?? '';
    case 'music':
      return pickString(content.url, attachmentUrl, message.text, message.summary) ?? '';
    default:
      return pickString(
        message.text,
        message.summary,
        content.text,
        content.title,
        content.displayName,
        content.prompt,
      ) ?? '';
  }
}

function mapReplyReferenceToMessageReply(replyTo: MessageReplyReference | undefined): Message['replyTo'] | undefined {
  if (!replyTo) {
    return undefined;
  }

  return {
    id: replyTo.messageId,
    senderName: replyTo.senderDisplayName,
    content: replyTo.contentPreview,
  };
}

function buildReplyReference(replyTo: Message['replyTo'] | undefined): MessageReplyReference | undefined {
  if (!replyTo) {
    return undefined;
  }

  return {
    messageId: replyTo.id,
    senderDisplayName: replyTo.senderName,
    contentPreview: replyTo.content,
  };
}

function normalizeResourceNodeSegment(value: string): string {
  const normalized = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return normalized || 'resource';
}

function normalizeDirectChatIdSegment(value: string): string {
  const normalized = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return normalized || 'user';
}

const STANDARD_AGENT_ID_PATTERN = /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;

function requireStandardAgentChatId(value: string): string {
  const agentId = value.trim();
  if (!agentId) {
    throw new Error('Agent chat target id is required');
  }
  if (!STANDARD_AGENT_ID_PATTERN.test(agentId)) {
    throw new Error('Agent chat target id must use the standard agent. id format');
  }
  return agentId;
}

function buildDirectChatStableIds(currentUserId: string, targetUserId: string): {
  conversationId: string;
  directChatId: string;
} {
  const pair = [
    normalizeDirectChatIdSegment(currentUserId),
    normalizeDirectChatIdSegment(targetUserId),
  ].sort();
  const pairKey = pair.join('-');
  return {
    conversationId: `pc-direct-${pairKey}`,
    directChatId: `pc-dc-${pairKey}`,
  };
}

function buildAgentDialogStableId(currentUserId: string, agentId: string): string {
  return `pc-agent-${normalizeDirectChatIdSegment(currentUserId)}-${normalizeDirectChatIdSegment(agentId)}`;
}

function buildEnterpriseDialogStableIds(currentUserId: string, enterpriseId: string): {
  conversationId: string;
  directChatId: string;
} {
  const userSegment = normalizeDirectChatIdSegment(currentUserId);
  const enterpriseSegment = normalizeDirectChatIdSegment(enterpriseId);
  return {
    conversationId: `pc-enterprise-${userSegment}-${enterpriseSegment}`,
    directChatId: `pc-enterprise-dc-${userSegment}-${enterpriseSegment}`,
  };
}

function resolveMediaKind(type: Message['type']): MediaKind {
  switch (type) {
    case 'image':
      return 'image';
    case 'video':
      return 'video';
    case 'voice':
      return 'voice';
    case 'file':
      return 'file';
    default:
      return 'document';
  }
}

function createDefaultDriveUploaderClient(
  session: SdkworkChatSession | null = readAppSdkSessionTokens(),
): Pick<DriveUploaderClient, 'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'> {
  const client = getDriveAppSdkClientWithSession(session ?? readAppSdkSessionTokens());
  return client.uploader;
}

function getDefaultDriveUploader(): Pick<
  DriveUploaderClient,
  'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
> {
  if (!driveUploaderClient) {
    driveUploaderClient = createDefaultDriveUploaderClient();
  }
  return driveUploaderClient;
}

function resolveChatUploadTenantId(session: SdkworkChatSession | null | undefined): string {
  const tenantId = resolveAppSdkTenantId(session ?? null);
  if (!tenantId) {
    throw new Error('Chat media upload requires tenant_id in the authenticated session.');
  }
  return tenantId;
}

function resolveChatUploadUserId(session: SdkworkChatSession | null | undefined): string {
  const userId = resolveAppSdkUserId(session ?? null);
  if (!userId) {
    throw new Error('Chat media upload requires user_id in the authenticated session.');
  }
  return userId;
}

function parseFileSizeBytes(value: string | undefined): string | undefined {
  const normalized = value?.trim();
  if (!normalized) {
    return undefined;
  }

  const exactBytes = Number(normalized);
  if (Number.isFinite(exactBytes)) {
    return String(Math.max(0, Math.round(exactBytes)));
  }

  const match = normalized.match(/^(\d+(?:\.\d+)?)\s*(b|bytes|kb|mb|gb)$/i);
  if (!match) {
    return undefined;
  }

  const amount = Number(match[1]);
  if (!Number.isFinite(amount)) {
    return undefined;
  }

  const unit = match[2].toLowerCase();
  const multiplier = unit === 'gb'
    ? 1024 * 1024 * 1024
    : unit === 'mb'
      ? 1024 * 1024
      : unit === 'kb'
        ? 1024
        : 1;
  return String(Math.max(0, Math.round(amount * multiplier)));
}

function normalizeDriveUploadResult(result: DriveUploaderUploadResult): DriveReference {
  const spaceId = result.uploadItem.spaceId || result.uploadSession.spaceId;
  const nodeId = result.uploadItem.nodeId || result.uploadSession.nodeId;
  if (!spaceId || !nodeId) {
    throw new Error('Drive uploader result is missing spaceId or nodeId.');
  }
  return {
    driveUri: `drive://spaces/${spaceId}/nodes/${nodeId}`,
    spaceId,
    nodeId,
  };
}

function buildDriveMediaResource(
  drive: DriveReference,
  type: SendableMediaMessageType,
  extraInfo: ChatMessageExtraInfo | undefined,
  uploadResult?: DriveUploaderUploadResult,
): MediaResource {
  const mediaKind = resolveMediaKind(type);
  const uploadItem = uploadResult?.uploadItem;
  return {
    id: drive.nodeId,
    kind: mediaKind,
    source: 'drive',
    uri: drive.driveUri,
    fileName: uploadItem?.originalFileName ?? extraInfo?.fileName,
    mimeType: uploadItem?.contentType ?? extraInfo?.mimeType,
    sizeBytes: uploadItem?.contentLength ?? parseFileSizeBytes(extraInfo?.fileSize),
    durationSeconds: extraInfo?.duration,
  };
}

function buildMediaMessageParts(
  upload: ChatMediaUploadResult,
): ContentPart[] {
  return [{
    kind: 'media' as const,
    drive: upload.drive,
    resource: upload.resource,
    mediaRole: 'attachment',
  }];
}

function buildStructuredMessagePayload(
  content: string,
  type: SendableStructuredMessageType,
  extraInfo: ChatMessageExtraInfo | undefined,
): Record<string, unknown> {
  return {
    ...(extraInfo?.fileName ? { title: extraInfo.fileName } : {}),
    ...(type === 'video_call' ? { state: content } : { url: content }),
    ...(extraInfo?.desc ? { description: extraInfo.desc } : {}),
    ...(extraInfo?.appIcon ? { iconUrl: extraInfo.appIcon } : {}),
    ...(extraInfo?.coverUrl ? { coverUrl: extraInfo.coverUrl } : {}),
    ...(extraInfo?.duration ? { durationSeconds: extraInfo.duration } : {}),
  };
}

function buildStructuredMessageParts(
  content: string,
  type: SendableStructuredMessageType,
  extraInfo: ChatMessageExtraInfo | undefined,
): ContentPart[] {
  return [{
    kind: 'data' as const,
    schemaRef: STRUCTURED_MESSAGE_SCHEMA_BY_TYPE[type],
    encoding: 'application/json',
    payload: JSON.stringify(buildStructuredMessagePayload(content, type, extraInfo)),
  }];
}

function buildFallbackTextMessageParts(
  content: string,
): ContentPart[] {
  return [{
    kind: 'text' as const,
    text: content,
  }];
}

function buildMessageParts(
  content: string,
  type: Message['type'],
  extraInfo: ChatMessageExtraInfo | undefined,
  mediaUpload?: ChatMediaUploadResult,
): ContentPart[] | undefined {
  if (isMediaMessageType(type)) {
    if (!mediaUpload) {
      throw new Error('Chat media messages require Drive upload result before IM send.');
    }
    return buildMediaMessageParts(mediaUpload);
  }
  if (isStructuredMessageType(type)) {
    return buildStructuredMessageParts(content, type, extraInfo);
  }
  return buildFallbackTextMessageParts(content);
}

function buildMessageRenderHints(
  type: Message['type'],
  extraInfo: ChatMessageExtraInfo | undefined,
) {
  const coverUrl = type === 'link'
    ? pickDurableDeliveryUrl(extraInfo?.coverUrl)
    : undefined;
  return {
    sdkworkChatPcType: type,
    ...(coverUrl ? { coverUrl } : {}),
    ...(extraInfo?.fileName ? { fileName: extraInfo.fileName } : {}),
    ...(extraInfo?.fileSize ? { fileSize: extraInfo.fileSize } : {}),
    ...(type === 'link' && extraInfo?.desc ? { desc: extraInfo.desc } : {}),
    ...(extraInfo?.duration ? { duration: String(extraInfo.duration) } : {}),
  };
}

function resolveMediaUploadProfile(type: SendableMediaMessageType): DriveUploaderProfile | undefined {
  switch (type) {
    case 'file':
      return 'attachment';
    default:
      return undefined;
  }
}

function resolveMediaUploadContentType(
  type: SendableMediaMessageType,
  file: DriveUploaderBlobLike,
  extraInfo: ChatMessageExtraInfo | undefined,
): string | undefined {
  return extraInfo?.mimeType
    ?? file.type
    ?? (type === 'voice' ? 'audio/webm' : undefined);
}

function resolveMediaUploadFileName(
  type: SendableMediaMessageType,
  file: DriveUploaderBlobLike,
  extraInfo: ChatMessageExtraInfo | undefined,
): string {
  const fallback = type === 'voice'
    ? `voice-${Date.now().toString(36)}.webm`
    : `chat-${type}-${Date.now().toString(36)}`;
  return pickString(extraInfo?.fileName, file.name, fallback) ?? fallback;
}

async function uploadChatMediaFile({
  chatId,
  content,
  extraInfo,
  getDriveUploader,
  getSession,
  type,
}: {
  chatId: string;
  content: string;
  extraInfo: ChatMessageExtraInfo | undefined;
  getDriveUploader: () => Pick<
    DriveUploaderClient,
    'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
  >;
  getSession: () => SdkworkChatSession | null;
  type: SendableMediaMessageType;
}): Promise<ChatMediaUploadResult> {
  const file = extraInfo?.file;
  if (!file) {
    throw new Error('Chat media messages require a File or Blob before sending.');
  }

  const session = getSession();
  const tenantId = resolveChatUploadTenantId(session);
  const userId = resolveChatUploadUserId(session);
  const organizationId = resolveAppSdkOrganizationId(session ?? null);
  const originalFileName = resolveMediaUploadFileName(type, file, extraInfo);
  const uploadRequest: DriveUploaderRequest = {
    file,
    tenantId,
    ...(organizationId ? { organizationId } : {}),
    userId,
    appId: CHAT_APP_ID,
    appResourceType: CHAT_DRIVE_APP_RESOURCE_TYPE,
    appResourceId: chatId,
    scene: CHAT_DRIVE_SCENE,
    source: CHAT_DRIVE_SOURCE,
    ...(resolveMediaUploadProfile(type) ? { uploadProfileCode: resolveMediaUploadProfile(type) } : {}),
    originalFileName,
    ...(resolveMediaUploadContentType(type, file, extraInfo) ? { contentType: resolveMediaUploadContentType(type, file, extraInfo) } : {}),
    operatorId: userId,
  };

  const uploader = getDriveUploader();
  const uploadResult = type === 'image'
    ? await uploader.uploadImage(uploadRequest)
    : type === 'voice'
      ? await uploader.uploadAudio(uploadRequest)
      : type === 'video'
        ? await uploader.uploadVideo(uploadRequest)
        : await uploader.uploadAttachment(uploadRequest);
  const drive = normalizeDriveUploadResult(uploadResult);
  const resource = buildDriveMediaResource(drive, type, {
    ...extraInfo,
    fileName: originalFileName,
    mimeType: uploadRequest.contentType,
  }, uploadResult);
  return {
    content: pickString(content, drive.driveUri) ?? drive.driveUri,
    drive,
    resource,
  };
}

function mapLiveMessageToMessage(
  fallbackChatId: string,
  decodedMessage: ImDecodedMessage,
  context: ImMessageContext,
): Message {
  const payload = toRecord(context.payload);
  const rawEvent = toRecord(context.rawEvent);
  const content = toRecord(decodedMessage.content);
  const resource = decodedMessage.attachments[0]?.resource;
  const type = resolveDecodedMessageType(decodedMessage);
  const messageId = pickString(
    context.messageId,
    payload.messageId,
    rawEvent.eventId,
  ) ?? `${fallbackChatId}:${context.sequence}`;
  const conversationId = pickString(context.conversationId, payload.conversationId) ?? fallbackChatId;
  const renderHints = decodedMessage.renderHints ?? {};
  const duration = pickNumber(renderHints.duration, renderHints.durationSeconds, content.durationSeconds, resource?.durationSeconds);
  const coverUrl = pickString(
    renderHints.coverUrl,
    content.coverUrl,
    content.imageUrl,
    resolveRenditionUrl(resource?.poster),
    resolveRenditionUrl(resource?.thumbnails?.[0]),
  );
  const fileName = pickString(
    renderHints.fileName,
    content.title,
    content.displayName,
    resource?.fileName,
    resource?.title,
  );
  const replyTo = mapReplyReferenceToMessageReply(decodedMessage.replyTo);

  return {
    id: messageId,
    chatId: conversationId,
    senderId: context.sender?.principalId ?? 'system',
    content: resolveDecodedMessageContent(decodedMessage, type),
    type,
    timestamp: parseTimestamp(context.receivedAt),
    ...(coverUrl ? { coverUrl } : {}),
    ...(duration ? { duration } : {}),
    ...(fileName ? { fileName } : {}),
    ...(pickString(renderHints.fileSize, resource?.sizeBytes) ? { fileSize: pickString(renderHints.fileSize, resource?.sizeBytes) } : {}),
    ...(pickString(renderHints.appIcon, content.avatarUrl, content.imageUrl) ? { appIcon: pickString(renderHints.appIcon, content.avatarUrl, content.imageUrl) } : {}),
    ...(pickString(renderHints.desc, content.description, content.subtitle, content.artist) ? { desc: pickString(renderHints.desc, content.description, content.subtitle, content.artist) } : {}),
    ...(replyTo ? { replyTo } : {}),
  };
}

function buildConversationName(entry: ConversationInboxEntry): string {
  if (entry.agentHandoff) {
    return `Agent Handoff ${entry.conversationId}`;
  }
  return normalizeConversationType(entry.conversationType) === 'group'
    ? `Group ${entry.conversationId}`
    : `Chat ${entry.conversationId}`;
}

function buildLastMessage(entry: ConversationInboxEntry, timestamp: number): Message | undefined {
  if (!entry.lastMessageId && !entry.lastSummary) {
    return undefined;
  }

  return {
    id: entry.lastMessageId ?? `${entry.conversationId}:${entry.lastMessageSeq}`,
    chatId: entry.conversationId,
    senderId: entry.lastSenderId ?? 'system',
    content: entry.lastSummary ?? '',
    type: 'text',
    timestamp,
  };
}

function mapInboxEntryToChat(entry: ConversationInboxEntry, viewState: ConversationViewState | undefined): Chat {
  const updatedAt = parseTimestamp(entry.lastActivityAt);
  return {
    id: entry.conversationId,
    name: viewState?.name ?? buildConversationName(entry),
    avatar: viewState?.avatar ?? createConversationAvatar(entry.conversationId),
    type: viewState?.type ?? normalizeConversationType(entry.conversationType),
    unreadCount: entry.unreadCount,
    updatedAt,
    activeCount: viewState?.activeCount,
    memberCount: viewState?.memberCount,
    members: viewState?.members,
    isMarkedUnread: viewState?.isMarkedUnread,
    isMuted: viewState?.isMuted,
    isPinned: viewState?.isPinned,
    notice: viewState?.notice,
    lastMessage: buildLastMessage(entry, updatedAt),
  };
}

function applyConversationProfile(
  viewState: ConversationViewState | undefined,
  profile: ConversationProfileView,
): ConversationViewState {
  return {
    ...viewState,
    ...(pickString(profile.displayName) ? { name: pickString(profile.displayName) } : {}),
    ...(pickString(profile.avatarUrl) ? { avatar: pickString(profile.avatarUrl) } : {}),
    notice: profile.notice,
  };
}

function buildConversationProfileUpdate(updates: Partial<Chat>): UpdateConversationProfileRequest {
  return {
    ...(updates.avatar !== undefined ? { avatarUrl: updates.avatar } : {}),
    ...(updates.name !== undefined ? { displayName: updates.name } : {}),
    ...(updates.notice !== undefined ? { notice: updates.notice } : {}),
  };
}

function hasProfileUpdate(update: UpdateConversationProfileRequest): boolean {
  return update.avatarUrl !== undefined
    || update.displayName !== undefined
    || update.notice !== undefined;
}

function buildLocalConversationViewUpdate(updates: Partial<Chat>): ConversationViewState {
  return {
    ...(updates.activeCount !== undefined ? { activeCount: updates.activeCount } : {}),
    ...(updates.isMuted !== undefined ? { isMuted: updates.isMuted } : {}),
    ...(updates.isMarkedUnread !== undefined ? { isMarkedUnread: updates.isMarkedUnread } : {}),
    ...(updates.isPinned !== undefined ? { isPinned: updates.isPinned } : {}),
    ...(updates.memberCount !== undefined ? { memberCount: updates.memberCount } : {}),
    ...(updates.members !== undefined ? { members: updates.members } : {}),
    ...(updates.type !== undefined ? { type: updates.type } : {}),
  };
}

function mapTimelineEntryToMessage(
  entry: TimelineViewEntry,
  index: number,
  total: number,
  cachedMessage?: Message,
): Message {
  if (cachedMessage) {
    return cachedMessage;
  }

  const type = resolveTimelineMessageType(entry);
  const renderHints = toRecord(entry.body?.renderHints);
  const resource = resolveTimelineResource(entry);
  const coverUrl = pickString(
    renderHints.coverUrl,
    resolveRenditionUrl(resource.poster),
    resolveRenditionUrl(Array.isArray(resource.thumbnails) ? resource.thumbnails[0] : undefined),
  );
  const fileName = pickString(
    renderHints.fileName,
    resource.fileName,
    resource.title,
  );
  const duration = pickNumber(renderHints.duration, renderHints.durationSeconds, resource.durationSeconds);
  const replyTo = mapReplyReferenceToMessageReply(entry.body?.replyTo);

  return {
    id: entry.messageId,
    chatId: entry.conversationId,
    senderId: pickString(entry.sender?.id) ?? 'system',
    content: resolveTimelineMessageContent(entry, type),
    type,
    timestamp: parseTimestamp(entry.committedAt ?? entry.occurredAt) || Date.now() - Math.max(total - index, 0) * 1000,
    ...(coverUrl ? { coverUrl } : {}),
    ...(duration ? { duration } : {}),
    ...(fileName ? { fileName } : {}),
    ...(pickString(renderHints.fileSize, resource.sizeBytes) ? { fileSize: pickString(renderHints.fileSize, resource.sizeBytes) } : {}),
    ...(replyTo ? { replyTo } : {}),
  };
}

function mapInteractionSummaryToReactions(
  summary: MessageInteractionSummaryView | undefined,
  cachedReactions: Message['reactions'],
): Message['reactions'] | undefined {
  if (!summary || summary.reactionCounts.length === 0) {
    return undefined;
  }

  return summary.reactionCounts
    .filter((reaction) => reaction.count > 0)
    .map((reaction) => {
      const cached = cachedReactions?.find((item) => item.emoji === reaction.reactionKey);
      return {
        emoji: reaction.reactionKey,
        count: reaction.count,
        hasReacted: cached?.hasReacted ?? false,
      };
    });
}

function withMessageInteractionSummary(
  message: Message,
  summary: MessageInteractionSummaryView | undefined,
): Message {
  const reactions = mapInteractionSummaryToReactions(summary, message.reactions);
  if (reactions && reactions.length > 0) {
    return { ...message, reactions };
  }

  const { reactions: _reactions, ...messageWithoutReactions } = message;
  return messageWithoutReactions;
}

function sortChats(left: Chat, right: Chat): number {
  if (left.isPinned !== right.isPinned) {
    return left.isPinned ? -1 : 1;
  }
  return right.updatedAt - left.updatedAt;
}

function mergeMessageLists(remoteMessages: Message[], localMessages: Message[]): Message[] {
  const byId = new Map<string, Message>();
  for (const message of remoteMessages) {
    byId.set(message.id, message);
  }
  for (const message of localMessages) {
    const existing = byId.get(message.id);
    if (!existing) {
      byId.set(message.id, message);
      continue;
    }
    byId.set(message.id, {
      ...existing,
      ...message,
      ...(message.reactions ? { reactions: message.reactions } : existing.reactions ? { reactions: existing.reactions } : {}),
    });
  }
  return Array.from(byId.values()).sort((left, right) => left.timestamp - right.timestamp);
}

class SdkworkChatService implements ChatService {
  private conversationViewState = new Map<string, ConversationViewState>();
  private liveSubscriptions = new Map<string, ConversationLiveSubscription>();
  private localMessages = new Map<string, Message[]>();
  private latestReadSeq = new Map<string, number>();
  private readonly getClient: () => ImSdkClient;
  private readonly getDriveUploader: () => Pick<
    DriveUploaderClient,
    'uploadAudio' | 'uploadAttachment' | 'uploadImage' | 'uploadVideo'
  >;
  private readonly getSession: () => SdkworkChatSession | null;

  private readonly handleAuthSessionChanged = (): void => {
    driveUploaderClient = null;
    this.closeAllLiveSubscriptions('auth session changed');
  };

  constructor(dependencies: ChatServiceDependencies | (() => ImSdkClient) = {}) {
    if (typeof dependencies === 'function') {
      this.getClient = dependencies;
      this.getDriveUploader = getDefaultDriveUploader;
      this.getSession = readAppSdkSessionTokens;
    } else {
      this.getClient = dependencies.getClient ?? getImSdkClientWithSession;
      this.getDriveUploader = dependencies.getDriveUploader ?? getDefaultDriveUploader;
      this.getSession = dependencies.getSession ?? readAppSdkSessionTokens;
    }
    if (typeof window !== 'undefined') {
      window.addEventListener(SDKWORK_CHAT_SESSION_CHANGED_EVENT, this.handleAuthSessionChanged);
    }
  }

  private client(): ImSdkClient {
    return this.getClient();
  }

  private async listAllInboxEntries(): Promise<ConversationInboxEntry[]> {
    const items: ConversationInboxEntry[] = [];
    let cursor: string | undefined;

    do {
      const response = await this.client().chat.inbox.retrieve({
        limit: INBOX_PAGE_LIMIT,
        ...(cursor ? { cursor } : {}),
      });
      items.push(...response.items);
      cursor = response.hasMore ? response.nextCursor : undefined;
    } while (cursor);

    return items;
  }

  private async listAllTimelineEntries(chatId: string): Promise<TimelineViewEntry[]> {
    const items: TimelineViewEntry[] = [];
    let afterSeq = 0;

    while (true) {
      const response = await this.client().conversations.listMessages(chatId, {
        afterSeq,
        limit: MESSAGE_PAGE_LIMIT,
      });
      items.push(...response.items);

      if (
        !response.hasMore
        || typeof response.nextAfterSeq !== 'number'
        || response.nextAfterSeq <= afterSeq
      ) {
        break;
      }

      afterSeq = response.nextAfterSeq;
    }

    return items;
  }

  async getChats(): Promise<Chat[]> {
    const inboxEntries = await this.listAllInboxEntries();
    const chatResults = await Promise.all(inboxEntries
      .map(async (entry): Promise<Chat | undefined> => {
        this.latestReadSeq.set(entry.conversationId, Math.max(
          this.latestReadSeq.get(entry.conversationId) ?? 0,
          entry.lastMessageSeq,
        ));
        let viewState = this.conversationViewState.get(entry.conversationId);
        try {
          const preferences = await this.client().conversations.getPreferences(entry.conversationId);
          viewState = {
            ...viewState,
            isMuted: preferences.isMuted,
            isMarkedUnread: preferences.isMarkedUnread,
            isPinned: preferences.isPinned,
            isHidden: preferences.isHidden,
          };
          this.conversationViewState.set(entry.conversationId, viewState);
        } catch {
          // Keep the chat list usable if a preference sync request fails.
        }
        if (viewState?.isHidden) {
          return undefined;
        }
        try {
          const profile = await this.client().conversations.getProfile(entry.conversationId);
          viewState = applyConversationProfile(viewState, profile);
          this.conversationViewState.set(entry.conversationId, viewState);
        } catch {
          // Keep local naming/avatar fallbacks usable if profile sync is temporarily unavailable.
        }
        return mapInboxEntryToChat(entry, viewState);
      }));
    const chats = chatResults.filter((chat): chat is Chat => Boolean(chat));

    for (const [chatId, localMessages] of this.localMessages.entries()) {
      if (this.conversationViewState.get(chatId)?.isHidden || chats.some((chat) => chat.id === chatId)) {
        continue;
      }
      const state = this.conversationViewState.get(chatId);
      const lastMessage = localMessages.at(-1);
      chats.push({
        id: chatId,
        name: state?.name ?? `Chat ${chatId}`,
        avatar: state?.avatar ?? createConversationAvatar(chatId),
        type: state?.type ?? 'single',
        unreadCount: 0,
        updatedAt: lastMessage?.timestamp ?? Date.now(),
        lastMessage,
        activeCount: state?.activeCount,
        memberCount: state?.memberCount,
        members: state?.members,
        isMarkedUnread: state?.isMarkedUnread,
        isMuted: state?.isMuted,
        isPinned: state?.isPinned,
        notice: state?.notice,
      });
    }

    return chats.sort(sortChats);
  }

  async getMessages(chatId: string): Promise<Message[]> {
    const timelineEntries = await this.listAllTimelineEntries(chatId);
    const cachedMessages = new Map(
      (this.localMessages.get(chatId) ?? []).map((message) => [message.id, message]),
    );
    const remoteMessages = await Promise.all(timelineEntries.map(async (entry, index) => {
      this.latestReadSeq.set(chatId, Math.max(
        this.latestReadSeq.get(chatId) ?? 0,
        entry.messageSeq,
      ));
      const message = mapTimelineEntryToMessage(
        entry,
        index,
        timelineEntries.length,
        cachedMessages.get(entry.messageId),
      );
      try {
        const summary = await this.client().conversations.getMessageInteractionSummary(
          chatId,
          entry.messageId,
        );
        return withMessageInteractionSummary(message, summary);
      } catch {
        return message;
      }
    }));

    const mergedMessages = mergeMessageLists(remoteMessages, this.localMessages.get(chatId) ?? []);
    this.localMessages.set(chatId, mergedMessages);
    return mergedMessages;
  }

  subscribeMessages(chatId: string, handler: MessageHandler): () => void {
    const subscription = this.getOrCreateLiveSubscription(chatId);
    subscription.handlers.add(handler);

    return () => {
      subscription.handlers.delete(handler);
      if (subscription.handlers.size === 0) {
        this.closeLiveSubscription(chatId, subscription);
      }
    };
  }

  async sendMessage(
    chatId: string,
    content: string,
    type: Message['type'] = 'text',
    replyTo?: Message['replyTo'],
    extraInfo?: ChatMessageExtraInfo,
  ): Promise<Message> {
    const client = this.client();
    const currentUser = contactService.getCurrentUser();
    const clientMsgId = `pc-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
    const replyReference = buildReplyReference(replyTo);
    const mediaUpload = isMediaMessageType(type)
      ? await uploadChatMediaFile({
          chatId,
          content,
          extraInfo,
          getDriveUploader: this.getDriveUploader,
          getSession: this.getSession,
          type,
        })
      : undefined;
    const remoteSummary = mediaUpload?.resource.fileName ?? (content || extraInfo?.fileName || type);
    const parts = type === 'text'
      ? undefined
      : buildMessageParts(mediaUpload?.content ?? content, type, extraInfo, mediaUpload);
    const postResult = type === 'text'
      ? await client.conversations.postText(chatId, content, {
          clientMsgId,
          summary: content,
          ...(replyReference ? { replyTo: replyReference } : {}),
        })
      : await client.conversations.postMessage(chatId, {
          clientMsgId,
          summary: remoteSummary,
          ...(replyReference ? { replyTo: replyReference } : {}),
          ...(parts ? { parts } : {}),
          renderHints: buildMessageRenderHints(type, extraInfo),
        });

    const {
      file: _file,
      mimeType: _mimeType,
      ...localExtraInfo
    } = extraInfo ?? {};
    const message: Message = {
      id: postResult.messageId,
      chatId,
      senderId: extraInfo?.senderId ?? currentUser.id,
      content,
      type,
      timestamp: Date.now(),
      replyTo,
      ...localExtraInfo,
    };
    this.upsertLocalMessage(chatId, message, true);
    this.latestReadSeq.set(chatId, Math.max(this.latestReadSeq.get(chatId) ?? 0, postResult.messageSeq));
    return message;
  }

  async forwardMessages(targetChatIds: string[], messages: Message[]): Promise<void> {
    for (const targetChatId of targetChatIds) {
      for (const message of messages) {
        if (isMediaMessageType(message.type)) {
          throw new Error('Forwarding media messages requires a reusable Drive reference before sending.');
        }
        await this.sendMessage(targetChatId, message.content, message.type, undefined, {
          fileName: message.fileName,
          fileSize: message.fileSize,
          coverUrl: message.coverUrl,
          duration: message.duration,
          appIcon: message.appIcon,
          desc: message.desc,
        });
      }
    }
  }

  async markAsRead(chatId: string): Promise<void> {
    const readSeq = this.latestReadSeq.get(chatId) ?? 0;
    if (readSeq > 0) {
      await this.client().conversations.updateReadCursor(chatId, { readSeq });
    }
    await this.client().conversations.updatePreferences(chatId, { isMarkedUnread: false });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: false,
    });
  }

  async markAsUnread(chatId: string): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isMarkedUnread: true });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isMarkedUnread: true,
    });
  }

  async deleteMessage(chatId: string, messageId: string): Promise<void> {
    await this.client().messages.deleteForMe(messageId);
    const messages = this.localMessages.get(chatId) ?? [];
    this.localMessages.set(chatId, messages.filter((message) => message.id !== messageId));
  }

  async deleteChat(chatId: string): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isHidden: true });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isHidden: true,
    });
    this.localMessages.delete(chatId);
  }

  async pinChat(chatId: string, isPinned: boolean): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isPinned });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isPinned,
    });
  }

  async muteChat(chatId: string, isMuted: boolean): Promise<void> {
    await this.client().conversations.updatePreferences(chatId, { isMuted });
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      isMuted,
    });
  }

  async addReaction(chatId: string, messageId: string, emoji: string): Promise<void> {
    await this.client().addReaction(messageId, emoji);
    this.updateLocalMessage(chatId, messageId, (message) => {
      const reactions = [...(message.reactions ?? [])];
      const existing = reactions.find((reaction) => reaction.emoji === emoji);
      if (existing) {
        existing.count += existing.hasReacted ? 0 : 1;
        existing.hasReacted = true;
      } else {
        reactions.push({ emoji, count: 1, hasReacted: true });
      }
      return { ...message, reactions };
    });
  }

  async removeReaction(chatId: string, messageId: string, emoji: string): Promise<void> {
    await this.client().removeReaction(messageId, emoji);
    this.updateLocalMessage(chatId, messageId, (message) => {
      const reactions = (message.reactions ?? [])
        .map((reaction) => {
          if (reaction.emoji !== emoji || !reaction.hasReacted) {
            return reaction;
          }
          return {
            ...reaction,
            count: reaction.count - 1,
            hasReacted: false,
          };
        })
        .filter((reaction) => reaction.count > 0);
      return { ...message, reactions };
    });
  }

  async updateChat(chatId: string, updates: Partial<Chat>): Promise<Chat> {
    const profileUpdate = buildConversationProfileUpdate(updates);
    if (hasProfileUpdate(profileUpdate)) {
      const profile = await this.client().conversations.updateProfile(chatId, profileUpdate);
      this.conversationViewState.set(chatId, applyConversationProfile(
        this.conversationViewState.get(chatId),
        profile,
      ));
    }
    const localViewUpdate = buildLocalConversationViewUpdate(updates);
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      ...localViewUpdate,
    });
    const updated = (await this.getChats()).find((chat) => chat.id === chatId);
    if (!updated) {
      throw new Error('Chat not found');
    }
    this.conversationViewState.set(chatId, {
      ...this.conversationViewState.get(chatId),
      ...localViewUpdate,
    });
    return {
      ...updated,
      ...localViewUpdate,
    };
  }

  async createChat(chat: Chat): Promise<void> {
    await this.client().conversations.create({
      conversationId: chat.id,
      conversationType: chat.type,
    });
    const profileUpdate = buildConversationProfileUpdate(chat);
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(chat.id, profileUpdate);
    }
    await this.client().conversations.updatePreferences(chat.id, { isHidden: false });
    this.conversationViewState.set(chat.id, {
      avatar: chat.avatar,
      isHidden: false,
      memberCount: chat.memberCount,
      name: chat.name,
      notice: chat.notice,
      type: chat.type,
    });
    if (chat.lastMessage) {
      this.localMessages.set(chat.id, [chat.lastMessage]);
    }
  }

  async startDirectChat(user: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat> {
    const targetUserId = user.id.trim();
    if (!targetUserId) {
      throw new Error('Direct chat target user id is required');
    }
    const currentUser = contactService.getCurrentUser();
    const currentUserId = currentUser.id.trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }
    const { conversationId, directChatId } = buildDirectChatStableIds(currentUserId, targetUserId);
    const result = await this.client().conversations.bindDirectChat({
      conversationId,
      directChatId,
      leftActorId: currentUserId,
      leftActorKind: 'user',
      rightActorId: targetUserId,
      rightActorKind: 'user',
    });
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: user.avatar,
      name: user.name,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(boundConversationId, profileUpdate);
    }
    await this.client().conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.conversationViewState.set(boundConversationId, {
      ...this.conversationViewState.get(boundConversationId),
      avatar: user.avatar,
      isHidden: false,
      name: user.name,
      type: 'single',
    });
    return {
      id: boundConversationId,
      name: user.name,
      avatar: user.avatar,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
    };
  }

  async startAgentChat(agent: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat> {
    const agentId = requireStandardAgentChatId(agent.id);
    const currentUser = contactService.getCurrentUser();
    const currentUserId = currentUser.id.trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }

    const conversationId = buildAgentDialogStableId(currentUserId, agentId);
    const result = await this.client().conversations.createAgentDialog({
      agentId,
      conversationId,
    });
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: agent.avatar,
      name: agent.name,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(boundConversationId, profileUpdate);
    }
    await this.client().conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.conversationViewState.set(boundConversationId, {
      ...this.conversationViewState.get(boundConversationId),
      avatar: agent.avatar,
      isHidden: false,
      name: agent.name,
      type: 'single',
    });
    return {
      id: boundConversationId,
      name: agent.name,
      avatar: agent.avatar,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
    };
  }

  async startEnterpriseChat(enterprise: Pick<Chat, 'avatar' | 'name'> & { id: string }): Promise<Chat> {
    const enterpriseId = enterprise.id.trim();
    if (!enterpriseId) {
      throw new Error('Enterprise chat target id is required');
    }
    const currentUser = contactService.getCurrentUser();
    const currentUserId = currentUser.id.trim();
    if (!currentUserId) {
      throw new Error('Current user id is required');
    }

    const { conversationId, directChatId } = buildEnterpriseDialogStableIds(currentUserId, enterpriseId);
    const displayName = enterprise.name.endsWith(' (Official)')
      ? enterprise.name
      : `${enterprise.name} (Official)`;
    const result = await this.client().conversations.bindDirectChat({
      conversationId,
      directChatId,
      leftActorId: currentUserId,
      leftActorKind: 'user',
      rightActorId: enterpriseId,
      rightActorKind: 'enterprise',
    });
    const boundConversationId = result.conversationId;
    const profileUpdate = buildConversationProfileUpdate({
      avatar: enterprise.avatar,
      name: displayName,
    });
    if (hasProfileUpdate(profileUpdate)) {
      await this.client().conversations.updateProfile(boundConversationId, profileUpdate);
    }
    await this.client().conversations.updatePreferences(boundConversationId, { isHidden: false });
    this.conversationViewState.set(boundConversationId, {
      ...this.conversationViewState.get(boundConversationId),
      avatar: enterprise.avatar,
      isHidden: false,
      name: displayName,
      type: 'single',
    });
    return {
      id: boundConversationId,
      name: displayName,
      avatar: enterprise.avatar,
      type: 'single',
      unreadCount: 0,
      updatedAt: Date.now(),
    };
  }

  async syncOfflineMessages(): Promise<ChatOfflineSyncResult> {
    const chats = await this.getChats();
    let appliedMessages = 0;
    let refreshedChats = 0;

    for (const chat of chats) {
      const messages = await this.getMessages(chat.id);
      appliedMessages += messages.length;
      refreshedChats += 1;
    }

    return {
      appliedMessages,
      refreshedChats,
    };
  }

  private getOrCreateLiveSubscription(chatId: string): ConversationLiveSubscription {
    const existing = this.liveSubscriptions.get(chatId);
    if (existing) {
      return existing;
    }

    const subscription: ConversationLiveSubscription = {
      handlers: new Set<MessageHandler>(),
      isClosed: false,
      started: Promise.resolve(),
    };
    subscription.started = this.startLiveSubscription(chatId, subscription);
    this.liveSubscriptions.set(chatId, subscription);
    return subscription;
  }

  private async startLiveSubscription(
    chatId: string,
    subscription: ConversationLiveSubscription,
  ): Promise<void> {
    try {
      const connection = await this.client().connect({
        deviceId: resolveSdkworkChatPcClientId(),
        subscriptions: {
          conversations: [chatId],
        },
      });
      if (subscription.isClosed) {
        connection.disconnect(1000, 'conversation subscription closed');
        return;
      }

      subscription.connection = connection;
      subscription.closeMessageStream = connection.messages.onConversation(
        chatId,
        (message, context) => {
          this.handleLiveMessage(chatId, message, context);
        },
      );
      subscription.closeLifecycleStream = connection.lifecycle.onStateChange((state) => {
        if ((state.status === 'closed' || state.status === 'error') && !subscription.isClosed) {
          this.scheduleLiveResubscribe(chatId, subscription);
        }
      });
      subscription.closeErrorStream = connection.lifecycle.onError(() => undefined);
    } catch {
      if (this.liveSubscriptions.get(chatId) === subscription && subscription.handlers.size > 0 && !subscription.isClosed) {
        this.scheduleLiveResubscribe(chatId, subscription);
      } else if (this.liveSubscriptions.get(chatId) === subscription) {
        this.liveSubscriptions.delete(chatId);
      }
    }
  }

  private closeLiveSubscription(
    chatId: string,
    subscription: ConversationLiveSubscription,
    reason = 'conversation subscription closed',
  ): void {
    subscription.isClosed = true;
    if (subscription.reconnectTimer) {
      clearTimeout(subscription.reconnectTimer);
      subscription.reconnectTimer = undefined;
    }
    subscription.closeMessageStream?.();
    subscription.closeLifecycleStream?.();
    subscription.closeErrorStream?.();
    subscription.connection?.disconnect(1000, reason);
    if (this.liveSubscriptions.get(chatId) === subscription) {
      this.liveSubscriptions.delete(chatId);
    }
  }

  private closeAllLiveSubscriptions(reason: string): void {
    for (const [chatId, subscription] of Array.from(this.liveSubscriptions.entries())) {
      this.closeLiveSubscription(chatId, subscription, reason);
    }
  }

  private restartLiveSubscription(chatId: string, subscription: ConversationLiveSubscription): void {
    if (
      subscription.isClosed
      || subscription.handlers.size === 0
      || this.liveSubscriptions.get(chatId) !== subscription
    ) {
      return;
    }

    subscription.closeMessageStream?.();
    subscription.closeLifecycleStream?.();
    subscription.closeErrorStream?.();
    subscription.connection?.disconnect(1000, 'restarting conversation subscription');
    subscription.closeMessageStream = undefined;
    subscription.closeLifecycleStream = undefined;
    subscription.closeErrorStream = undefined;
    subscription.connection = undefined;
    subscription.reconnectTimer = undefined;
    subscription.started = this.startLiveSubscription(chatId, subscription);
  }

  private scheduleLiveResubscribe(chatId: string, subscription: ConversationLiveSubscription): void {
    if (
      subscription.reconnectTimer
      || subscription.isClosed
      || subscription.handlers.size === 0
      || this.liveSubscriptions.get(chatId) !== subscription
    ) {
      return;
    }

    subscription.reconnectTimer = setTimeout(() => this.restartLiveSubscription(chatId, subscription), 1000);
  }

  private handleLiveMessage(
    fallbackChatId: string,
    decodedMessage: ImDecodedMessage,
    context: ImMessageContext,
  ): void {
    const message = mapLiveMessageToMessage(fallbackChatId, decodedMessage, context);
    const storedMessage = this.upsertLocalMessage(message.chatId, message);
    this.latestReadSeq.set(message.chatId, Math.max(this.latestReadSeq.get(message.chatId) ?? 0, context.sequence));
    const subscription = this.liveSubscriptions.get(fallbackChatId);
    if (subscription) {
      for (const handler of subscription.handlers) {
        handler(storedMessage);
      }
    }
    void context.ack().catch(() => undefined);
  }

  private upsertLocalMessage(chatId: string, message: Message, preferNew = false): Message {
    const messages = this.localMessages.get(chatId) ?? [];
    const byId = new Map(messages.map((item) => [item.id, item]));
    const existingMessage = byId.get(message.id);
    const nextMessage = existingMessage
      ? preferNew
        ? { ...existingMessage, ...message }
        : { ...message, ...existingMessage }
      : message;
    byId.set(message.id, nextMessage);
    this.localMessages.set(
      chatId,
      Array.from(byId.values()).sort((left, right) => left.timestamp - right.timestamp),
    );
    return nextMessage;
  }

  private updateLocalMessage(
    chatId: string,
    messageId: string,
    updater: (message: Message) => Message,
  ): void {
    const messages = this.localMessages.get(chatId) ?? [];
    this.localMessages.set(
      chatId,
      messages.map((message) => message.id === messageId ? updater(message) : message),
    );
  }
}

export function createSdkworkChatService(dependencies?: ChatServiceDependencies | (() => ImSdkClient)): ChatService {
  return new SdkworkChatService(dependencies);
}

export const chatService = createSdkworkChatService();
