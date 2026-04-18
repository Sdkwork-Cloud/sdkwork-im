from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class AckRealtimeEventsRequest:
    acked_seq: int
    device_id: str = None
