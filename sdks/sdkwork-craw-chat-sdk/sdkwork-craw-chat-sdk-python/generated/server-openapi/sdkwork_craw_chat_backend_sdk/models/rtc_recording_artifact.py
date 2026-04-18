from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RtcRecordingArtifact:
    tenant_id: str
    rtc_session_id: str
    bucket: str
    object_key: str
    storage_provider: str = None
    playback_url: str = None
