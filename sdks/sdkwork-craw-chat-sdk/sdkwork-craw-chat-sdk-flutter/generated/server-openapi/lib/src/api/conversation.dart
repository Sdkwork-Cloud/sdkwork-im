import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class ConversationApi {
  final HttpClient _client;

  ConversationApi(this._client);

  /// Create a conversation
  Future<CreateConversationResult?> createConversation(CreateConversationRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CreateConversationResult.fromJson(map);
    })();
  }

  /// Create an agent dialog conversation
  Future<CreateConversationResult?> createAgentDialog(CreateAgentDialogRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/agent-dialogs'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CreateConversationResult.fromJson(map);
    })();
  }

  /// Create an agent handoff conversation
  Future<CreateConversationResult?> createAgentHandoff(CreateAgentHandoffRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/agent-handoffs'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CreateConversationResult.fromJson(map);
    })();
  }

  /// Create a system channel conversation
  Future<CreateConversationResult?> createSystemChannel(CreateSystemChannelRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/system-channels'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CreateConversationResult.fromJson(map);
    })();
  }

  /// Get projected conversation summary
  Future<ConversationSummaryView?> getConversationSummary(String conversationId) async {
    final response = await _client.get(ApiPaths.backendPath('/conversations/$conversationId'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationSummaryView.fromJson(map);
    })();
  }

  /// Get current agent handoff state
  Future<AgentHandoffStateView?> getAgentHandoffState(String conversationId) async {
    final response = await _client.get(ApiPaths.backendPath('/conversations/$conversationId/agent-handoff'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AgentHandoffStateView.fromJson(map);
    })();
  }

  /// Accept an agent handoff
  Future<AgentHandoffStateView?> acceptAgentHandoff(String conversationId) async {
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/agent-handoff/accept'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AgentHandoffStateView.fromJson(map);
    })();
  }

  /// Resolve an accepted agent handoff
  Future<AgentHandoffStateView?> resolveAgentHandoff(String conversationId) async {
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/agent-handoff/resolve'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AgentHandoffStateView.fromJson(map);
    })();
  }

  /// Close an agent handoff
  Future<AgentHandoffStateView?> closeAgentHandoff(String conversationId) async {
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/agent-handoff/close'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : AgentHandoffStateView.fromJson(map);
    })();
  }

  /// List members in a conversation
  Future<ListMembersResponse?> listConversationMembers(String conversationId) async {
    final response = await _client.get(ApiPaths.backendPath('/conversations/$conversationId/members'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ListMembersResponse.fromJson(map);
    })();
  }

  /// Add a member to a conversation
  Future<ConversationMember?> addConversationMember(String conversationId, AddConversationMemberRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/members/add'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationMember.fromJson(map);
    })();
  }

  /// Remove a member from a conversation
  Future<ConversationMember?> removeConversationMember(String conversationId, RemoveConversationMemberRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/members/remove'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationMember.fromJson(map);
    })();
  }

  /// Transfer conversation ownership
  Future<TransferConversationOwnerResult?> transferConversationOwner(String conversationId, TransferConversationOwnerRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/members/transfer-owner'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : TransferConversationOwnerResult.fromJson(map);
    })();
  }

  /// Change a conversation member role
  Future<ChangeConversationMemberRoleResult?> changeConversationMemberRole(String conversationId, ChangeConversationMemberRoleRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/members/change-role'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ChangeConversationMemberRoleResult.fromJson(map);
    })();
  }

  /// Leave a conversation
  Future<ConversationMember?> leave(String conversationId) async {
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/members/leave'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationMember.fromJson(map);
    })();
  }

  /// Get the current member read cursor
  Future<ConversationReadCursorView?> getConversationReadCursor(String conversationId) async {
    final response = await _client.get(ApiPaths.backendPath('/conversations/$conversationId/read-cursor'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationReadCursorView.fromJson(map);
    })();
  }

  /// Update the current member read cursor
  Future<ConversationReadCursorView?> updateConversationReadCursor(String conversationId, UpdateReadCursorRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/read-cursor'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ConversationReadCursorView.fromJson(map);
    })();
  }

  /// List projected conversation timeline entries
  Future<TimelineResponse?> listConversationMessages(String conversationId) async {
    final response = await _client.get(ApiPaths.backendPath('/conversations/$conversationId/messages'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : TimelineResponse.fromJson(map);
    })();
  }

  /// Post a standard conversation message
  Future<PostMessageResult?> postConversationMessage(String conversationId, PostMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/messages'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PostMessageResult.fromJson(map);
    })();
  }

  /// Publish a message into a system channel conversation
  Future<PostMessageResult?> publishSystemChannelMessage(String conversationId, PostMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/conversations/$conversationId/system-channel/publish'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PostMessageResult.fromJson(map);
    })();
  }
}
