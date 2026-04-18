import Foundation

public class ConversationApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Create a conversation
    public func createConversation(body: CreateConversationRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.backendPath("/conversations"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Create an agent dialog conversation
    public func createAgentDialog(body: CreateAgentDialogRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/agent-dialogs"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Create an agent handoff conversation
    public func createAgentHandoff(body: CreateAgentHandoffRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/agent-handoffs"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Create a system channel conversation
    public func createSystemChannel(body: CreateSystemChannelRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/system-channels"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Get projected conversation summary
    public func getConversationSummary(conversationId: String) async throws -> ConversationSummaryView? {
        return try await client.get(ApiPaths.backendPath("/conversations/\(conversationId)"), responseType: ConversationSummaryView.self)
    }

    /// Get current agent handoff state
    public func getAgentHandoffState(conversationId: String) async throws -> AgentHandoffStateView? {
        return try await client.get(ApiPaths.backendPath("/conversations/\(conversationId)/agent-handoff"), responseType: AgentHandoffStateView.self)
    }

    /// Accept an agent handoff
    public func acceptAgentHandoff(conversationId: String) async throws -> AgentHandoffStateView? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/agent-handoff/accept"), body: nil, responseType: AgentHandoffStateView.self)
    }

    /// Resolve an accepted agent handoff
    public func resolveAgentHandoff(conversationId: String) async throws -> AgentHandoffStateView? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/agent-handoff/resolve"), body: nil, responseType: AgentHandoffStateView.self)
    }

    /// Close an agent handoff
    public func closeAgentHandoff(conversationId: String) async throws -> AgentHandoffStateView? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/agent-handoff/close"), body: nil, responseType: AgentHandoffStateView.self)
    }

    /// List members in a conversation
    public func listConversationMembers(conversationId: String) async throws -> ListMembersResponse? {
        return try await client.get(ApiPaths.backendPath("/conversations/\(conversationId)/members"), responseType: ListMembersResponse.self)
    }

    /// Add a member to a conversation
    public func addConversationMember(conversationId: String, body: AddConversationMemberRequest) async throws -> ConversationMember? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/members/add"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationMember.self)
    }

    /// Remove a member from a conversation
    public func removeConversationMember(conversationId: String, body: RemoveConversationMemberRequest) async throws -> ConversationMember? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/members/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationMember.self)
    }

    /// Transfer conversation ownership
    public func transferConversationOwner(conversationId: String, body: TransferConversationOwnerRequest) async throws -> TransferConversationOwnerResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/members/transfer-owner"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: TransferConversationOwnerResult.self)
    }

    /// Change a conversation member role
    public func changeConversationMemberRole(conversationId: String, body: ChangeConversationMemberRoleRequest) async throws -> ChangeConversationMemberRoleResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/members/change-role"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ChangeConversationMemberRoleResult.self)
    }

    /// Leave a conversation
    public func leave(conversationId: String) async throws -> ConversationMember? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/members/leave"), body: nil, responseType: ConversationMember.self)
    }

    /// Get the current member read cursor
    public func getConversationReadCursor(conversationId: String) async throws -> ConversationReadCursorView? {
        return try await client.get(ApiPaths.backendPath("/conversations/\(conversationId)/read-cursor"), responseType: ConversationReadCursorView.self)
    }

    /// Update the current member read cursor
    public func updateConversationReadCursor(conversationId: String, body: UpdateReadCursorRequest) async throws -> ConversationReadCursorView? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/read-cursor"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationReadCursorView.self)
    }

    /// List projected conversation timeline entries
    public func listConversationMessages(conversationId: String) async throws -> TimelineResponse? {
        return try await client.get(ApiPaths.backendPath("/conversations/\(conversationId)/messages"), responseType: TimelineResponse.self)
    }

    /// Post a standard conversation message
    public func postConversationMessage(conversationId: String, body: PostMessageRequest) async throws -> PostMessageResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/messages"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PostMessageResult.self)
    }

    /// Publish a message into a system channel conversation
    public func publishSystemChannelMessage(conversationId: String, body: PostMessageRequest) async throws -> PostMessageResult? {
        return try await client.post(ApiPaths.backendPath("/conversations/\(conversationId)/system-channel/publish"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PostMessageResult.self)
    }
}
