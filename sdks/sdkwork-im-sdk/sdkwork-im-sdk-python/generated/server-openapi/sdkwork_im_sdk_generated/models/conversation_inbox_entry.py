from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ConversationInboxEntry:
    tenant_id: str
    conversation_id: str
    conversation_type: str
    last_activity_at: str
    message_count: int
    last_message_seq: int
    unread_count: int
    agent_handoff: Optional[bool] = None
    last_message_id: Optional[str] = None
    last_sender_id: Optional[str] = None
    last_summary: Optional[str] = None
    last_message_at: Optional[str] = None
