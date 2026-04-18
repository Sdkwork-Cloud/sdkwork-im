from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PostRtcSignalRequest:
    signal_type: str
    payload: str
    schema_ref: str = None
    signaling_stream_id: str = None
