from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class DevicePresenceView:
    tenant_id: str
    principal_id: str
    device_id: str
    status: str
    last_sync_seq: int
    platform: str = None
    session_id: str = None
    last_resume_at: str = None
    last_seen_at: str = None
