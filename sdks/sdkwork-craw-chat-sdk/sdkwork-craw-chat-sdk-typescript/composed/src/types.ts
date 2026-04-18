import type {
  AckRealtimeEventsRequest,
  AddConversationMemberRequest,
  AgentHandoffStateView,
  AbortStreamRequest,
  AppendStreamFrameRequest,
  AttachMediaRequest,
  ChangeConversationMemberRoleRequest,
  ChangeConversationMemberRoleResult,
  CheckpointStreamRequest,
  CompleteStreamRequest,
  CompleteUploadRequest,
  ConversationMember,
  ConversationReadCursorView,
  ConversationSummaryView,
  CreateAgentDialogRequest,
  CreateAgentHandoffRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateRtcSessionRequest,
  CreateSystemChannelRequest,
  CreateUploadRequest,
  DeviceSyncFeedResponse,
  EditMessageRequest,
  InboxResponse,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  ListMembersResponse,
  MediaAsset,
  MediaDownloadUrlResponse,
  MediaUploadSession,
  MessageMutationResult,
  OpenStreamRequest,
  PostMessageRequest,
  PostMessageResult,
  PostRtcSignalRequest,
  PresenceDeviceRequest,
  PresenceSnapshotView,
  RealtimeAckState,
  RealtimeEventWindow,
  RealtimeSubscriptionSnapshot,
  RegisterDeviceRequest,
  RegisteredDeviceView,
  QueryParams,
  RemoveConversationMemberRequest,
  ResumeSessionRequest,
  RtcParticipantCredential,
  RtcRecordingArtifact,
  RtcSession,
  RtcSignalEvent,
  SdkworkBackendConfig,
  SessionResumeView,
  StringMap,
  StreamFrame,
  StreamFrameWindow,
  StreamSession,
  SyncRealtimeSubscriptionsRequest,
  TimelineResponse,
  TransferConversationOwnerRequest,
  TransferConversationOwnerResult,
  UpdateReadCursorRequest,
  UpdateRtcSessionRequest,
} from './generated-backend-types.js';

export type {
  AckRealtimeEventsRequest,
  AddConversationMemberRequest,
  AgentHandoffStateView,
  AbortStreamRequest,
  AppendStreamFrameRequest,
  AttachMediaRequest,
  ChangeConversationMemberRoleRequest,
  ChangeConversationMemberRoleResult,
  CheckpointStreamRequest,
  CompleteStreamRequest,
  CompleteUploadRequest,
  ConversationMember,
  ConversationReadCursorView,
  ConversationSummaryView,
  CreateAgentDialogRequest,
  CreateAgentHandoffRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateRtcSessionRequest,
  CreateSystemChannelRequest,
  CreateUploadRequest,
  DeviceSyncFeedResponse,
  EditMessageRequest,
  InboxResponse,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  ListMembersResponse,
  MediaAsset,
  MediaDownloadUrlResponse,
  MediaUploadSession,
  MessageMutationResult,
  OpenStreamRequest,
  PostMessageRequest,
  PostMessageResult,
  PostRtcSignalRequest,
  PresenceDeviceRequest,
  PresenceSnapshotView,
  RealtimeAckState,
  RealtimeEventWindow,
  RealtimeSubscriptionSnapshot,
  RegisterDeviceRequest,
  RegisteredDeviceView,
  QueryParams,
  RemoveConversationMemberRequest,
  ResumeSessionRequest,
  RtcParticipantCredential,
  RtcRecordingArtifact,
  RtcSession,
  RtcSignalEvent,
  SdkworkBackendConfig,
  SessionResumeView,
  StringMap,
  StreamFrame,
  StreamFrameWindow,
  StreamSession,
  SyncRealtimeSubscriptionsRequest,
  TimelineResponse,
  TransferConversationOwnerRequest,
  TransferConversationOwnerResult,
  UpdateReadCursorRequest,
  UpdateRtcSessionRequest,
} from './generated-backend-types.js';

