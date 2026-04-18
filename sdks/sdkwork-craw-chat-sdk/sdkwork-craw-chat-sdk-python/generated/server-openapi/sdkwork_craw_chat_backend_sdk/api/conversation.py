from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AddConversationMemberRequest, AgentHandoffStateView, ChangeConversationMemberRoleRequest, ChangeConversationMemberRoleResult, ConversationMember, ConversationReadCursorView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateSystemChannelRequest, ListMembersResponse, PostMessageRequest, PostMessageResult, RemoveConversationMemberRequest, TimelineResponse, TransferConversationOwnerRequest, TransferConversationOwnerResult, UpdateReadCursorRequest

class ConversationApi:
    """conversation API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def create_conversation(self, body: CreateConversationRequest) -> CreateConversationResult:
        """Create a conversation"""
        return self._client.post(f"/api/v1/conversations", json=body)

    def create_agent_dialog(self, body: CreateAgentDialogRequest) -> CreateConversationResult:
        """Create an agent dialog conversation"""
        return self._client.post(f"/api/v1/conversations/agent-dialogs", json=body)

    def create_agent_handoff(self, body: CreateAgentHandoffRequest) -> CreateConversationResult:
        """Create an agent handoff conversation"""
        return self._client.post(f"/api/v1/conversations/agent-handoffs", json=body)

    def create_system_channel(self, body: CreateSystemChannelRequest) -> CreateConversationResult:
        """Create a system channel conversation"""
        return self._client.post(f"/api/v1/conversations/system-channels", json=body)

    def get_conversation_summary(self, conversation_id: str) -> ConversationSummaryView:
        """Get projected conversation summary"""
        return self._client.get(f"/api/v1/conversations/{conversation_id}")

    def get_agent_handoff_state(self, conversation_id: str) -> AgentHandoffStateView:
        """Get current agent handoff state"""
        return self._client.get(f"/api/v1/conversations/{conversation_id}/agent-handoff")

    def accept_agent_handoff(self, conversation_id: str) -> AgentHandoffStateView:
        """Accept an agent handoff"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/agent-handoff/accept")

    def resolve_agent_handoff(self, conversation_id: str) -> AgentHandoffStateView:
        """Resolve an accepted agent handoff"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/agent-handoff/resolve")

    def close_agent_handoff(self, conversation_id: str) -> AgentHandoffStateView:
        """Close an agent handoff"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/agent-handoff/close")

    def list_conversation_members(self, conversation_id: str) -> ListMembersResponse:
        """List members in a conversation"""
        return self._client.get(f"/api/v1/conversations/{conversation_id}/members")

    def add_conversation_member(self, conversation_id: str, body: AddConversationMemberRequest) -> ConversationMember:
        """Add a member to a conversation"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/members/add", json=body)

    def remove_conversation_member(self, conversation_id: str, body: RemoveConversationMemberRequest) -> ConversationMember:
        """Remove a member from a conversation"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/members/remove", json=body)

    def transfer_conversation_owner(self, conversation_id: str, body: TransferConversationOwnerRequest) -> TransferConversationOwnerResult:
        """Transfer conversation ownership"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/members/transfer-owner", json=body)

    def change_conversation_member_role(self, conversation_id: str, body: ChangeConversationMemberRoleRequest) -> ChangeConversationMemberRoleResult:
        """Change a conversation member role"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/members/change-role", json=body)

    def leave(self, conversation_id: str) -> ConversationMember:
        """Leave a conversation"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/members/leave")

    def get_conversation_read_cursor(self, conversation_id: str) -> ConversationReadCursorView:
        """Get the current member read cursor"""
        return self._client.get(f"/api/v1/conversations/{conversation_id}/read-cursor")

    def update_conversation_read_cursor(self, conversation_id: str, body: UpdateReadCursorRequest) -> ConversationReadCursorView:
        """Update the current member read cursor"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/read-cursor", json=body)

    def list_conversation_messages(self, conversation_id: str) -> TimelineResponse:
        """List projected conversation timeline entries"""
        return self._client.get(f"/api/v1/conversations/{conversation_id}/messages")

    def post_conversation_message(self, conversation_id: str, body: PostMessageRequest) -> PostMessageResult:
        """Post a standard conversation message"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/messages", json=body)

    def publish_system_channel_message(self, conversation_id: str, body: PostMessageRequest) -> PostMessageResult:
        """Publish a message into a system channel conversation"""
        return self._client.post(f"/api/v1/conversations/{conversation_id}/system-channel/publish", json=body)
