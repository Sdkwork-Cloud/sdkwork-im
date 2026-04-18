from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class DeviceSyncFeedEntry:
    tenant_id: str
    principal_id: str
    device_id: str
    sync_seq: int
    origin_event_id: str
    origin_event_type: str
    occurred_at: str
    conversation_id: str = None
    message_id: str = None
    message_seq: int = None
    member_id: str = None
    read_seq: int = None
    last_read_message_id: str = None
    actor_id: str = None
    actor_kind: str = None
    actor_device_id: str = None
    summary: str = None
    payload_schema: str = None
    payload: str = None
