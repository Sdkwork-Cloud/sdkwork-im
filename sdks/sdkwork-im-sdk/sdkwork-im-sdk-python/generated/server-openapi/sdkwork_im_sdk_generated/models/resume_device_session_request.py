from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class ResumeDeviceSessionRequest:
    device_id: Optional[str] = None
    last_seen_sync_seq: Optional[int] = None
