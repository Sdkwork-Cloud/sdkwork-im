from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class SyncRealtimeSubscriptionsRequest:
    device_id: str = None
    items: List[RealtimeSubscriptionItemInput] = None
