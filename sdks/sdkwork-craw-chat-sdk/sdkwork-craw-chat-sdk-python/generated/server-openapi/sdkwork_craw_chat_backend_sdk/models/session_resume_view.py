from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class SessionResumeView:
    tenant_id: str
    actor_id: str
    actor_kind: str
    device_id: str
    resume_required: bool
    resume_from_sync_seq: int
    latest_sync_seq: int
    resumed_at: str
    presence: PresenceSnapshotView
    session_id: str = None
