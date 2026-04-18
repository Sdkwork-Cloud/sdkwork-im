from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class CreateUploadRequest:
    media_asset_id: str
    resource: MediaResource
