from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .device_sync_feed_entry import DeviceSyncFeedEntry


@dataclass
class DeviceSyncFeedResponse:
    items: List[DeviceSyncFeedEntry]
    has_more: bool
    trimmed_through_seq: int
    next_after_seq: Optional[int] = None
