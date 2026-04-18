from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class UpdateReadCursorRequest:
    read_seq: int
    last_read_message_id: str = None
