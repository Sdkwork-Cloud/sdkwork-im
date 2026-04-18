from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PostMessageRequest:
    client_msg_id: str = None
    summary: str = None
    text: str = None
    parts: List[ContentPart] = None
    render_hints: Dict[str, str] = None
