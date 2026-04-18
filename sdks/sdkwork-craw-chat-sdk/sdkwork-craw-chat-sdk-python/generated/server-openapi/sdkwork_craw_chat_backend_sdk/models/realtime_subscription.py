from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RealtimeSubscription:
    scope_type: str
    scope_id: str
    event_types: List[str]
    subscribed_at: str
