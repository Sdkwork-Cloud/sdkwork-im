import type { AuthTokenManager } from '@sdkwork/sdk-common';
import type { AppendStreamFrameRequest, AttachMediaRequest, ContentPart, EditMessageRequest, MediaAsset, MediaResource, MediaResourceType, MessageBody, PostMessageRequest, PostMessageResult, PostRtcSignalRequest, RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscriptionItemInput, RtcSignalEvent, StringMap, StreamFrame } from './generated-backend-types.js';
export type { AckRealtimeEventsRequest, AddConversationMemberRequest, AgentHandoffStateView, AbortStreamRequest, AppendStreamFrameRequest, AttachMediaRequest, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, CheckpointStreamRequest, CompleteStreamRequest, CompleteUploadRequest, ContentPart, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateRtcSessionRequest, CreateSystemChannelRequest, CreateUploadRequest, DeviceSyncFeedResponse, EditMessageRequest, InboxResponse, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, ListMembersResponse, MediaAsset, MediaResource, MediaResourceType, MediaDownloadUrlResponse, MediaUploadSessionResponse, MessageBody, MessageMutationResult, OpenStreamRequest, PostMessageRequest, PostMessageResult, PostRtcSignalRequest, PresenceDeviceRequest, PresenceSnapshotView, RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscriptionItemInput, RealtimeSubscriptionSnapshot, RegisterDeviceRequest, RegisteredDeviceView, QueryParams, RemoveConversationMemberRequest, ResumeSessionRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, SdkworkBackendConfig, SessionResumeView, StringMap, StreamFrame, StreamFrameWindow, StreamSession, SyncRealtimeSubscriptionsRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest, UpdateRtcSessionRequest, } from './generated-backend-types.js';
export interface CrawChatAuthLoginRequest {
    tenantId: string;
    login: string;
    password: string;
    deviceId?: string;
    sessionId?: string;
    clientKind?: string;
}
export interface CrawChatAuthUser {
    id: string;
    login: string;
    name: string;
    role: string;
    email: string;
    actorKind: string;
    clientKind: string;
    permissions: string[];
}
export interface CrawChatWorkspace {
    name: string;
    slug: string;
    tier: string;
    region: string;
    supportPlan: string;
    seats: number;
    activeBrands: number;
    uptime: string;
}
export interface CrawChatAuthLoginResult {
    accessToken: string;
    refreshToken: string;
    expiresAt: number;
    user: CrawChatAuthUser;
    workspace?: CrawChatWorkspace;
}
export interface CrawChatAuthSession {
    tenantId: string;
    user: CrawChatAuthUser;
    workspace?: CrawChatWorkspace;
}
export type CrawChatAppSnapshot = Record<string, unknown>;
export type CrawChatTokenProvider = AuthTokenManager;
export interface CrawChatSdkClientOptions {
    baseUrl?: string;
    apiBaseUrl?: string;
    websocketBaseUrl?: string;
    authToken?: string;
    tokenProvider?: CrawChatTokenProvider;
    webSocketFactory?: CrawChatWebSocketFactory;
}
export interface CrawChatRealtimeSubscriptionScopeOptions {
    deviceId?: string;
}
export interface PostTextMessageOptions extends Omit<PostMessageRequest, 'text'> {
}
export interface EditTextMessageOptions extends Omit<EditMessageRequest, 'text'> {
}
export interface AttachTextMediaOptions extends Omit<AttachMediaRequest, 'text'> {
    text: string;
}
export interface AppendTextFrameOptions extends Omit<AppendStreamFrameRequest, 'frameType' | 'encoding' | 'payload'> {
    text: string;
    encoding?: string;
}
export interface PostJsonRtcSignalOptions extends Omit<PostRtcSignalRequest, 'signalType' | 'payload'> {
    payload: unknown;
    pretty?: boolean;
}
export type CrawChatMessageChannel = 'conversation' | 'system';
export interface CrawChatMessageTarget {
    conversationId: string | number;
    channel?: CrawChatMessageChannel;
}
export type CrawChatMessageKind = 'text' | 'image' | 'video' | 'audio' | 'file' | 'location' | 'link' | 'card' | 'music' | 'contact' | 'sticker' | 'voice' | 'agent' | 'agent_state' | 'agent_handoff' | 'ai_text' | 'ai_image_generation' | 'ai_video_generation' | 'tool_result' | 'workflow_event' | 'data' | 'signal' | 'stream_ref' | 'custom';
export interface CrawChatClientMessage<TKind extends CrawChatMessageKind = CrawChatMessageKind> {
    kind: TKind;
    target: {
        conversationId: string | number;
        channel: CrawChatMessageChannel;
    };
    body: PostMessageRequest;
}
export interface CrawChatPreparedMediaAsset {
    mediaAssetId: string;
    createdAsset: MediaAsset;
    completedAsset: MediaAsset;
}
export interface CrawChatMediaUploadSession {
    mediaAssetId: string;
    mediaAsset: MediaAsset;
    bucket: string;
    objectKey: string;
    storageProvider: string;
    uploadMethod: string;
    uploadUrl: string;
    uploadHeaders?: StringMap;
    uploadExpiresInSeconds?: number;
    requestKey?: string;
    deliveryStatus?: 'applied' | 'replayed';
    proofVersion?: string;
}
export interface CrawChatMediaUploadOptions {
    mediaAssetId: string;
    bucket: string;
    objectKey?: string;
    resource: MediaResource;
    body: Exclude<RequestInit['body'], null | undefined>;
    expiresInSeconds?: number;
    checksum?: string;
}
export interface CrawChatUploadedMediaAsset extends CrawChatPreparedMediaAsset {
    asset: MediaAsset;
    url?: string;
    session: CrawChatMediaUploadSession;
    etag?: string;
}
export interface CrawChatCreateMessageOptions extends Omit<PostMessageRequest, 'text' | 'parts'> {
    channel?: CrawChatMessageChannel;
}
export interface CrawChatCreateMediaMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    schemaRef?: string;
    encoding?: string;
    payload?: string;
}
export interface CrawChatCreateDataMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    schemaRef?: string;
    encoding?: string;
    payload: string;
}
export interface CrawChatCreateSignalMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    signalType: string;
    schemaRef?: string;
    encoding?: string;
    payload?: string;
    state?: string;
}
export interface CrawChatCreateStreamReferenceMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    streamId: string | number;
    streamType?: string;
    state?: string;
}
export type CrawChatStructuredMessageKind = 'location' | 'link' | 'card' | 'music' | 'contact' | 'sticker' | 'voice' | 'agent' | 'agent_state' | 'agent_handoff' | 'ai_text' | 'ai_image_generation' | 'ai_video_generation' | 'tool_result' | 'workflow_event';
export interface CrawChatCreateStructuredMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
}
export interface CrawChatCreateLocationMessageOptions extends CrawChatCreateStructuredMessageOptions {
    latitude: number;
    longitude: number;
    name?: string;
    address?: string;
    mapUrl?: string;
}
export interface CrawChatCreateLinkMessageOptions extends CrawChatCreateStructuredMessageOptions {
    title: string;
    url: string;
    description?: string;
    imageUrl?: string;
    siteName?: string;
}
export interface CrawChatCardAction {
    label: string;
    url?: string;
    action?: string;
    payload?: unknown;
}
export interface CrawChatCreateCardMessageOptions extends CrawChatCreateStructuredMessageOptions {
    title: string;
    subtitle?: string;
    imageUrl?: string;
    actions?: CrawChatCardAction[];
}
export interface CrawChatCreateMusicMessageOptions extends CrawChatCreateStructuredMessageOptions {
    title: string;
    artist?: string;
    album?: string;
    url: string;
    coverUrl?: string;
    durationSeconds?: number;
}
export interface CrawChatCreateContactMessageOptions extends CrawChatCreateStructuredMessageOptions {
    displayName: string;
    avatarUrl?: string;
    description?: string;
    profileUrl?: string;
    contactId?: string;
}
export interface CrawChatCreateStickerMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    stickerId?: string;
    packId?: string;
    emoji?: string;
}
export interface CrawChatCreateVoiceMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    durationSeconds?: number;
    transcription?: string;
    waveform?: number[];
}
export interface CrawChatCreateAgentMessageOptions extends CrawChatCreateStructuredMessageOptions {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}
export interface CrawChatCreateAgentStateMessageOptions extends CrawChatCreateStructuredMessageOptions {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}
export interface CrawChatCreateAgentHandoffMessageOptions extends CrawChatCreateStructuredMessageOptions {
    handoffId?: string;
    fromAgentId: string;
    fromAgentName?: string;
    toAgentId: string;
    toAgentName?: string;
    reason?: string;
    status?: string;
}
export interface CrawChatCreateCustomMessageOptions extends CrawChatCreateStructuredMessageOptions {
    customType: string;
    data?: unknown;
}
export interface CrawChatCreateAiTextMessageOptions extends CrawChatCreateStructuredMessageOptions {
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}
export interface CrawChatCreateAiImageGenerationMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}
export interface CrawChatCreateAiVideoGenerationMessageOptions extends CrawChatCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    prompt: string;
    status?: string;
    model?: string;
    durationSeconds?: number;
}
export interface CrawChatCreateToolResultMessageOptions extends CrawChatCreateStructuredMessageOptions {
    toolName: string;
    invocationId: string;
    status?: string;
    output?: unknown;
    error?: unknown;
}
export interface CrawChatCreateWorkflowEventMessageOptions extends CrawChatCreateStructuredMessageOptions {
    workflowId: string;
    eventName: string;
    stage?: string;
    status?: string;
    data?: unknown;
}
export type CrawChatDecodableMessageBody = MessageBody | PostMessageRequest;
export interface CrawChatDecodedMessageAttachment {
    type?: MediaResourceType;
    mediaAssetId?: string;
    resource?: MediaResource;
    schemaRef?: string;
    encoding?: string;
    payload?: unknown;
}
export interface CrawChatDecodedDataPayload {
    schemaRef?: string;
    encoding?: string;
    payload: unknown;
    rawPayload?: string;
}
export interface CrawChatDecodedSignalPayload {
    signalType?: string;
    schemaRef?: string;
    encoding?: string;
    state?: string;
    payload: unknown;
    rawPayload?: string;
}
export interface CrawChatDecodedStreamReferencePayload {
    streamId?: string;
    streamType?: string;
    state?: string;
}
export interface CrawChatDecodedMessageBase<TType extends CrawChatMessageKind, TContent> {
    type: TType;
    text?: string;
    summary?: string;
    renderHints?: StringMap;
    content: TContent;
    attachments: CrawChatDecodedMessageAttachment[];
    parts: ContentPart[];
    rawBody: CrawChatDecodableMessageBody;
}
export interface CrawChatDecodedTextMessage extends CrawChatDecodedMessageBase<'text', {
    text: string;
}> {
}
export interface CrawChatDecodedMediaMessage<TType extends 'image' | 'video' | 'audio' | 'file'> extends CrawChatDecodedMessageBase<TType, {
    mediaType: TType;
}> {
}
export interface CrawChatDecodedLocationMessage extends CrawChatDecodedMessageBase<'location', {
    latitude: number;
    longitude: number;
    name?: string;
    address?: string;
    mapUrl?: string;
}> {
}
export interface CrawChatDecodedLinkMessage extends CrawChatDecodedMessageBase<'link', {
    title?: string;
    url: string;
    description?: string;
    imageUrl?: string;
    siteName?: string;
}> {
}
export interface CrawChatDecodedCardMessage extends CrawChatDecodedMessageBase<'card', {
    title: string;
    subtitle?: string;
    imageUrl?: string;
    actions: CrawChatCardAction[];
}> {
}
export interface CrawChatDecodedMusicMessage extends CrawChatDecodedMessageBase<'music', {
    title: string;
    artist?: string;
    album?: string;
    url: string;
    coverUrl?: string;
    durationSeconds?: number;
}> {
}
export interface CrawChatDecodedContactMessage extends CrawChatDecodedMessageBase<'contact', {
    displayName: string;
    avatarUrl?: string;
    description?: string;
    profileUrl?: string;
    contactId?: string;
}> {
}
export interface CrawChatDecodedStickerMessage extends CrawChatDecodedMessageBase<'sticker', {
    stickerId?: string;
    packId?: string;
    emoji?: string;
}> {
}
export interface CrawChatDecodedVoiceMessage extends CrawChatDecodedMessageBase<'voice', {
    durationSeconds?: number;
    transcription?: string;
    waveform?: number[];
}> {
}
export interface CrawChatDecodedAgentMessage extends CrawChatDecodedMessageBase<'agent', {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}> {
}
export interface CrawChatDecodedAgentStateMessage extends CrawChatDecodedMessageBase<'agent_state', {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}> {
}
export interface CrawChatDecodedAgentHandoffMessage extends CrawChatDecodedMessageBase<'agent_handoff', {
    handoffId?: string;
    fromAgentId: string;
    fromAgentName?: string;
    toAgentId: string;
    toAgentName?: string;
    reason?: string;
    status?: string;
}> {
}
export interface CrawChatDecodedAiTextMessage extends CrawChatDecodedMessageBase<'ai_text', {
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}> {
}
export interface CrawChatDecodedAiImageGenerationMessage extends CrawChatDecodedMessageBase<'ai_image_generation', {
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}> {
}
export interface CrawChatDecodedAiVideoGenerationMessage extends CrawChatDecodedMessageBase<'ai_video_generation', {
    prompt: string;
    status?: string;
    model?: string;
    durationSeconds?: number;
}> {
}
export interface CrawChatDecodedToolResultMessage extends CrawChatDecodedMessageBase<'tool_result', {
    toolName: string;
    invocationId: string;
    status?: string;
    output?: unknown;
    error?: unknown;
}> {
}
export interface CrawChatDecodedWorkflowEventMessage extends CrawChatDecodedMessageBase<'workflow_event', {
    workflowId: string;
    eventName: string;
    stage?: string;
    status?: string;
    data?: unknown;
}> {
}
export interface CrawChatDecodedDataMessage extends CrawChatDecodedMessageBase<'data', CrawChatDecodedDataPayload> {
}
export interface CrawChatDecodedSignalMessage extends CrawChatDecodedMessageBase<'signal', CrawChatDecodedSignalPayload> {
}
export interface CrawChatDecodedStreamReferenceMessage extends CrawChatDecodedMessageBase<'stream_ref', CrawChatDecodedStreamReferencePayload> {
}
export interface CrawChatDecodedCustomMessage extends CrawChatDecodedMessageBase<'custom', {
    customType?: string;
    schemaRef?: string;
    data?: unknown;
    parts?: ContentPart[];
}> {
}
export type CrawChatDecodedMessage = CrawChatDecodedTextMessage | CrawChatDecodedMediaMessage<'image'> | CrawChatDecodedMediaMessage<'video'> | CrawChatDecodedMediaMessage<'audio'> | CrawChatDecodedMediaMessage<'file'> | CrawChatDecodedLocationMessage | CrawChatDecodedLinkMessage | CrawChatDecodedCardMessage | CrawChatDecodedMusicMessage | CrawChatDecodedContactMessage | CrawChatDecodedStickerMessage | CrawChatDecodedVoiceMessage | CrawChatDecodedAgentMessage | CrawChatDecodedAgentStateMessage | CrawChatDecodedAgentHandoffMessage | CrawChatDecodedAiTextMessage | CrawChatDecodedAiImageGenerationMessage | CrawChatDecodedAiVideoGenerationMessage | CrawChatDecodedToolResultMessage | CrawChatDecodedWorkflowEventMessage | CrawChatDecodedDataMessage | CrawChatDecodedSignalMessage | CrawChatDecodedStreamReferenceMessage | CrawChatDecodedCustomMessage;
export interface CrawChatDecodedRtcSignal {
    rtcSessionId: string | number;
    signalType: string;
    schemaRef?: string;
    payload: unknown;
    conversationId?: string;
    rtcMode?: string;
    signalingStreamId?: string;
    occurredAt?: string;
    rawSignal?: RtcSignalEvent;
}
export interface CrawChatDecodedStreamFrame {
    streamId: string | number;
    streamType?: string;
    frameType: string;
    schemaRef?: string;
    encoding?: string;
    payload: unknown;
    attributes: StringMap;
    occurredAt?: string;
    rawFrame: StreamFrame;
}
export type CrawChatSubscription = () => void;
export interface CrawChatRealtimeConnectedFrame {
    type: 'realtime.connected';
    tenantId?: string;
    principalId?: string;
    deviceId?: string;
    actor?: {
        id?: string;
        kind?: string;
    };
    sender?: {
        principalId?: string;
        deviceId?: string;
        sessionId?: string;
        senderId?: string;
    };
    ackedThroughSeq?: number;
    trimmedThroughSeq?: number;
    latestRealtimeSeq?: number;
}
export interface CrawChatRealtimeErrorFrame {
    type: 'error';
    requestId?: string | null;
    code: string;
    message: string;
}
export interface CrawChatWebSocketMessageEventLike {
    data: unknown;
}
export interface CrawChatWebSocketCloseEventLike {
    code?: number;
    reason?: string;
    wasClean?: boolean;
}
export interface CrawChatWebSocketLike {
    send(data: string): void | Promise<void>;
    close(code?: number, reason?: string): void;
    readyState?: number;
    addEventListener?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): void;
    removeEventListener?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): void;
    on?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): unknown;
    off?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): unknown;
    onopen?: ((event: unknown) => void) | null;
    onmessage?: ((event: CrawChatWebSocketMessageEventLike) => void) | null;
    onclose?: ((event: CrawChatWebSocketCloseEventLike) => void) | null;
    onerror?: ((event: unknown) => void) | null;
}
export interface CrawChatRealtimeWebSocketFactoryRequest {
    url: string;
    protocols: string[];
    headers: Record<string, string>;
    authToken?: string;
}
export type CrawChatWebSocketFactory = (request: CrawChatRealtimeWebSocketFactoryRequest) => CrawChatWebSocketLike | Promise<CrawChatWebSocketLike>;
export interface CrawChatUploadAndSendMessageOptions<TKind extends CrawChatMessageKind = CrawChatMessageKind> {
    upload: CrawChatMediaUploadOptions;
    createMessage: (uploaded: CrawChatUploadedMediaAsset) => CrawChatClientMessage<TKind>;
}
export interface CrawChatUploadAndSendMessageResult<TKind extends CrawChatMessageKind = CrawChatMessageKind> extends CrawChatUploadedMediaAsset {
    message: CrawChatClientMessage<TKind>;
    delivery: PostMessageResult;
}
export interface CrawChatCreateTextInput extends CrawChatCreateMessageOptions {
    conversationId: string | number;
    text: string;
}
export interface CrawChatCreateMediaInput extends CrawChatCreateMediaMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateDataInput extends CrawChatCreateDataMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateSignalInput extends CrawChatCreateSignalMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateStreamReferenceInput extends CrawChatCreateStreamReferenceMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateLocationInput extends CrawChatCreateLocationMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateLinkInput extends CrawChatCreateLinkMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateCardInput extends CrawChatCreateCardMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateMusicInput extends CrawChatCreateMusicMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateContactInput extends CrawChatCreateContactMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateStickerInput extends CrawChatCreateStickerMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateVoiceInput extends CrawChatCreateVoiceMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateAgentInput extends CrawChatCreateAgentMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateAgentStateInput extends CrawChatCreateAgentStateMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateAgentHandoffInput extends CrawChatCreateAgentHandoffMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateCustomInput extends CrawChatCreateCustomMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateAiTextInput extends CrawChatCreateAiTextMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateAiImageGenerationInput extends CrawChatCreateAiImageGenerationMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateAiVideoGenerationInput extends CrawChatCreateAiVideoGenerationMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateToolResultInput extends CrawChatCreateToolResultMessageOptions {
    conversationId: string | number;
}
export interface CrawChatCreateWorkflowEventInput extends CrawChatCreateWorkflowEventMessageOptions {
    conversationId: string | number;
}
export type CrawChatReceiveSource = 'live' | 'catch_up';
export type CrawChatLiveConnectionStatus = 'connected' | 'error' | 'closed';
export interface CrawChatReceiveSender {
    principalId?: string;
    deviceId?: string;
}
export interface CrawChatReceiveContextBase<TKind extends string> {
    kind: TKind;
    sequence: number;
    source: CrawChatReceiveSource;
    receivedAt?: string;
    sender?: CrawChatReceiveSender;
    eventType: string;
    scopeType: string;
    scopeId: string;
    payload: unknown;
    rawEvent: RealtimeEvent;
    ack: () => Promise<RealtimeAckState>;
}
export interface CrawChatMessageContext extends CrawChatReceiveContextBase<'message'> {
    messageId?: string;
    conversationId?: string;
    message: CrawChatDecodedMessage;
}
export interface CrawChatDataContext extends CrawChatReceiveContextBase<'data'> {
    data: CrawChatDecodedDataPayload;
}
export interface CrawChatSignalContext extends CrawChatReceiveContextBase<'signal'> {
    signal: CrawChatDecodedRtcSignal;
}
export interface CrawChatUnknownContext extends CrawChatReceiveContextBase<'unknown'> {
}
export type CrawChatReceiveContext = CrawChatMessageContext | CrawChatDataContext | CrawChatSignalContext | CrawChatUnknownContext;
export interface CrawChatLiveState {
    status: CrawChatLiveConnectionStatus;
    connectedFrame?: CrawChatRealtimeConnectedFrame;
    error?: unknown;
    closeEvent?: CrawChatWebSocketCloseEventLike | unknown;
    updatedAt: string;
}
export interface CrawChatLiveErrorContext {
    code: string;
    source: 'realtime' | 'socket';
    error: unknown;
    requestId?: string;
    frame?: CrawChatRealtimeErrorFrame;
}
export interface CrawChatCatchUpBatch {
    items: CrawChatReceiveContext[];
    highestSequence: number;
    rawWindow: RealtimeEventWindow;
}
export interface CrawChatRealtimeSubscriptionGroups {
    conversations?: Array<string | number>;
    rtcSessions?: Array<string | number>;
    items?: RealtimeSubscriptionItemInput[];
}
export interface CrawChatConnectOptions {
    deviceId?: string;
    subscriptions?: CrawChatRealtimeSubscriptionGroups;
    socket?: CrawChatWebSocketLike;
    url?: string;
    headers?: Record<string, string>;
    protocols?: string[];
    requestTimeoutMs?: number;
}
export interface CrawChatLiveMessageStream {
    on(handler: (message: CrawChatDecodedMessage, context: CrawChatMessageContext) => void): CrawChatSubscription;
    onConversation(conversationId: string | number, handler: (message: CrawChatDecodedMessage, context: CrawChatMessageContext) => void): CrawChatSubscription;
}
export interface CrawChatLiveDataStream {
    on(handler: (data: CrawChatDecodedDataPayload, context: CrawChatDataContext) => void): CrawChatSubscription;
}
export interface CrawChatLiveSignalStream {
    on(handler: (signal: CrawChatDecodedRtcSignal, context: CrawChatSignalContext) => void): CrawChatSubscription;
    onRtcSession(rtcSessionId: string | number, handler: (signal: CrawChatDecodedRtcSignal, context: CrawChatSignalContext) => void): CrawChatSubscription;
}
export interface CrawChatLiveEventStream {
    on(handler: (context: CrawChatReceiveContext) => void): CrawChatSubscription;
}
export interface CrawChatLiveLifecycleStream {
    onStateChange(handler: (state: CrawChatLiveState) => void): CrawChatSubscription;
    onError(handler: (context: CrawChatLiveErrorContext) => void): CrawChatSubscription;
    getState(): CrawChatLiveState;
}
export interface CrawChatLiveConnection {
    messages: CrawChatLiveMessageStream;
    data: CrawChatLiveDataStream;
    signals: CrawChatLiveSignalStream;
    events: CrawChatLiveEventStream;
    lifecycle: CrawChatLiveLifecycleStream;
    disconnect(code?: number, reason?: string): void;
}
//# sourceMappingURL=types.d.ts.map