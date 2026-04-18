from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class Sender:
    id: str
    kind: str
    metadata: Dict[str, str]
    member_id: str = None
    device_id: str = None
    session_id: str = None
