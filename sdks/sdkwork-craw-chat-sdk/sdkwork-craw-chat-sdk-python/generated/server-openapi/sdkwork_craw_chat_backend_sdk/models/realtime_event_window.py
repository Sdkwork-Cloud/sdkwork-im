from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RealtimeEventWindow:
    device_id: str
    items: List[RealtimeEvent]
    has_more: bool
    acked_through_seq: int
    trimmed_through_seq: int
    next_after_seq: int = None
