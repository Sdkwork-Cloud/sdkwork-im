from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ConversationReadCursorView:
    tenant_id: str
    conversation_id: str
    member_id: str
    principal_id: str
    read_seq: int
    updated_at: str
    unread_count: int
    last_read_message_id: str = None
