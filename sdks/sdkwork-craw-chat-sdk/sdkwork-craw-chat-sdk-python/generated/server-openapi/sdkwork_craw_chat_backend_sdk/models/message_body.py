from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class MessageBody:
    parts: List[ContentPart]
    render_hints: Dict[str, str]
    summary: str = None
