from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class UpdateRtcSessionRequest:
    artifact_message_id: str = None
