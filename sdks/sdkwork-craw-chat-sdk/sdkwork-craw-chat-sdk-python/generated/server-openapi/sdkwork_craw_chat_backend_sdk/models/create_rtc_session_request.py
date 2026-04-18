from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CreateRtcSessionRequest:
    rtc_session_id: str
    rtc_mode: str
    conversation_id: str = None
