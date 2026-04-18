from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class AppendStreamFrameRequest:
    frame_seq: int
    frame_type: str
    encoding: str
    payload: str
    schema_ref: str = None
    attributes: Dict[str, str] = None
