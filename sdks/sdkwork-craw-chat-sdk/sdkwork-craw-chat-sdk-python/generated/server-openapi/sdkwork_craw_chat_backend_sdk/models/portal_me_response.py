from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PortalMeResponse:
    tenant_id: str
    user: PortalUserView
    workspace: PortalWorkspaceView = None
