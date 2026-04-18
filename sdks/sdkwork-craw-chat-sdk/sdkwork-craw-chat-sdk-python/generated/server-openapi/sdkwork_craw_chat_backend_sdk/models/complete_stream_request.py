from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CompleteStreamRequest:
    frame_seq: int
    result_message_id: str = None
