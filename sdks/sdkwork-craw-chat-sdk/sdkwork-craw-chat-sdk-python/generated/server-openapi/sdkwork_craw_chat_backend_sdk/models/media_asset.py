from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class MediaAsset:
    tenant_id: str
    principal_id: str
    principal_kind: str
    media_asset_id: str
    processing_state: str
    resource: MediaResource
    created_at: str
    bucket: str = None
    object_key: str = None
    storage_provider: str = None
    checksum: str = None
    completed_at: str = None
