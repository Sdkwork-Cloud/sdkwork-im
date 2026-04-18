from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ConversationInboxEntry:
    tenant_id: str
    principal_id: str
    member_id: str
    conversation_id: str
    conversation_type: str
    message_count: int
    last_message_seq: int
    unread_count: int
    last_activity_at: str
    last_message_id: str = None
    last_sender_id: str = None
    last_sender_kind: str = None
    last_summary: str = None
    agent_handoff: ConversationAgentHandoffView = None
