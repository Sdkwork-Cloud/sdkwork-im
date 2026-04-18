from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PortalUserView:
    id: str
    login: str
    name: str
    role: str
    email: str
    actor_kind: str
    client_kind: str
    permissions: List[str]
