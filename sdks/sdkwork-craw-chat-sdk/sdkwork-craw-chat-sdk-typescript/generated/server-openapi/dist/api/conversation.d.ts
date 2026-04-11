import type { HttpClient } from '../http/client';
import type { AddConversationMemberRequest, AgentHandoffStateView, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateSystemChannelRequest, ListMembersResponse, PostMessageRequest, PostMessageResult, RemoveConversationMemberRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest } from '../types';
export declare class ConversationApi {
    private client;
    constructor(client: HttpClient);
    /** Create a conversation */
    createConversation(body: CreateConversationRequest): Promise<CreateConversationResult>;
    /** Create an agent dialog conversation */
    createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
    /** Create an agent handoff conversation */
    createAgentHandoff(body: CreateAgentHandoffRequest): Promise<CreateConversationResult>;
    /** Create a system channel conversation */
    createSystemChannel(body: CreateSystemChannelRequest): Promise<CreateConversationResult>;
    /** Get projected conversation summary */
    getConversationSummary(conversationId: string | number): Promise<ConversationSummaryView>;
    /** Get current agent handoff state */
    getAgentHandoffState(conversationId: string | number): Promise<AgentHandoffStateView>;
    /** Accept an agent handoff */
    acceptAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
    /** Resolve an accepted agent handoff */
    resolveAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
    /** Close an agent handoff */
    closeAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView>;
    /** List members in a conversation */
    listConversationMembers(conversationId: string | number): Promise<ListMembersResponse>;
    /** Add a member to a conversation */
    addConversationMember(conversationId: string | number, body: AddConversationMemberRequest): Promise<ConversationMember>;
    /** Remove a member from a conversation */
    removeConversationMember(conversationId: string | number, body: RemoveConversationMemberRequest): Promise<ConversationMember>;
    /** Transfer conversation ownership */
    transferConversationOwner(conversationId: string | number, body: TransferConversationOwnerRequest): Promise<TransferConversationOwnerResult>;
    /** Change a conversation member role */
    changeConversationMemberRole(conversationId: string | number, body: ChangeConversationMemberRoleRequest): Promise<ChangeConversationMemberRoleResult>;
    /** Leave a conversation */
    leave(conversationId: string | number): Promise<ConversationMember>;
    /** Get the current member read cursor */
    getConversationReadCursor(conversationId: string | number): Promise<ConversationReadCursorView>;
    /** Update the current member read cursor */
    updateConversationReadCursor(conversationId: string | number, body: UpdateReadCursorRequest): Promise<ConversationReadCursorView>;
    /** List projected conversation timeline entries */
    listConversationMessages(conversationId: string | number): Promise<TimelineResponse>;
    /** Post a standard conversation message */
    postConversationMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult>;
    /** Publish a message into a system channel conversation */
    publishSystemChannelMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult>;
}
export declare function createConversationApi(client: HttpClient): ConversationApi;
//# sourceMappingURL=conversation.d.ts.map