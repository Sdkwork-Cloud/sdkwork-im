from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class OpenStreamRequest:
    stream_id: str
    stream_type: str
    scope_kind: str
    scope_id: str
    durability_class: str
    schema_ref: str = None
