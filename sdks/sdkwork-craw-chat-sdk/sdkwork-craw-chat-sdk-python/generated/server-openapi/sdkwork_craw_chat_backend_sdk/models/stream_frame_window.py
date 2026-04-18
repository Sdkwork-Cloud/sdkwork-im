from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class StreamFrameWindow:
    items: List[StreamFrame]
    has_more: bool
    next_after_frame_seq: int = None
