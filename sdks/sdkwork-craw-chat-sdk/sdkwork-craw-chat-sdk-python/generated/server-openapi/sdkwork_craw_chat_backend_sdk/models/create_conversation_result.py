from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CreateConversationResult:
    conversation_id: str
    event_id: str
