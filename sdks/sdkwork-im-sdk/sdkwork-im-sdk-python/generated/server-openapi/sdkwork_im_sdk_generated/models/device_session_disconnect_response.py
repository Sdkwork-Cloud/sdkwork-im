from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, List, Dict, Any


@dataclass
class DeviceSessionDisconnectResponse:
    device_id: str
    disconnected: bool
