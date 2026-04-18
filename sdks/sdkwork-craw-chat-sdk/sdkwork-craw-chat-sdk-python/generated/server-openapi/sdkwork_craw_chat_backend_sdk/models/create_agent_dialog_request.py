from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CreateAgentDialogRequest:
    conversation_id: str
    agent_id: str
