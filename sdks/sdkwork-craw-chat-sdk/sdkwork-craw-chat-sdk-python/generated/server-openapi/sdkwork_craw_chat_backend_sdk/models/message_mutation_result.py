from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class MessageMutationResult:
    conversation_id: str
    message_id: str
    message_seq: int
    event_id: str
