from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RtcSession:
    tenant_id: str
    rtc_session_id: str
    rtc_mode: str
    initiator_id: str
    state: str
    started_at: str
    conversation_id: str = None
    provider_plugin_id: str = None
    provider_session_id: str = None
    access_endpoint: str = None
    provider_region: str = None
    signaling_stream_id: str = None
    artifact_message_id: str = None
    ended_at: str = None
