from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import PresenceDeviceRequest, PresenceSnapshotView, ResumeSessionRequest, SessionResumeView

class SessionApi:
    """session API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def resume(self, body: ResumeSessionRequest) -> SessionResumeView:
        """Resume the current app session"""
        return self._client.post(f"/api/v1/sessions/resume", json=body)

    def disconnect(self, body: PresenceDeviceRequest) -> PresenceSnapshotView:
        """Disconnect the current app session device route"""
        return self._client.post(f"/api/v1/sessions/disconnect", json=body)
