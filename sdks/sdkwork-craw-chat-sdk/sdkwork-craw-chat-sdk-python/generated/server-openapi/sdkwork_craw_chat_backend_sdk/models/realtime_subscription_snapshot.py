from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RealtimeSubscriptionSnapshot:
    tenant_id: str
    principal_id: str
    device_id: str
    items: List[RealtimeSubscription]
    synced_at: str
