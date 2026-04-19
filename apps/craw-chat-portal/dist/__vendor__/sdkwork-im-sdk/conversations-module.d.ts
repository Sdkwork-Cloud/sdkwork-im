import type { AddConversationMemberRequest, AgentHandoffStateView, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateSystemChannelRequest, ListMembersResponse, PostMessageRequest, PostMessageResult, PostTextMessageOptions, RemoveConversationMemberRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImConversationsModule {
    private readonly context;
    constructor(context: ImSdkContext);
    create(body: CreateConversationRequest): Promise<CreateConversationResult>;
    createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
    createAgentHandoff(body: CreateAgentHandoffRequest): Promise<CreateConversationResult>;
    createSystemChannel(body: CreateSystemChannelRequest): Promise<CreateConversationResult>;
    get(conversationId: string | number): Promise<ConversationSummaryView>;
    getAgentHandoffState(conversationId: string | number): Promise<AgentHandoffStateView>;
    acceptAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
    resolveAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
    closeAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
    listMembers(conversationId: string | number): Promise<ListMembersResponse>;
    addMember(conversationId: string | number, body: AddConversationMemberRequest): Promise<ConversationMember>;
    removeMember(conversationId: string | number, body: RemoveConversationMemberRequest): Promise<ConversationMember>;
    transferOwner(conversationId: string | number, body: TransferConversationOwnerRequest): Promise<TransferConversationOwnerResult>;
    changeMemberRole(conversationId: string | number, body: ChangeConversationMemberRoleRequest): Promise<ChangeConversationMemberRoleResult>;
    leave(conversationId: string | number): Promise<ConversationMember>;
    getReadCursor(conversationId: string | number): Promise<ConversationReadCursorView>;
    updateReadCursor(conversationId: string | number, body: UpdateReadCursorRequest): Promise<ConversationReadCursorView>;
    listMessages(conversationId: string | number): Promise<TimelineResponse>;
    postMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult>;
    postText(conversationId: string | number, text: string, options?: PostTextMessageOptions): Promise<PostMessageResult>;
    publishSystemMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult>;
    publishSystemText(conversationId: string | number, text: string, options?: PostTextMessageOptions): Promise<PostMessageResult>;
}
//# sourceMappingURL=conversations-module.d.ts.map