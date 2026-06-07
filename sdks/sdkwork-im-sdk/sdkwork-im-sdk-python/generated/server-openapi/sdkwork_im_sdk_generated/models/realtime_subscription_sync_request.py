from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RealtimeSubscriptionSyncRequest:
    device_id: Optional[str] = None
    conversations: Optional[List[str]] = None
