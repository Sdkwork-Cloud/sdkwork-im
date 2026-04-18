from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class AddConversationMemberRequest:
    principal_id: str
    principal_kind: str
    role: str
