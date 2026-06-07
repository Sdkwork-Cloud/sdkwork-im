from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DeviceSessionView:
    tenant_id: str
    principal_id: str
    principal_kind: str
    device_id: str
    resumed_at: str
