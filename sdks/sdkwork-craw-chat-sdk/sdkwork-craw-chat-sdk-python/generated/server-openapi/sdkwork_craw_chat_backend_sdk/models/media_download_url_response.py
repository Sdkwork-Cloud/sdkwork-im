from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class MediaDownloadUrlResponse:
    media_asset_id: str
    storage_provider: str
    download_url: str
    expires_in_seconds: int
