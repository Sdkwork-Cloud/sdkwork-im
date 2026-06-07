from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DeviceSyncFeedEntry:
    tenant_id: str
    principal_id: str
    principal_kind: str
    sync_seq: int
    event_id: str
    origin_event_type: str
    occurred_at: str
    device_id: Optional[str] = None
    actor_id: Optional[str] = None
    conversation_id: Optional[str] = None
    message_id: Optional[str] = None
    message_seq: Optional[int] = None
    payload: Optional[str] = None
    read_seq: Optional[int] = None
    summary: Optional[str] = None
