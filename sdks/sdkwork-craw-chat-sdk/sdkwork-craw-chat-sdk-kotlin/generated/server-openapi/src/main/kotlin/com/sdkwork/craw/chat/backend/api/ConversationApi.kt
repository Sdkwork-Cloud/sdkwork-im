package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class ConversationApi(private val client: HttpClient) {

    /** Create a conversation */
    suspend fun createConversation(body: CreateConversationRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Create an agent dialog conversation */
    suspend fun createAgentDialog(body: CreateAgentDialogRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/agent-dialogs"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Create an agent handoff conversation */
    suspend fun createAgentHandoff(body: CreateAgentHandoffRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/agent-handoffs"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Create a system channel conversation */
    suspend fun createSystemChannel(body: CreateSystemChannelRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/system-channels"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Get projected conversation summary */
    suspend fun getConversationSummary(conversationId: String): ConversationSummaryView? {
        val raw = client.get(ApiPaths.backendPath("/conversations/$conversationId"))
        return client.convertValue(raw, object : TypeReference<ConversationSummaryView>() {})
    }

    /** Get current agent handoff state */
    suspend fun getAgentHandoffState(conversationId: String): AgentHandoffStateView? {
        val raw = client.get(ApiPaths.backendPath("/conversations/$conversationId/agent-handoff"))
        return client.convertValue(raw, object : TypeReference<AgentHandoffStateView>() {})
    }

    /** Accept an agent handoff */
    suspend fun acceptAgentHandoff(conversationId: String): AgentHandoffStateView? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/agent-handoff/accept"), null)
        return client.convertValue(raw, object : TypeReference<AgentHandoffStateView>() {})
    }

    /** Resolve an accepted agent handoff */
    suspend fun resolveAgentHandoff(conversationId: String): AgentHandoffStateView? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/agent-handoff/resolve"), null)
        return client.convertValue(raw, object : TypeReference<AgentHandoffStateView>() {})
    }

    /** Close an agent handoff */
    suspend fun closeAgentHandoff(conversationId: String): AgentHandoffStateView? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/agent-handoff/close"), null)
        return client.convertValue(raw, object : TypeReference<AgentHandoffStateView>() {})
    }

    /** List members in a conversation */
    suspend fun listConversationMembers(conversationId: String): ListMembersResponse? {
        val raw = client.get(ApiPaths.backendPath("/conversations/$conversationId/members"))
        return client.convertValue(raw, object : TypeReference<ListMembersResponse>() {})
    }

    /** Add a member to a conversation */
    suspend fun addConversationMember(conversationId: String, body: AddConversationMemberRequest): ConversationMember? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/members/add"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationMember>() {})
    }

    /** Remove a member from a conversation */
    suspend fun removeConversationMember(conversationId: String, body: RemoveConversationMemberRequest): ConversationMember? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/members/remove"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationMember>() {})
    }

    /** Transfer conversation ownership */
    suspend fun transferConversationOwner(conversationId: String, body: TransferConversationOwnerRequest): TransferConversationOwnerResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/members/transfer-owner"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<TransferConversationOwnerResult>() {})
    }

    /** Change a conversation member role */
    suspend fun changeConversationMemberRole(conversationId: String, body: ChangeConversationMemberRoleRequest): ChangeConversationMemberRoleResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/members/change-role"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ChangeConversationMemberRoleResult>() {})
    }

    /** Leave a conversation */
    suspend fun leave(conversationId: String): ConversationMember? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/members/leave"), null)
        return client.convertValue(raw, object : TypeReference<ConversationMember>() {})
    }

    /** Get the current member read cursor */
    suspend fun getConversationReadCursor(conversationId: String): ConversationReadCursorView? {
        val raw = client.get(ApiPaths.backendPath("/conversations/$conversationId/read-cursor"))
        return client.convertValue(raw, object : TypeReference<ConversationReadCursorView>() {})
    }

    /** Update the current member read cursor */
    suspend fun updateConversationReadCursor(conversationId: String, body: UpdateReadCursorRequest): ConversationReadCursorView? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/read-cursor"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationReadCursorView>() {})
    }

    /** List projected conversation timeline entries */
    suspend fun listConversationMessages(conversationId: String): TimelineResponse? {
        val raw = client.get(ApiPaths.backendPath("/conversations/$conversationId/messages"))
        return client.convertValue(raw, object : TypeReference<TimelineResponse>() {})
    }

    /** Post a standard conversation message */
    suspend fun postConversationMessage(conversationId: String, body: PostMessageRequest): PostMessageResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/messages"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PostMessageResult>() {})
    }

    /** Publish a message into a system channel conversation */
    suspend fun publishSystemChannelMessage(conversationId: String, body: PostMessageRequest): PostMessageResult? {
        val raw = client.post(ApiPaths.backendPath("/conversations/$conversationId/system-channel/publish"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PostMessageResult>() {})
    }
}
