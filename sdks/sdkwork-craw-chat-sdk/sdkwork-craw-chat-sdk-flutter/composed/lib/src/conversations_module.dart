import 'package:backend_sdk/backend_sdk.dart';

import 'builders.dart';
import 'context.dart';
import 'types.dart';

class CrawChatConversationsModule {
  final CrawChatSdkContext context;

  CrawChatConversationsModule(this.context);

  Future<CreateConversationResult?> create(CreateConversationRequest body) {
    return context.backendClient.conversation.createConversation(body);
  }

  Future<CreateConversationResult?> createAgentDialog(
    CreateAgentDialogRequest body,
  ) {
    return context.backendClient.conversation.createAgentDialog(body);
  }

  Future<CreateConversationResult?> createAgentHandoff(
    CreateAgentHandoffRequest body,
  ) {
    return context.backendClient.conversation.createAgentHandoff(body);
  }

  Future<CreateConversationResult?> createSystemChannel(
    CreateSystemChannelRequest body,
  ) {
    return context.backendClient.conversation.createSystemChannel(body);
  }

  Future<ConversationSummaryView?> get(String conversationId) {
    return context.backendClient.conversation.getConversationSummary(conversationId);
  }

  Future<AgentHandoffStateView?> getAgentHandoffState(String conversationId) {
    return context.backendClient.conversation.getAgentHandoffState(conversationId);
  }

  Future<AgentHandoffStateView?> acceptAgentHandoff(String conversationId) {
    return context.backendClient.conversation.acceptAgentHandoff(conversationId);
  }

  Future<AgentHandoffStateView?> resolveAgentHandoff(String conversationId) {
    return context.backendClient.conversation.resolveAgentHandoff(conversationId);
  }

  Future<AgentHandoffStateView?> closeAgentHandoff(String conversationId) {
    return context.backendClient.conversation.closeAgentHandoff(conversationId);
  }

  Future<ListMembersResponse?> listMembers(String conversationId) {
    return context.backendClient.conversation.listConversationMembers(conversationId);
  }

  Future<ConversationMember?> addMember(
    String conversationId,
    AddConversationMemberRequest body,
  ) {
    return context.backendClient.conversation.addConversationMember(
      conversationId,
      body,
    );
  }

  Future<ConversationMember?> removeMember(
    String conversationId,
    RemoveConversationMemberRequest body,
  ) {
    return context.backendClient.conversation.removeConversationMember(
      conversationId,
      body,
    );
  }

  Future<TransferConversationOwnerResult?> transferOwner(
    String conversationId,
    TransferConversationOwnerRequest body,
  ) {
    return context.backendClient.conversation.transferConversationOwner(
      conversationId,
      body,
    );
  }

  Future<ChangeConversationMemberRoleResult?> changeMemberRole(
    String conversationId,
    ChangeConversationMemberRoleRequest body,
  ) {
    return context.backendClient.conversation.changeConversationMemberRole(
      conversationId,
      body,
    );
  }

  Future<ConversationMember?> leave(String conversationId) {
    return context.backendClient.conversation.leave(conversationId);
  }

  Future<ConversationReadCursorView?> getReadCursor(String conversationId) {
    return context.backendClient.conversation.getConversationReadCursor(
      conversationId,
    );
  }

  Future<ConversationReadCursorView?> updateReadCursor(
    String conversationId,
    UpdateReadCursorRequest body,
  ) {
    return context.backendClient.conversation.updateConversationReadCursor(
      conversationId,
      body,
    );
  }

  Future<TimelineResponse?> listMessages(String conversationId) {
    return context.backendClient.conversation.listConversationMessages(
      conversationId,
    );
  }

  Future<PostMessageResult?> postMessage(
    String conversationId,
    PostMessageRequest body,
  ) {
    return context.backendClient.conversation.postConversationMessage(
      conversationId,
      body,
    );
  }

  Future<PostMessageResult?> postText(
    String conversationId, {
    required String text,
    CrawChatTextMessageOptions options = const CrawChatTextMessageOptions(),
  }) {
    return postMessage(
      conversationId,
      CrawChatBuilders.textMessage(text: text, options: options),
    );
  }

  Future<PostMessageResult?> publishSystemMessage(
    String conversationId,
    PostMessageRequest body,
  ) {
    return context.backendClient.conversation.publishSystemChannelMessage(
      conversationId,
      body,
    );
  }

  Future<PostMessageResult?> publishSystemText(
    String conversationId, {
    required String text,
    CrawChatTextMessageOptions options = const CrawChatTextMessageOptions(),
  }) {
    return publishSystemMessage(
      conversationId,
      CrawChatBuilders.textMessage(text: text, options: options),
    );
  }
}
