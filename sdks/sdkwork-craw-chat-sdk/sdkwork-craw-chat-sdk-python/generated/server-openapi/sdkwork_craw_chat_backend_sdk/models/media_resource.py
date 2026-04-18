from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class MediaResource:
    id: int = None
    uuid: str = None
    url: str = None
    bytes: List[int] = None
    local_file: str = None
    base64: str = None
    type: str = None
    mime_type: str = None
    size: int = None
    name: str = None
    extension: str = None
    tags: Dict[str, str] = None
    metadata: Dict[str, str] = None
    prompt: str = None
