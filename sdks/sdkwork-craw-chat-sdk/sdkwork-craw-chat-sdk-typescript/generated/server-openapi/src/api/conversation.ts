import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { AddConversationMemberRequest, AgentHandoffStateView, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateSystemChannelRequest, ListMembersResponse, PostMessageRequest, PostMessageResult, RemoveConversationMemberRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest } from '../types';


export class ConversationApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Create a conversation */
  async createConversation(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(backendApiPath(`/conversations`), body, undefined, undefined, 'application/json');
  }

/** Create an agent dialog conversation */
  async createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(backendApiPath(`/conversations/agent-dialogs`), body, undefined, undefined, 'application/json');
  }

/** Create an agent handoff conversation */
  async createAgentHandoff(body: CreateAgentHandoffRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(backendApiPath(`/conversations/agent-handoffs`), body, undefined, undefined, 'application/json');
  }

/** Create a system channel conversation */
  async createSystemChannel(body: CreateSystemChannelRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(backendApiPath(`/conversations/system-channels`), body, undefined, undefined, 'application/json');
  }

/** Get projected conversation summary */
  async getConversationSummary(conversationId: string | number): Promise<ConversationSummaryView> {
    return this.client.get<ConversationSummaryView>(backendApiPath(`/conversations/${conversationId}`));
  }

/** Get current agent handoff state */
  async getAgentHandoffState(conversationId: string | number): Promise<AgentHandoffStateView> {
    return this.client.get<AgentHandoffStateView>(backendApiPath(`/conversations/${conversationId}/agent-handoff`));
  }

/** Accept an agent handoff */
  async acceptAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView> {
    return this.client.post<AgentHandoffStateView>(backendApiPath(`/conversations/${conversationId}/agent-handoff/accept`));
  }

/** Resolve an accepted agent handoff */
  async resolveAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView> {
    return this.client.post<AgentHandoffStateView>(backendApiPath(`/conversations/${conversationId}/agent-handoff/resolve`));
  }

/** Close an agent handoff */
  async closeAgentHandoff(conversationId: string | number): Promise<AgentHandoffStateView> {
    return this.client.post<AgentHandoffStateView>(backendApiPath(`/conversations/${conversationId}/agent-handoff/close`));
  }

/** List members in a conversation */
  async listConversationMembers(conversationId: string | number): Promise<ListMembersResponse> {
    return this.client.get<ListMembersResponse>(backendApiPath(`/conversations/${conversationId}/members`));
  }

/** Add a member to a conversation */
  async addConversationMember(conversationId: string | number, body: AddConversationMemberRequest): Promise<ConversationMember> {
    return this.client.post<ConversationMember>(backendApiPath(`/conversations/${conversationId}/members/add`), body, undefined, undefined, 'application/json');
  }

/** Remove a member from a conversation */
  async removeConversationMember(conversationId: string | number, body: RemoveConversationMemberRequest): Promise<ConversationMember> {
    return this.client.post<ConversationMember>(backendApiPath(`/conversations/${conversationId}/members/remove`), body, undefined, undefined, 'application/json');
  }

/** Transfer conversation ownership */
  async transferConversationOwner(conversationId: string | number, body: TransferConversationOwnerRequest): Promise<TransferConversationOwnerResult> {
    return this.client.post<TransferConversationOwnerResult>(backendApiPath(`/conversations/${conversationId}/members/transfer-owner`), body, undefined, undefined, 'application/json');
  }

/** Change a conversation member role */
  async changeConversationMemberRole(conversationId: string | number, body: ChangeConversationMemberRoleRequest): Promise<ChangeConversationMemberRoleResult> {
    return this.client.post<ChangeConversationMemberRoleResult>(backendApiPath(`/conversations/${conversationId}/members/change-role`), body, undefined, undefined, 'application/json');
  }

/** Leave a conversation */
  async leave(conversationId: string | number): Promise<ConversationMember> {
    return this.client.post<ConversationMember>(backendApiPath(`/conversations/${conversationId}/members/leave`));
  }

/** Get the current member read cursor */
  async getConversationReadCursor(conversationId: string | number): Promise<ConversationReadCursorView> {
    return this.client.get<ConversationReadCursorView>(backendApiPath(`/conversations/${conversationId}/read-cursor`));
  }

/** Update the current member read cursor */
  async updateConversationReadCursor(conversationId: string | number, body: UpdateReadCursorRequest): Promise<ConversationReadCursorView> {
    return this.client.post<ConversationReadCursorView>(backendApiPath(`/conversations/${conversationId}/read-cursor`), body, undefined, undefined, 'application/json');
  }

/** List projected conversation timeline entries */
  async listConversationMessages(conversationId: string | number): Promise<TimelineResponse> {
    return this.client.get<TimelineResponse>(backendApiPath(`/conversations/${conversationId}/messages`));
  }

/** Post a standard conversation message */
  async postConversationMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult> {
    return this.client.post<PostMessageResult>(backendApiPath(`/conversations/${conversationId}/messages`), body, undefined, undefined, 'application/json');
  }

/** Publish a message into a system channel conversation */
  async publishSystemChannelMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostMessageResult> {
    return this.client.post<PostMessageResult>(backendApiPath(`/conversations/${conversationId}/system-channel/publish`), body, undefined, undefined, 'application/json');
  }
}

export function createConversationApi(client: HttpClient): ConversationApi {
  return new ConversationApi(client);
}
