import type { AckRealtimeEventsRequest, AddConversationMemberRequest, AgentHandoffStateView, AbortStreamRequest, AppendStreamFrameRequest, AttachMediaRequest, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, CheckpointStreamRequest, CompleteStreamRequest, CompleteUploadRequest, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateRtcSessionRequest, CreateSystemChannelRequest, CreateUploadRequest, DeviceSyncFeedResponse, EditMessageRequest, InboxResponse, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest, ListMembersResponse, MediaAsset, MediaDownloadUrlResponse, MediaUploadSessionResponse, MessageMutationResult, PostMessageRequest, PostMessageResult, PostRtcSignalRequest, PresenceDeviceRequest, PresenceSnapshotView, OpenStreamRequest, QueryParams, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, RegisterDeviceRequest, RegisteredDeviceView, RemoveConversationMemberRequest, ResumeSessionRequest, RtcParticipantCredential, RtcRecordingArtifact, RtcSession, RtcSignalEvent, SessionResumeView, StreamFrame, StreamFrameWindow, StreamSession, SyncRealtimeSubscriptionsRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest, UpdateRtcSessionRequest } from './generated-client-types.js';
import type { ImAppSnapshot, ImAuthLoginRequest, ImAuthLoginResult, ImAuthSession, ImWorkspace } from './types.js';
export interface ImTransportClientLike {
    auth: {
        login(body: ImAuthLoginRequest): Promise<ImAuthLoginResult>;
        me(): Promise<ImAuthSession>;
    };
    portal: {
        getHome(): Promise<ImAppSnapshot>;
        getAuth(): Promise<ImAppSnapshot>;
        getWorkspace(): Promise<ImWorkspace>;
        getDashboard(): Promise<ImAppSnapshot>;
        getConversations(): Promise<ImAppSnapshot>;
        getRealtime(): Promise<ImAppSnapshot>;
        getMedia(): Promise<ImAppSnapshot>;
        getAutomation(): Promise<ImAppSnapshot>;
        getGovernance(): Promise<ImAppSnapshot>;
    };
    session: {
        resume(body: ResumeSessionRequest): Promise<SessionResumeView>;
        disconnect(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
    };
    presence: {
        heartbeat(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
        getPresenceMe(): Promise<PresenceSnapshotView>;
    };
    realtime: {
        syncRealtimeSubscriptions(body: SyncRealtimeSubscriptionsRequest): Promise<RealtimeSubscriptionSnapshot>;
        listRealtimeEvents(params?: QueryParams): Promise<RealtimeEventWindow>;
        ackRealtimeEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState>;
    };
    device: {
        register(body: RegisterDeviceRequest): Promise<RegisteredDeviceView>;
        getDeviceSyncFeed(deviceId: string | number, params?: QueryParams): Promise<DeviceSyncFeedResponse>;
    };
    inbox: {
        getInbox(): Promise<InboxResponse>;
    };
    conversation: {
        createConversation(body: CreateConversationRequest): Promise<CreateConversationResult>;
        createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
        createAgentHandoff(body: CreateAgentHandoffRequest): Promise<CreateConversationResult>;
        createSystemChannel(body: CreateSystemChannelRequest): Promise<CreateConversationResult>;
        getConversationSummary(conversationId: string | number): Promise<ConversationSummaryView>;
        getAgentHandoffState(conversationId: string | number): Promise<AgentHandoffStateView>;
        acceptAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
        resolveAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
        closeAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
        listConversationMembers(conversationId: string | number): Promise<ListMembersResponse>;
        addConversationMember(conversationId: string | number, body: AddConversationMemberRequest): Promise<ConversationMember>;
        removeConversationMember(conversationId: string | number, body: RemoveConversationMemberRequest): Promise<ConversationMember>;
        transferConversationOwner(conversationId: string | number, body: TransferConversationOwnerRequest): Promise<TransferConversationOwnerResult>;
        changeConversationMemberRole(conversationId: string | number, body: ChangeConversationMemberRoleRequest): Promise<ChangeConversationMemberRoleResult>;
        leave(conversationId: string | number): Promise<ConversationMember>;
        getConversationReadCursor(conversationId: string | number): Promise<ConversationReadCursorView>;
        updateConversationReadCursor(conversationId: string | number, body: UpdateReadCursorRequest): Promise<ConversationReadCursorView>;
        listConversationMessages(conversationId: string | number): Promise<TimelineResponse>;
        postConversationMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult>;
        publishSystemChannelMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult>;
    };
    message: {
        edit(messageId: string | number, body: EditMessageRequest): Promise<MessageMutationResult>;
        recall(messageId: string | number): Promise<MessageMutationResult>;
    };
    media: {
        createMediaUpload(body: CreateUploadRequest): Promise<MediaUploadSessionResponse>;
        completeMediaUpload(mediaAssetId: string | number, body: CompleteUploadRequest): Promise<MediaAsset>;
        getMediaDownloadUrl(mediaAssetId: string | number, params?: QueryParams): Promise<MediaDownloadUrlResponse>;
        getMediaAsset(mediaAssetId: string | number): Promise<MediaAsset>;
        attachMediaAsset(mediaAssetId: string | number, body: AttachMediaRequest): Promise<PostMessageResult>;
    };
    stream: {
        open(body: OpenStreamRequest): Promise<StreamSession>;
        listStreamFrames(streamId: string | number, params?: QueryParams): Promise<StreamFrameWindow>;
        appendStreamFrame(streamId: string | number, body: AppendStreamFrameRequest): Promise<StreamFrame>;
        checkpoint(streamId: string | number, body: CheckpointStreamRequest): Promise<StreamSession>;
        complete(streamId: string | number, body: CompleteStreamRequest): Promise<StreamSession>;
        abort(streamId: string | number, body: AbortStreamRequest): Promise<StreamSession>;
    };
    rtc: {
        createRtcSession(body: CreateRtcSessionRequest): Promise<RtcSession>;
        inviteRtcSession(rtcSessionId: string | number, body: InviteRtcSessionRequest): Promise<RtcSession>;
        acceptRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
        rejectRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
        endRtcSession(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSession>;
        postRtcSignal(rtcSessionId: string | number, body: PostRtcSignalRequest): Promise<RtcSignalEvent>;
        issueRtcParticipantCredential(rtcSessionId: string | number, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential>;
        getRtcRecordingArtifact(rtcSessionId: string | number): Promise<RtcRecordingArtifact>;
    };
    setAuthToken?(token: string): unknown;
    clearAuthToken?(): unknown;
}
//# sourceMappingURL=transport-client-like.d.ts.map