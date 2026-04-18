from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CreateAgentHandoffRequest:
    conversation_id: str
    target_id: str
    target_kind: str
    handoff_session_id: str
    handoff_reason: str = None
