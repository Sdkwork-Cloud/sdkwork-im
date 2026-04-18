from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ContentPart:
    kind: str
    text: str = None
    schema_ref: str = None
    encoding: str = None
    payload: str = None
    media_asset_id: str = None
    resource: MediaResource = None
    signal_type: str = None
    stream_id: str = None
    stream_type: str = None
    state: str = None
