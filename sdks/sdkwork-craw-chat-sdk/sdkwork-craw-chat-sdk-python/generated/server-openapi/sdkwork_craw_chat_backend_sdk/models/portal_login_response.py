from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class PortalLoginResponse:
    access_token: str
    refresh_token: str
    expires_at: int
    user: PortalUserView
    workspace: PortalWorkspaceView = None
