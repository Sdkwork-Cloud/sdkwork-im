from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RealtimeAckState:
    tenant_id: str
    principal_id: str
    device_id: str
    acked_through_seq: int
    trimmed_through_seq: int
    retained_event_count: int
    acked_at: str
