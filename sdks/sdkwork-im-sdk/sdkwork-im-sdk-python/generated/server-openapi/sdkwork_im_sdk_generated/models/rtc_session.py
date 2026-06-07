from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RtcSession:
    tenant_id: str
    rtc_session_id: str
    rtc_mode: str
    state: str
    created_at: str
    updated_at: str
    conversation_id: Optional[str] = None
    provider_plugin_id: Optional[str] = None
    provider_session_id: Optional[str] = None
