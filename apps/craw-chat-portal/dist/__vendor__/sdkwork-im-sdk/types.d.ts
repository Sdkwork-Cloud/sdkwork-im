import type { AuthTokenManager, AppendStreamFrameRequest, AttachMediaRequest, ContentPart, EditMessageRequest, MediaAsset, MediaResource, MediaResourceType, MessageBody, PostMessageRequest, PostMessageResult, PostRtcSignalRequest, RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscriptionItemInput, RtcSignalEvent, StringMap, StreamFrame } from './generated-client-types.js';
export type { AckRealtimeEventsRequest, AddConversationMemberRequest, AgentHandoffStateView, AbortStreamRequest, AppendStreamFrameRequest, AttachMediaRequest, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, CheckpointStreamRequest, CompleteStreamRequest, CompleteUploadRequest, ContentPart, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateRtcSessionRequest, CreateSystemChannelRequest, CreateUploadRequest, DeviceSyncFeedResponse, EditMessageRequest, InboxResponse, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, ListMembersResponse, MediaAsset, MediaResource, MediaResourceType, MediaDownloadUrlResponse, MediaUploadSessionResponse, MessageBody, MessageMutationResult, OpenStreamRequest, PostMessageRequest, PostMessageResult, PostRtcSignalRequest, PresenceDeviceRequest, PresenceSnapshotView, RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscriptionItemInput, RealtimeSubscriptionSnapshot, RegisterDeviceRequest, RegisteredDeviceView, QueryParams, RemoveConversationMemberRequest, ResumeSessionRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, SessionResumeView, StringMap, StreamFrame, StreamFrameWindow, StreamSession, SyncRealtimeSubscriptionsRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest, UpdateRtcSessionRequest, } from './generated-client-types.js';
export interface ImAuthLoginRequest {
    tenantId: string;
    login: string;
    password: string;
    deviceId?: string;
    sessionId?: string;
    clientKind?: string;
}
export interface ImAuthUser {
    id: string;
    login: string;
    name: string;
    role: string;
    email: string;
    actorKind: string;
    clientKind: string;
    permissions: string[];
}
export interface ImWorkspace {
    name: string;
    slug: string;
    tier: string;
    region: string;
    supportPlan: string;
    seats: number;
    activeBrands: number;
    uptime: string;
}
export interface ImAuthLoginResult {
    accessToken: string;
    refreshToken: string;
    expiresAt: number;
    user: ImAuthUser;
    workspace?: ImWorkspace;
}
export interface ImAuthSession {
    tenantId: string;
    user: ImAuthUser;
    workspace?: ImWorkspace;
}
export type ImAppSnapshot = Record<string, unknown>;
export type ImTokenProvider = AuthTokenManager;
export interface ImSdkClientOptions {
    baseUrl?: string;
    apiBaseUrl?: string;
    websocketBaseUrl?: string;
    authToken?: string;
    tokenProvider?: ImTokenProvider;
    webSocketFactory?: ImWebSocketFactory;
}
export interface ImRealtimeSubscriptionScopeOptions {
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
export type ImMessageChannel = 'conversation' | 'system';
export interface ImMessageTarget {
    conversationId: string | number;
    channel?: ImMessageChannel;
}
export type ImMessageKind = 'text' | 'image' | 'video' | 'audio' | 'file' | 'location' | 'link' | 'card' | 'music' | 'contact' | 'sticker' | 'voice' | 'agent' | 'agent_state' | 'agent_handoff' | 'ai_text' | 'ai_image_generation' | 'ai_video_generation' | 'tool_result' | 'workflow_event' | 'data' | 'signal' | 'stream_ref' | 'custom';
export interface ImClientMessage<TKind extends ImMessageKind = ImMessageKind> {
    kind: TKind;
    target: {
        conversationId: string | number;
        channel: ImMessageChannel;
    };
    body: PostMessageRequest;
}
export interface ImPreparedMediaAsset {
    mediaAssetId: string;
    createdAsset: MediaAsset;
    completedAsset: MediaAsset;
}
export interface ImMediaUploadSession {
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
export interface ImMediaUploadOptions {
    mediaAssetId: string;
    bucket: string;
    objectKey?: string;
    resource: MediaResource;
    body: Exclude<RequestInit['body'], null | undefined>;
    expiresInSeconds?: number;
    checksum?: string;
}
export interface ImUploadedMediaAsset extends ImPreparedMediaAsset {
    asset: MediaAsset;
    url?: string;
    session: ImMediaUploadSession;
    etag?: string;
}
export interface ImCreateMessageOptions extends Omit<PostMessageRequest, 'text' | 'parts'> {
    channel?: ImMessageChannel;
}
export interface ImCreateMediaMessageOptions extends ImCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    schemaRef?: string;
    encoding?: string;
    payload?: string;
}
export interface ImCreateDataMessageOptions extends ImCreateMessageOptions {
    text?: string;
    schemaRef?: string;
    encoding?: string;
    payload: string;
}
export interface ImCreateSignalMessageOptions extends ImCreateMessageOptions {
    text?: string;
    signalType: string;
    schemaRef?: string;
    encoding?: string;
    payload?: string;
    state?: string;
}
export interface ImCreateStreamReferenceMessageOptions extends ImCreateMessageOptions {
    text?: string;
    streamId: string | number;
    streamType?: string;
    state?: string;
}
export type ImStructuredMessageKind = 'location' | 'link' | 'card' | 'music' | 'contact' | 'sticker' | 'voice' | 'agent' | 'agent_state' | 'agent_handoff' | 'ai_text' | 'ai_image_generation' | 'ai_video_generation' | 'tool_result' | 'workflow_event';
export interface ImCreateStructuredMessageOptions extends ImCreateMessageOptions {
    text?: string;
}
export interface ImCreateLocationMessageOptions extends ImCreateStructuredMessageOptions {
    latitude: number;
    longitude: number;
    name?: string;
    address?: string;
    mapUrl?: string;
}
export interface ImCreateLinkMessageOptions extends ImCreateStructuredMessageOptions {
    title: string;
    url: string;
    description?: string;
    imageUrl?: string;
    siteName?: string;
}
export interface ImCardAction {
    label: string;
    url?: string;
    action?: string;
    payload?: unknown;
}
export interface ImCreateCardMessageOptions extends ImCreateStructuredMessageOptions {
    title: string;
    subtitle?: string;
    imageUrl?: string;
    actions?: ImCardAction[];
}
export interface ImCreateMusicMessageOptions extends ImCreateStructuredMessageOptions {
    title: string;
    artist?: string;
    album?: string;
    url: string;
    coverUrl?: string;
    durationSeconds?: number;
}
export interface ImCreateContactMessageOptions extends ImCreateStructuredMessageOptions {
    displayName: string;
    avatarUrl?: string;
    description?: string;
    profileUrl?: string;
    contactId?: string;
}
export interface ImCreateStickerMessageOptions extends ImCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    stickerId?: string;
    packId?: string;
    emoji?: string;
}
export interface ImCreateVoiceMessageOptions extends ImCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    durationSeconds?: number;
    transcription?: string;
    waveform?: number[];
}
export interface ImCreateAgentMessageOptions extends ImCreateStructuredMessageOptions {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}
export interface ImCreateAgentStateMessageOptions extends ImCreateStructuredMessageOptions {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}
export interface ImCreateAgentHandoffMessageOptions extends ImCreateStructuredMessageOptions {
    handoffId?: string;
    fromAgentId: string;
    fromAgentName?: string;
    toAgentId: string;
    toAgentName?: string;
    reason?: string;
    status?: string;
}
export interface ImCreateCustomMessageOptions extends ImCreateStructuredMessageOptions {
    customType: string;
    data?: unknown;
}
export interface ImCreateAiTextMessageOptions extends ImCreateStructuredMessageOptions {
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}
export interface ImCreateAiImageGenerationMessageOptions extends ImCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}
export interface ImCreateAiVideoGenerationMessageOptions extends ImCreateMessageOptions {
    text?: string;
    mediaAssetId?: string | number;
    resource?: MediaResource;
    prompt: string;
    status?: string;
    model?: string;
    durationSeconds?: number;
}
export interface ImCreateToolResultMessageOptions extends ImCreateStructuredMessageOptions {
    toolName: string;
    invocationId: string;
    status?: string;
    output?: unknown;
    error?: unknown;
}
export interface ImCreateWorkflowEventMessageOptions extends ImCreateStructuredMessageOptions {
    workflowId: string;
    eventName: string;
    stage?: string;
    status?: string;
    data?: unknown;
}
export type ImDecodableMessageBody = MessageBody | PostMessageRequest;
export interface ImDecodedMessageAttachment {
    type?: MediaResourceType;
    mediaAssetId?: string;
    resource?: MediaResource;
    schemaRef?: string;
    encoding?: string;
    payload?: unknown;
}
export interface ImDecodedDataPayload {
    schemaRef?: string;
    encoding?: string;
    payload: unknown;
    rawPayload?: string;
}
export interface ImDecodedSignalPayload {
    signalType?: string;
    schemaRef?: string;
    encoding?: string;
    state?: string;
    payload: unknown;
    rawPayload?: string;
}
export interface ImDecodedStreamReferencePayload {
    streamId?: string;
    streamType?: string;
    state?: string;
}
export interface ImDecodedMessageBase<TType extends ImMessageKind, TContent> {
    type: TType;
    text?: string;
    summary?: string;
    renderHints?: StringMap;
    content: TContent;
    attachments: ImDecodedMessageAttachment[];
    parts: ContentPart[];
    rawBody: ImDecodableMessageBody;
}
export interface ImDecodedTextMessage extends ImDecodedMessageBase<'text', {
    text: string;
}> {
}
export interface ImDecodedMediaMessage<TType extends 'image' | 'video' | 'audio' | 'file'> extends ImDecodedMessageBase<TType, {
    mediaType: TType;
}> {
}
export interface ImDecodedLocationMessage extends ImDecodedMessageBase<'location', {
    latitude: number;
    longitude: number;
    name?: string;
    address?: string;
    mapUrl?: string;
}> {
}
export interface ImDecodedLinkMessage extends ImDecodedMessageBase<'link', {
    title?: string;
    url: string;
    description?: string;
    imageUrl?: string;
    siteName?: string;
}> {
}
export interface ImDecodedCardMessage extends ImDecodedMessageBase<'card', {
    title: string;
    subtitle?: string;
    imageUrl?: string;
    actions: ImCardAction[];
}> {
}
export interface ImDecodedMusicMessage extends ImDecodedMessageBase<'music', {
    title: string;
    artist?: string;
    album?: string;
    url: string;
    coverUrl?: string;
    durationSeconds?: number;
}> {
}
export interface ImDecodedContactMessage extends ImDecodedMessageBase<'contact', {
    displayName: string;
    avatarUrl?: string;
    description?: string;
    profileUrl?: string;
    contactId?: string;
}> {
}
export interface ImDecodedStickerMessage extends ImDecodedMessageBase<'sticker', {
    stickerId?: string;
    packId?: string;
    emoji?: string;
}> {
}
export interface ImDecodedVoiceMessage extends ImDecodedMessageBase<'voice', {
    durationSeconds?: number;
    transcription?: string;
    waveform?: number[];
}> {
}
export interface ImDecodedAgentMessage extends ImDecodedMessageBase<'agent', {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}> {
}
export interface ImDecodedAgentStateMessage extends ImDecodedMessageBase<'agent_state', {
    agentId: string;
    agentName?: string;
    stage?: string;
    status?: string;
    capabilities?: string[];
}> {
}
export interface ImDecodedAgentHandoffMessage extends ImDecodedMessageBase<'agent_handoff', {
    handoffId?: string;
    fromAgentId: string;
    fromAgentName?: string;
    toAgentId: string;
    toAgentName?: string;
    reason?: string;
    status?: string;
}> {
}
export interface ImDecodedAiTextMessage extends ImDecodedMessageBase<'ai_text', {
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}> {
}
export interface ImDecodedAiImageGenerationMessage extends ImDecodedMessageBase<'ai_image_generation', {
    prompt: string;
    status?: string;
    model?: string;
    revisedPrompt?: string;
}> {
}
export interface ImDecodedAiVideoGenerationMessage extends ImDecodedMessageBase<'ai_video_generation', {
    prompt: string;
    status?: string;
    model?: string;
    durationSeconds?: number;
}> {
}
export interface ImDecodedToolResultMessage extends ImDecodedMessageBase<'tool_result', {
    toolName: string;
    invocationId: string;
    status?: string;
    output?: unknown;
    error?: unknown;
}> {
}
export interface ImDecodedWorkflowEventMessage extends ImDecodedMessageBase<'workflow_event', {
    workflowId: string;
    eventName: string;
    stage?: string;
    status?: string;
    data?: unknown;
}> {
}
export interface ImDecodedDataMessage extends ImDecodedMessageBase<'data', ImDecodedDataPayload> {
}
export interface ImDecodedSignalMessage extends ImDecodedMessageBase<'signal', ImDecodedSignalPayload> {
}
export interface ImDecodedStreamReferenceMessage extends ImDecodedMessageBase<'stream_ref', ImDecodedStreamReferencePayload> {
}
export interface ImDecodedCustomMessage extends ImDecodedMessageBase<'custom', {
    customType?: string;
    schemaRef?: string;
    data?: unknown;
    parts?: ContentPart[];
}> {
}
export type ImDecodedMessage = ImDecodedTextMessage | ImDecodedMediaMessage<'image'> | ImDecodedMediaMessage<'video'> | ImDecodedMediaMessage<'audio'> | ImDecodedMediaMessage<'file'> | ImDecodedLocationMessage | ImDecodedLinkMessage | ImDecodedCardMessage | ImDecodedMusicMessage | ImDecodedContactMessage | ImDecodedStickerMessage | ImDecodedVoiceMessage | ImDecodedAgentMessage | ImDecodedAgentStateMessage | ImDecodedAgentHandoffMessage | ImDecodedAiTextMessage | ImDecodedAiImageGenerationMessage | ImDecodedAiVideoGenerationMessage | ImDecodedToolResultMessage | ImDecodedWorkflowEventMessage | ImDecodedDataMessage | ImDecodedSignalMessage | ImDecodedStreamReferenceMessage | ImDecodedCustomMessage;
export interface ImDecodedRtcSignal {
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
export interface ImDecodedStreamFrame {
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
export type ImSubscription = () => void;
export interface ImRealtimeConnectedFrame {
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
export interface ImRealtimeErrorFrame {
    type: 'error';
    requestId?: string | null;
    code: string;
    message: string;
}
export interface ImWebSocketMessageEventLike {
    data: unknown;
}
export interface ImWebSocketCloseEventLike {
    code?: number;
    reason?: string;
    wasClean?: boolean;
}
export interface ImWebSocketLike {
    send(data: string): void | Promise<void>;
    close(code?: number, reason?: string): void;
    readyState?: number;
    addEventListener?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): void;
    removeEventListener?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): void;
    on?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): unknown;
    off?(type: 'open' | 'message' | 'close' | 'error', listener: (event: unknown) => void): unknown;
    onopen?: ((event: unknown) => void) | null;
    onmessage?: ((event: ImWebSocketMessageEventLike) => void) | null;
    onclose?: ((event: ImWebSocketCloseEventLike) => void) | null;
    onerror?: ((event: unknown) => void) | null;
}
export interface ImRealtimeWebSocketFactoryRequest {
    url: string;
    protocols: string[];
    headers: Record<string, string>;
    authToken?: string;
}
export type ImWebSocketFactory = (request: ImRealtimeWebSocketFactoryRequest) => ImWebSocketLike | Promise<ImWebSocketLike>;
export interface ImUploadAndSendMessageOptions<TKind extends ImMessageKind = ImMessageKind> {
    upload: ImMediaUploadOptions;
    createMessage: (uploaded: ImUploadedMediaAsset) => ImClientMessage<TKind>;
}
export interface ImUploadAndSendMessageResult<TKind extends ImMessageKind = ImMessageKind> extends ImUploadedMediaAsset {
    message: ImClientMessage<TKind>;
    delivery: PostMessageResult;
}
export interface ImCreateTextInput extends ImCreateMessageOptions {
    conversationId: string | number;
    text: string;
}
export interface ImCreateMediaInput extends ImCreateMediaMessageOptions {
    conversationId: string | number;
}
export interface ImCreateDataInput extends ImCreateDataMessageOptions {
    conversationId: string | number;
}
export interface ImCreateSignalInput extends ImCreateSignalMessageOptions {
    conversationId: string | number;
}
export interface ImCreateStreamReferenceInput extends ImCreateStreamReferenceMessageOptions {
    conversationId: string | number;
}
export interface ImCreateLocationInput extends ImCreateLocationMessageOptions {
    conversationId: string | number;
}
export interface ImCreateLinkInput extends ImCreateLinkMessageOptions {
    conversationId: string | number;
}
export interface ImCreateCardInput extends ImCreateCardMessageOptions {
    conversationId: string | number;
}
export interface ImCreateMusicInput extends ImCreateMusicMessageOptions {
    conversationId: string | number;
}
export interface ImCreateContactInput extends ImCreateContactMessageOptions {
    conversationId: string | number;
}
export interface ImCreateStickerInput extends ImCreateStickerMessageOptions {
    conversationId: string | number;
}
export interface ImCreateVoiceInput extends ImCreateVoiceMessageOptions {
    conversationId: string | number;
}
export interface ImCreateAgentInput extends ImCreateAgentMessageOptions {
    conversationId: string | number;
}
export interface ImCreateAgentStateInput extends ImCreateAgentStateMessageOptions {
    conversationId: string | number;
}
export interface ImCreateAgentHandoffInput extends ImCreateAgentHandoffMessageOptions {
    conversationId: string | number;
}
export interface ImCreateCustomInput extends ImCreateCustomMessageOptions {
    conversationId: string | number;
}
export interface ImCreateAiTextInput extends ImCreateAiTextMessageOptions {
    conversationId: string | number;
}
export interface ImCreateAiImageGenerationInput extends ImCreateAiImageGenerationMessageOptions {
    conversationId: string | number;
}
export interface ImCreateAiVideoGenerationInput extends ImCreateAiVideoGenerationMessageOptions {
    conversationId: string | number;
}
export interface ImCreateToolResultInput extends ImCreateToolResultMessageOptions {
    conversationId: string | number;
}
export interface ImCreateWorkflowEventInput extends ImCreateWorkflowEventMessageOptions {
    conversationId: string | number;
}
export type ImReceiveSource = 'live' | 'catch_up';
export type ImLiveConnectionStatus = 'connected' | 'error' | 'closed';
export interface ImReceiveSender {
    principalId?: string;
    deviceId?: string;
}
export interface ImReceiveContextBase<TKind extends string> {
    kind: TKind;
    sequence: number;
    source: ImReceiveSource;
    receivedAt?: string;
    sender?: ImReceiveSender;
    eventType: string;
    scopeType: string;
    scopeId: string;
    payload: unknown;
    rawEvent: RealtimeEvent;
    ack: () => Promise<RealtimeAckState>;
}
export interface ImMessageContext extends ImReceiveContextBase<'message'> {
    messageId?: string;
    conversationId?: string;
    message: ImDecodedMessage;
}
export interface ImDataContext extends ImReceiveContextBase<'data'> {
    data: ImDecodedDataPayload;
}
export interface ImSignalContext extends ImReceiveContextBase<'signal'> {
    signal: ImDecodedRtcSignal;
}
export interface ImUnknownContext extends ImReceiveContextBase<'unknown'> {
}
export type ImReceiveContext = ImMessageContext | ImDataContext | ImSignalContext | ImUnknownContext;
export interface ImLiveState {
    status: ImLiveConnectionStatus;
    connectedFrame?: ImRealtimeConnectedFrame;
    error?: unknown;
    closeEvent?: ImWebSocketCloseEventLike | unknown;
    updatedAt: string;
}
export interface ImLiveErrorContext {
    code: string;
    source: 'realtime' | 'socket';
    error: unknown;
    requestId?: string;
    frame?: ImRealtimeErrorFrame;
}
export interface ImCatchUpBatch {
    items: ImReceiveContext[];
    highestSequence: number;
    rawWindow: RealtimeEventWindow;
}
export interface ImRealtimeSubscriptionGroups {
    conversations?: Array<string | number>;
    rtcSessions?: Array<string | number>;
    items?: RealtimeSubscriptionItemInput[];
}
export interface ImConnectOptions {
    deviceId?: string;
    subscriptions?: ImRealtimeSubscriptionGroups;
    socket?: ImWebSocketLike;
    url?: string;
    headers?: Record<string, string>;
    protocols?: string[];
    requestTimeoutMs?: number;
}
export interface ImLiveMessageStream {
    on(handler: (message: ImDecodedMessage, context: ImMessageContext) => void): ImSubscription;
    onConversation(conversationId: string | number, handler: (message: ImDecodedMessage, context: ImMessageContext) => void): ImSubscription;
}
export interface ImLiveDataStream {
    on(handler: (data: ImDecodedDataPayload, context: ImDataContext) => void): ImSubscription;
}
export interface ImLiveSignalStream {
    on(handler: (signal: ImDecodedRtcSignal, context: ImSignalContext) => void): ImSubscription;
    onRtcSession(rtcSessionId: string | number, handler: (signal: ImDecodedRtcSignal, context: ImSignalContext) => void): ImSubscription;
}
export interface ImLiveEventStream {
    on(handler: (context: ImReceiveContext) => void): ImSubscription;
}
export interface ImLiveLifecycleStream {
    onStateChange(handler: (state: ImLiveState) => void): ImSubscription;
    onError(handler: (context: ImLiveErrorContext) => void): ImSubscription;
    getState(): ImLiveState;
}
export interface ImLiveConnection {
    messages: ImLiveMessageStream;
    data: ImLiveDataStream;
    signals: ImLiveSignalStream;
    events: ImLiveEventStream;
    lifecycle: ImLiveLifecycleStream;
    disconnect(code?: number, reason?: string): void;
}
//# sourceMappingURL=types.d.ts.map