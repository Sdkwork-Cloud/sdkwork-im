from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ResumeSessionRequest:
    device_id: str = None
    last_seen_sync_seq: int = None
