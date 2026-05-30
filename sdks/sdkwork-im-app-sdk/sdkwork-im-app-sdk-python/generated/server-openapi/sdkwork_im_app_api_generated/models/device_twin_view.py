from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DeviceTwinView:
    tenant_id: str
    device_id: str
    desired_state_json: str
    reported_state_json: str
    updated_at: str
