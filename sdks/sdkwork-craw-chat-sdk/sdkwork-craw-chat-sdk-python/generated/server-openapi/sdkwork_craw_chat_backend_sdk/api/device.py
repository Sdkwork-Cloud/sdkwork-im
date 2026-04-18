from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import DeviceSyncFeedResponse, RegisterDeviceRequest, RegisteredDeviceView

class DeviceApi:
    """device API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def register(self, body: RegisterDeviceRequest) -> RegisteredDeviceView:
        """Register the current device"""
        return self._client.post(f"/api/v1/devices/register", json=body)

    def get_device_sync_feed(self, device_id: str, params: Optional[Dict[str, Any]] = None) -> DeviceSyncFeedResponse:
        """Get device sync feed entries"""
        return self._client.get(f"/api/v1/devices/{device_id}/sync-feed", params=params)
