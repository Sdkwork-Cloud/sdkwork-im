from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import PresenceDeviceRequest, PresenceSnapshotView

class PresenceApi:
    """presence API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def heartbeat(self, body: PresenceDeviceRequest) -> PresenceSnapshotView:
        """Refresh device presence"""
        return self._client.post(f"/api/v1/presence/heartbeat", json=body)

    def get_presence_me(self) -> PresenceSnapshotView:
        """Get current presence"""
        return self._client.get(f"/api/v1/presence/me")
