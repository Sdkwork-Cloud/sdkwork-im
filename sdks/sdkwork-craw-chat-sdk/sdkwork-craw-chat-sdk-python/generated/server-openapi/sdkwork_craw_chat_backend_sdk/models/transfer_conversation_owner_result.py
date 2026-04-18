from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class TransferConversationOwnerResult:
    event_id: str
    transferred_at: str
    previous_owner: ConversationMember
    new_owner: ConversationMember