export interface CrawChatBackendClientLike {
  session: {
    resume(body: ResumeSessionRequest): Promise<SessionResumeView>;
    disconnect(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
  };
  presence: {
    heartbeat(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
    getPresenceMe(): Promise<PresenceSnapshotView>;
  };
  realtime: {
    syncRealtimeSubscriptions(
      body: SyncRealtimeSubscriptionsRequest,
    ): Promise<RealtimeSubscriptionSnapshot>;
    listRealtimeEvents(params?: QueryParams): Promise<RealtimeEventWindow>;
    ackRealtimeEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState>;
  };
  device: {
    register(body: RegisterDeviceRequest): Promise<RegisteredDeviceView>;
    getDeviceSyncFeed(
      deviceId: string | number,
      params?: QueryParams,
    ): Promise<DeviceSyncFeedResponse>;
  };
  inbox: {
    getInbox(): Promise<InboxResponse>;
  };
  conversation: {
    createConversation(body: CreateConversationRequest): Promise<CreateConversationResult>;
    createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
    createAgentHandoff(body: CreateAgentHandoffRequest): Promise<CreateConversationResult>;
    createSystemChannel(body: CreateSystemChannelRequest): Promise<CreateConversationResult>;
    getConversationSummary(
      conversationId: string | number,
    ): Promise<ConversationSummaryView>;
    getAgentHandoffState(
      conversationId: string | number,
    ): Promise<AgentHandoffStateView>;
    acceptAgentHandoff(
      conversationId: string | number,
    ): Promise<AgentHandoffStateView>;
    resolveAgentHandoff(
      conversationId: string | number,
    ): Promise<AgentHandoffStateView>;
    closeAgentHandoff(
      conversationId: string | number,
    ): Promise<AgentHandoffStateView>;
    listConversationMembers(
      conversationId: string | number,
    ): Promise<ListMembersResponse>;
    addConversationMember(
      conversationId: string | number,
      body: AddConversationMemberRequest,
    ): Promise<ConversationMember>;
    removeConversationMember(
      conversationId: string | number,
      body: RemoveConversationMemberRequest,
    ): Promise<ConversationMember>;
    transferConversationOwner(
      conversationId: string | number,
      body: TransferConversationOwnerRequest,
    ): Promise<TransferConversationOwnerResult>;
    changeConversationMemberRole(
      conversationId: string | number,
      body: ChangeConversationMemberRoleRequest,
    ): Promise<ChangeConversationMemberRoleResult>;
    leave(conversationId: string | number): Promise<ConversationMember>;
    getConversationReadCursor(
      conversationId: string | number,
    ): Promise<ConversationReadCursorView>;
    updateConversationReadCursor(
      conversationId: string | number,
      body: UpdateReadCursorRequest,
    ): Promise<ConversationReadCursorView>;
    listConversationMessages(
      conversationId: string | number,
    ): Promise<TimelineResponse>;
    postConversationMessage(
      conversationId: string | number,
      body: PostMessageRequest,
    ): Promise<PostMessageResult>;
    publishSystemChannelMessage(
      conversationId: string | number,
      body: PostMessageRequest,
    ): Promise<PostMessageResult>;
  };
  message: {
    edit(
      messageId: string | number,
      body: EditMessageRequest,
    ): Promise<MessageMutationResult>;
    recall(messageId: string | number): Promise<MessageMutationResult>;
  };
  media: {
    createMediaUpload(body: CreateUploadRequest): Promise<MediaUploadMutationResponse>;
    completeMediaUpload(
      mediaAssetId: string | number,
      body: CompleteUploadRequest,
    ): Promise<MediaUploadMutationResponse>;
    getMediaDownloadUrl(
      mediaAssetId: string | number,
      params?: QueryParams,
    ): Promise<MediaDownloadUrlResponse>;
    getMediaAsset(mediaAssetId: string | number): Promise<MediaAsset>;
    attachMediaAsset(
      mediaAssetId: string | number,
      body: AttachMediaRequest,
    ): Promise<PostMessageResult>;
  };
  stream: {
    open(body: OpenStreamRequest): Promise<StreamSession>;
    listStreamFrames(
      streamId: string | number,
      params?: QueryParams,
    ): Promise<StreamFrameWindow>;
    appendStreamFrame(
      streamId: string | number,
      body: AppendStreamFrameRequest,
    ): Promise<StreamFrame>;
    checkpoint(
      streamId: string | number,
      body: CheckpointStreamRequest,
    ): Promise<StreamSession>;
    complete(
      streamId: string | number,
      body: CompleteStreamRequest,
    ): Promise<StreamSession>;
    abort(
      streamId: string | number,
      body: AbortStreamRequest,
    ): Promise<StreamSession>;
  };
  rtc: {
    createRtcSession(body: CreateRtcSessionRequest): Promise<RtcSession>;
    inviteRtcSession(
      rtcSessionId: string | number,
      body: InviteRtcSessionRequest,
    ): Promise<RtcSession>;
    acceptRtcSession(
      rtcSessionId: string | number,
      body: UpdateRtcSessionRequest,
    ): Promise<RtcSession>;
    rejectRtcSession(
      rtcSessionId: string | number,
      body: UpdateRtcSessionRequest,
    ): Promise<RtcSession>;
    endRtcSession(
      rtcSessionId: string | number,
      body: UpdateRtcSessionRequest,
    ): Promise<RtcSession>;
    postRtcSignal(
      rtcSessionId: string | number,
      body: PostRtcSignalRequest,
    ): Promise<RtcSignalEvent>;
    issueRtcParticipantCredential(
      rtcSessionId: string | number,
      body: IssueRtcParticipantCredentialRequest,
    ): Promise<RtcParticipantCredential>;
    getRtcRecordingArtifact(
      rtcSessionId: string | number,
    ): Promise<RtcRecordingArtifact>;
  };
  setAuthToken?(token: string): unknown;
}

export interface CrawChatSdkClientOptions {
  backendClient: CrawChatBackendClientLike;
}

export interface CrawChatSdkClientCreateOptions {
  backendClient?: CrawChatBackendClientLike;
  baseUrl?: string;
  authToken?: string;
  tokenManager?: SdkworkBackendConfig['tokenManager'];
  timeout?: number;
  headers?: Record<string, string>;
}

export interface PostTextMessageOptions extends Omit<PostMessageRequest, 'text'> {}

export interface EditTextMessageOptions extends Omit<EditMessageRequest, 'text'> {}

export interface AttachTextMediaOptions extends Omit<AttachMediaRequest, 'text'> {
  text: string;
}

export type MediaUploadDeliveryStatus = 'applied' | 'replayed';

export interface MediaUploadMutationResponse extends MediaAsset {
  upload?: MediaUploadSession;
  requestKey: string;
  deliveryStatus: MediaUploadDeliveryStatus;
  proofVersion: string;
}

export type CrawChatUploadBody = ArrayBuffer | ArrayBufferView | Blob | string;

export interface CrawChatUploadFetchResponseLike {
  ok: boolean;
  status: number;
  text(): Promise<string>;
}

export interface CrawChatUploadFetchLike {
  (
    input: string,
    init: {
      method?: string;
      headers?: Record<string, string>;
      body?: CrawChatUploadBody;
    },
  ): Promise<CrawChatUploadFetchResponseLike>;
}

export interface CrawChatMediaUploadOptions {
  checksum?: string;
  fetch?: CrawChatUploadFetchLike;
}

export interface AppendTextFrameOptions
  extends Omit<AppendStreamFrameRequest, 'frameType' | 'encoding' | 'payload'> {
  text: string;
  encoding?: string;
}

export interface PostJsonRtcSignalOptions
  extends Omit<PostRtcSignalRequest, 'signalType' | 'payload'> {
  payload: unknown;
  pretty?: boolean;
}
