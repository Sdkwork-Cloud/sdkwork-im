from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class TimelineViewEntry:
    tenant_id: str
    conversation_id: str
    message_id: str
    message_seq: int
    summary: str = None
