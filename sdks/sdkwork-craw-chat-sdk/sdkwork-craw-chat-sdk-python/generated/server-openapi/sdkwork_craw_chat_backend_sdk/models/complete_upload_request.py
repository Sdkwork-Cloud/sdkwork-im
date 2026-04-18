from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CompleteUploadRequest:
    bucket: str
    object_key: str
    url: str
    storage_provider: str = None
    checksum: str = None
