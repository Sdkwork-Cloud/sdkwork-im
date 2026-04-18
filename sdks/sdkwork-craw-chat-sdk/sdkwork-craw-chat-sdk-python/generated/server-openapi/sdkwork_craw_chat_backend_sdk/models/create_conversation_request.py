from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CreateConversationRequest:
    conversation_id: str
    conversation_type: str
