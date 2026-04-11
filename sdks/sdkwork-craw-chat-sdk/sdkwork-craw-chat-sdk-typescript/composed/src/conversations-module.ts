import { buildTextMessageRequest } from './builders.js';
import type {
  AddConversationMemberRequest,
  AgentHandoffStateView,
  ChangeConversationMemberRoleRequest,
  ChangeConversationMemberRoleResult,
  ConversationMember,
  ConversationReadCursorView,
  ConversationSummaryView,
  CreateAgentDialogRequest,
  CreateAgentHandoffRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateSystemChannelRequest,
  ListMembersResponse,
  PostMessageRequest,
  PostMessageResult,
  PostTextMessageOptions,
  RemoveConversationMemberRequest,
  TimelineResponse,
  TransferConversationOwnerRequest,
  TransferConversationOwnerResult,
  UpdateReadCursorRequest,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatConversationsModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  create(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.context.backendClient.conversation.createConversation(body);
  }

  createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult> {
    return this.context.backendClient.conversation.createAgentDialog(body);
  }

  createAgentHandoff(
    body: CreateAgentHandoffRequest,
  ): Promise<CreateConversationResult> {
    return this.context.backendClient.conversation.createAgentHandoff(body);
  }

  createSystemChannel(
    body: CreateSystemChannelRequest,
  ): Promise<CreateConversationResult> {
    return this.context.backendClient.conversation.createSystemChannel(body);
  }

  get(conversationId: string | number): Promise<ConversationSummaryView> {
    return this.context.backendClient.conversation.getConversationSummary(
      conversationId,
    );
  }

  getAgentHandoffState(
    conversationId: string | number,
  ): Promise<AgentHandoffStateView> {
    return this.context.backendClient.conversation.getAgentHandoffState(
      conversationId,
    );
  }

  acceptAgentHandoff(
    conversationId: string | number,
  ): Promise<AgentHandoffStateView> {
    return this.context.backendClient.conversation.acceptAgentHandoff(
      conversationId,
    );
  }

  resolveAgentHandoff(
    conversationId: string | number,
  ): Promise<AgentHandoffStateView> {
    return this.context.backendClient.conversation.resolveAgentHandoff(
      conversationId,
    );
  }

  closeAgentHandoff(
    conversationId: string | number,
  ): Promise<AgentHandoffStateView> {
    return this.context.backendClient.conversation.closeAgentHandoff(
      conversationId,
    );
  }

  listMembers(conversationId: string | number): Promise<ListMembersResponse> {
    return this.context.backendClient.conversation.listConversationMembers(
      conversationId,
    );
  }

  addMember(
    conversationId: string | number,
    body: AddConversationMemberRequest,
  ): Promise<ConversationMember> {
    return this.context.backendClient.conversation.addConversationMember(
      conversationId,
      body,
    );
  }

  removeMember(
    conversationId: string | number,
    body: RemoveConversationMemberRequest,
  ): Promise<ConversationMember> {
    return this.context.backendClient.conversation.removeConversationMember(
      conversationId,
      body,
    );
  }

  transferOwner(
    conversationId: string | number,
    body: TransferConversationOwnerRequest,
  ): Promise<TransferConversationOwnerResult> {
    return this.context.backendClient.conversation.transferConversationOwner(
      conversationId,
      body,
    );
  }

  changeMemberRole(
    conversationId: string | number,
    body: ChangeConversationMemberRoleRequest,
  ): Promise<ChangeConversationMemberRoleResult> {
    return this.context.backendClient.conversation.changeConversationMemberRole(
      conversationId,
      body,
    );
  }

  leave(conversationId: string | number): Promise<ConversationMember> {
    return this.context.backendClient.conversation.leave(conversationId);
  }

  getReadCursor(
    conversationId: string | number,
  ): Promise<ConversationReadCursorView> {
    return this.context.backendClient.conversation.getConversationReadCursor(
      conversationId,
    );
  }

  updateReadCursor(
    conversationId: string | number,
    body: UpdateReadCursorRequest,
  ): Promise<ConversationReadCursorView> {
    return this.context.backendClient.conversation.updateConversationReadCursor(
      conversationId,
      body,
    );
  }

  listMessages(conversationId: string | number): Promise<TimelineResponse> {
    return this.context.backendClient.conversation.listConversationMessages(
      conversationId,
    );
  }

  postMessage(
    conversationId: string | number,
    body: PostMessageRequest,
  ): Promise<PostMessageResult> {
    return this.context.backendClient.conversation.postConversationMessage(
      conversationId,
      body,
    );
  }

  postText(
    conversationId: string | number,
    text: string,
    options: PostTextMessageOptions = {},
  ): Promise<PostMessageResult> {
    return this.postMessage(conversationId, buildTextMessageRequest(text, options));
  }

  publishSystemMessage(
    conversationId: string | number,
    body: PostMessageRequest,
  ): Promise<PostMessageResult> {
    return this.context.backendClient.conversation.publishSystemChannelMessage(
      conversationId,
      body,
    );
  }

  publishSystemText(
    conversationId: string | number,
    text: string,
    options: PostTextMessageOptions = {},
  ): Promise<PostMessageResult> {
    return this.publishSystemMessage(
      conversationId,
      buildTextMessageRequest(text, options),
    );
  }
}
