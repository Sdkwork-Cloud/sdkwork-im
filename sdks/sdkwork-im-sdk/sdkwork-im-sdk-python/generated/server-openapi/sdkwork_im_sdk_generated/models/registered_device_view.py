from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class RegisteredDeviceView:
    tenant_id: str
    principal_id: str
    principal_kind: str
    device_id: str
    registered_at: str
