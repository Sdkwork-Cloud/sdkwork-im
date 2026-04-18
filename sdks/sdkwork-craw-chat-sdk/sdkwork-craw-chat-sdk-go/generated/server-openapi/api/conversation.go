package api

import (
    "fmt"
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type ConversationApi struct {
    client *sdkhttp.Client
}

func NewConversationApi(client *sdkhttp.Client) *ConversationApi {
    return &ConversationApi{client: client}
}

// Create a conversation
func (a *ConversationApi) CreateConversation(body sdktypes.CreateConversationRequest) (sdktypes.CreateConversationResult, error) {
    raw, err := a.client.Post(BackendApiPath("/conversations"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.CreateConversationResult
        return zero, err
    }
    return decodeResult[sdktypes.CreateConversationResult](raw)
}

// Create an agent dialog conversation
func (a *ConversationApi) CreateAgentDialog(body sdktypes.CreateAgentDialogRequest) (sdktypes.CreateConversationResult, error) {
    raw, err := a.client.Post(BackendApiPath("/conversations/agent-dialogs"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.CreateConversationResult
        return zero, err
    }
    return decodeResult[sdktypes.CreateConversationResult](raw)
}

// Create an agent handoff conversation
func (a *ConversationApi) CreateAgentHandoff(body sdktypes.CreateAgentHandoffRequest) (sdktypes.CreateConversationResult, error) {
    raw, err := a.client.Post(BackendApiPath("/conversations/agent-handoffs"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.CreateConversationResult
        return zero, err
    }
    return decodeResult[sdktypes.CreateConversationResult](raw)
}

// Create a system channel conversation
func (a *ConversationApi) CreateSystemChannel(body sdktypes.CreateSystemChannelRequest) (sdktypes.CreateConversationResult, error) {
    raw, err := a.client.Post(BackendApiPath("/conversations/system-channels"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.CreateConversationResult
        return zero, err
    }
    return decodeResult[sdktypes.CreateConversationResult](raw)
}

// Get projected conversation summary
func (a *ConversationApi) GetConversationSummary(conversationId string) (sdktypes.ConversationSummaryView, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/conversations/%s", conversationId)), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationSummaryView
        return zero, err
    }
    return decodeResult[sdktypes.ConversationSummaryView](raw)
}

// Get current agent handoff state
func (a *ConversationApi) GetAgentHandoffState(conversationId string) (sdktypes.AgentHandoffStateView, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/conversations/%s/agent-handoff", conversationId)), nil, nil)
    if err != nil {
        var zero sdktypes.AgentHandoffStateView
        return zero, err
    }
    return decodeResult[sdktypes.AgentHandoffStateView](raw)
}

// Accept an agent handoff
func (a *ConversationApi) AcceptAgentHandoff(conversationId string) (sdktypes.AgentHandoffStateView, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/agent-handoff/accept", conversationId)), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.AgentHandoffStateView
        return zero, err
    }
    return decodeResult[sdktypes.AgentHandoffStateView](raw)
}

// Resolve an accepted agent handoff
func (a *ConversationApi) ResolveAgentHandoff(conversationId string) (sdktypes.AgentHandoffStateView, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/agent-handoff/resolve", conversationId)), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.AgentHandoffStateView
        return zero, err
    }
    return decodeResult[sdktypes.AgentHandoffStateView](raw)
}

// Close an agent handoff
func (a *ConversationApi) CloseAgentHandoff(conversationId string) (sdktypes.AgentHandoffStateView, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/agent-handoff/close", conversationId)), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.AgentHandoffStateView
        return zero, err
    }
    return decodeResult[sdktypes.AgentHandoffStateView](raw)
}

// List members in a conversation
func (a *ConversationApi) ListConversationMembers(conversationId string) (sdktypes.ListMembersResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/conversations/%s/members", conversationId)), nil, nil)
    if err != nil {
        var zero sdktypes.ListMembersResponse
        return zero, err
    }
    return decodeResult[sdktypes.ListMembersResponse](raw)
}

// Add a member to a conversation
func (a *ConversationApi) AddConversationMember(conversationId string, body sdktypes.AddConversationMemberRequest) (sdktypes.ConversationMember, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/members/add", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationMember
        return zero, err
    }
    return decodeResult[sdktypes.ConversationMember](raw)
}

// Remove a member from a conversation
func (a *ConversationApi) RemoveConversationMember(conversationId string, body sdktypes.RemoveConversationMemberRequest) (sdktypes.ConversationMember, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/members/remove", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationMember
        return zero, err
    }
    return decodeResult[sdktypes.ConversationMember](raw)
}

// Transfer conversation ownership
func (a *ConversationApi) TransferConversationOwner(conversationId string, body sdktypes.TransferConversationOwnerRequest) (sdktypes.TransferConversationOwnerResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/members/transfer-owner", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.TransferConversationOwnerResult
        return zero, err
    }
    return decodeResult[sdktypes.TransferConversationOwnerResult](raw)
}

// Change a conversation member role
func (a *ConversationApi) ChangeConversationMemberRole(conversationId string, body sdktypes.ChangeConversationMemberRoleRequest) (sdktypes.ChangeConversationMemberRoleResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/members/change-role", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ChangeConversationMemberRoleResult
        return zero, err
    }
    return decodeResult[sdktypes.ChangeConversationMemberRoleResult](raw)
}

// Leave a conversation
func (a *ConversationApi) Leave(conversationId string) (sdktypes.ConversationMember, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/members/leave", conversationId)), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ConversationMember
        return zero, err
    }
    return decodeResult[sdktypes.ConversationMember](raw)
}

// Get the current member read cursor
func (a *ConversationApi) GetConversationReadCursor(conversationId string) (sdktypes.ConversationReadCursorView, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/conversations/%s/read-cursor", conversationId)), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationReadCursorView
        return zero, err
    }
    return decodeResult[sdktypes.ConversationReadCursorView](raw)
}

// Update the current member read cursor
func (a *ConversationApi) UpdateConversationReadCursor(conversationId string, body sdktypes.UpdateReadCursorRequest) (sdktypes.ConversationReadCursorView, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/read-cursor", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationReadCursorView
        return zero, err
    }
    return decodeResult[sdktypes.ConversationReadCursorView](raw)
}

// List projected conversation timeline entries
func (a *ConversationApi) ListConversationMessages(conversationId string) (sdktypes.TimelineResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/conversations/%s/messages", conversationId)), nil, nil)
    if err != nil {
        var zero sdktypes.TimelineResponse
        return zero, err
    }
    return decodeResult[sdktypes.TimelineResponse](raw)
}

// Post a standard conversation message
func (a *ConversationApi) PostConversationMessage(conversationId string, body sdktypes.PostMessageRequest) (sdktypes.PostMessageResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/messages", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PostMessageResult
        return zero, err
    }
    return decodeResult[sdktypes.PostMessageResult](raw)
}

// Publish a message into a system channel conversation
func (a *ConversationApi) PublishSystemChannelMessage(conversationId string, body sdktypes.PostMessageRequest) (sdktypes.PostMessageResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/conversations/%s/system-channel/publish", conversationId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PostMessageResult
        return zero, err
    }
    return decodeResult[sdktypes.PostMessageResult](raw)
}
