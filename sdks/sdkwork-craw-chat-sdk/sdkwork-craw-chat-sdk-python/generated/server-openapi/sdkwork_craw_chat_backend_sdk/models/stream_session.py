from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class StreamSession:
    tenant_id: str
    stream_id: str
    stream_type: str
    scope_kind: str
    scope_id: str
    durability_class: str
    ordering_scope: str
    state: str
    last_frame_seq: int
    opened_at: str
    schema_ref: str = None
    last_checkpoint_seq: int = None
    result_message_id: str = None
    closed_at: str = None
    expires_at: str = None
