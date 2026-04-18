from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ChangeConversationMemberRoleRequest:
    member_id: str
    role: str
