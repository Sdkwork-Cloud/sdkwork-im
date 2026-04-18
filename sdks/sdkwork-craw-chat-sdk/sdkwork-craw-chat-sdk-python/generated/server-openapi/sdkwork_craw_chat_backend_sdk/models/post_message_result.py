from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PostMessageResult:
    message_id: str
    message_seq: int
    event_id: str
