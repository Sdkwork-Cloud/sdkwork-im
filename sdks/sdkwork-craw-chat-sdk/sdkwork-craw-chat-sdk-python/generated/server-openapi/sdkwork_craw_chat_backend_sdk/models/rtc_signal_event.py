from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RtcSignalEvent:
    tenant_id: str
    rtc_session_id: str
    rtc_mode: str
    signal_type: str
    payload: str
    sender: Sender
    occurred_at: str
    conversation_id: str = None
    schema_ref: str = None
    signaling_stream_id: str = None
