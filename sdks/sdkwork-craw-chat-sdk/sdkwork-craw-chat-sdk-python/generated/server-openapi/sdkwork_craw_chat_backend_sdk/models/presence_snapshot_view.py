from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PresenceSnapshotView:
    tenant_id: str
    principal_id: str
    devices: List[DevicePresenceView]
    current_device_id: str = None
