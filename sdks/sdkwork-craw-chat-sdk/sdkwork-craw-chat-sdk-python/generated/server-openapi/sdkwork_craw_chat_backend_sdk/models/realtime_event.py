from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RealtimeEvent:
    tenant_id: str
    principal_id: str
    device_id: str
    realtime_seq: int
    scope_type: str
    scope_id: str
    event_type: str
    delivery_class: str
    payload: str
    occurred_at: str
