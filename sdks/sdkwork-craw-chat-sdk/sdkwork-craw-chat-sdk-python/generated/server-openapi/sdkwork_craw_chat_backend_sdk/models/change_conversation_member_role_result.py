from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ChangeConversationMemberRoleResult:
    event_id: str
    changed_at: str
    previous_member: ConversationMember
    updated_member: ConversationMember
