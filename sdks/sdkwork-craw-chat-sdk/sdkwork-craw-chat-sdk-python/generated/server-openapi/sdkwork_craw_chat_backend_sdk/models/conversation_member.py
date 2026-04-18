from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ConversationMember:
    tenant_id: str
    conversation_id: str
    member_id: str
    principal_id: str
    principal_kind: str
    role: str
    state: str
    joined_at: str
    attributes: Dict[str, str]
    invited_by: str = None
    removed_at: str = None
