package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class ConversationApi {
    private final HttpClient client;
    
    public ConversationApi(HttpClient client) {
        this.client = client;
    }

    /** Create a conversation */
    public CreateConversationResult createConversation(CreateConversationRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Create an agent dialog conversation */
    public CreateConversationResult createAgentDialog(CreateAgentDialogRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/agent-dialogs"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Create an agent handoff conversation */
    public CreateConversationResult createAgentHandoff(CreateAgentHandoffRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/agent-handoffs"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Create a system channel conversation */
    public CreateConversationResult createSystemChannel(CreateSystemChannelRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/system-channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CreateConversationResult>() {});
    }

    /** Get projected conversation summary */
    public ConversationSummaryView getConversationSummary(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/conversations/" + conversationId + ""));
        return client.convertValue(raw, new TypeReference<ConversationSummaryView>() {});
    }

    /** Get current agent handoff state */
    public AgentHandoffStateView getAgentHandoffState(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/conversations/" + conversationId + "/agent-handoff"));
        return client.convertValue(raw, new TypeReference<AgentHandoffStateView>() {});
    }

    /** Accept an agent handoff */
    public AgentHandoffStateView acceptAgentHandoff(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/agent-handoff/accept"), null);
        return client.convertValue(raw, new TypeReference<AgentHandoffStateView>() {});
    }

    /** Resolve an accepted agent handoff */
    public AgentHandoffStateView resolveAgentHandoff(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/agent-handoff/resolve"), null);
        return client.convertValue(raw, new TypeReference<AgentHandoffStateView>() {});
    }

    /** Close an agent handoff */
    public AgentHandoffStateView closeAgentHandoff(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/agent-handoff/close"), null);
        return client.convertValue(raw, new TypeReference<AgentHandoffStateView>() {});
    }

    /** List members in a conversation */
    public ListMembersResponse listConversationMembers(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/conversations/" + conversationId + "/members"));
        return client.convertValue(raw, new TypeReference<ListMembersResponse>() {});
    }

    /** Add a member to a conversation */
    public ConversationMember addConversationMember(String conversationId, AddConversationMemberRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/members/add"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationMember>() {});
    }

    /** Remove a member from a conversation */
    public ConversationMember removeConversationMember(String conversationId, RemoveConversationMemberRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/members/remove"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationMember>() {});
    }

    /** Transfer conversation ownership */
    public TransferConversationOwnerResult transferConversationOwner(String conversationId, TransferConversationOwnerRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/members/transfer-owner"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<TransferConversationOwnerResult>() {});
    }

    /** Change a conversation member role */
    public ChangeConversationMemberRoleResult changeConversationMemberRole(String conversationId, ChangeConversationMemberRoleRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/members/change-role"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ChangeConversationMemberRoleResult>() {});
    }

    /** Leave a conversation */
    public ConversationMember leave(String conversationId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/members/leave"), null);
        return client.convertValue(raw, new TypeReference<ConversationMember>() {});
    }

    /** Get the current member read cursor */
    public ConversationReadCursorView getConversationReadCursor(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/conversations/" + conversationId + "/read-cursor"));
        return client.convertValue(raw, new TypeReference<ConversationReadCursorView>() {});
    }

    /** Update the current member read cursor */
    public ConversationReadCursorView updateConversationReadCursor(String conversationId, UpdateReadCursorRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/read-cursor"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ConversationReadCursorView>() {});
    }

    /** List projected conversation timeline entries */
    public TimelineResponse listConversationMessages(String conversationId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/conversations/" + conversationId + "/messages"));
        return client.convertValue(raw, new TypeReference<TimelineResponse>() {});
    }

    /** Post a standard conversation message */
    public PostMessageResult postConversationMessage(String conversationId, PostMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/messages"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PostMessageResult>() {});
    }

    /** Publish a message into a system channel conversation */
    public PostMessageResult publishSystemChannelMessage(String conversationId, PostMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/conversations/" + conversationId + "/system-channel/publish"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PostMessageResult>() {});
    }
}
