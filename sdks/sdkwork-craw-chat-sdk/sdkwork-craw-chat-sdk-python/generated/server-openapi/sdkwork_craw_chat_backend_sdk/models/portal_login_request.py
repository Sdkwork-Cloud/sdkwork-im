from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PortalLoginRequest:
    tenant_id: str
    login: str
    password: str
    device_id: str = None
    session_id: str = None
    client_kind: str = None
