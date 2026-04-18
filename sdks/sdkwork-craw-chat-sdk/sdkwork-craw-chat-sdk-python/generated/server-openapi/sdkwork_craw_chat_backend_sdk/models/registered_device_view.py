from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class RegisteredDeviceView:
    tenant_id: str
    principal_id: str
    device_id: str
    registered_at: str
