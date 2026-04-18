from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class AttachMediaRequest:
    conversation_id: str
    client_msg_id: str = None
    summary: str = None
    text: str = None
    render_hints: Dict[str, str] = None
