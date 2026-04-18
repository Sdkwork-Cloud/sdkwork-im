from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class TransferConversationOwnerRequest:
    member_id: str
