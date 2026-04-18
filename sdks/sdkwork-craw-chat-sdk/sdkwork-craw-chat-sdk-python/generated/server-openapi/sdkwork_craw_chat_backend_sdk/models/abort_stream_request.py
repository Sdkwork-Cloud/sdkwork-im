from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class AbortStreamRequest:
    frame_seq: int = None
    reason: str = None
