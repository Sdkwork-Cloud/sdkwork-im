from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import PortalWorkspaceView

class PortalApi:
    """portal API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def get_home(self) -> Dict[str, Any]:
        """Read the tenant portal home snapshot"""
        return self._client.get(f"/api/v1/portal/home")

    def get_auth(self) -> Dict[str, Any]:
        """Read the tenant portal sign-in snapshot"""
        return self._client.get(f"/api/v1/portal/auth")

    def get_workspace(self) -> PortalWorkspaceView:
        """Read the current tenant workspace snapshot"""
        return self._client.get(f"/api/v1/portal/workspace")

    def get_dashboard(self) -> Dict[str, Any]:
        """Read the tenant dashboard snapshot"""
        return self._client.get(f"/api/v1/portal/dashboard")

    def get_conversations(self) -> Dict[str, Any]:
        """Read the tenant conversations snapshot"""
        return self._client.get(f"/api/v1/portal/conversations")

    def get_realtime(self) -> Dict[str, Any]:
        """Read the tenant realtime snapshot"""
        return self._client.get(f"/api/v1/portal/realtime")

    def get_media(self) -> Dict[str, Any]:
        """Read the tenant media snapshot"""
        return self._client.get(f"/api/v1/portal/media")

    def get_automation(self) -> Dict[str, Any]:
        """Read the tenant automation snapshot"""
        return self._client.get(f"/api/v1/portal/automation")

    def get_governance(self) -> Dict[str, Any]:
        """Read the tenant governance snapshot"""
        return self._client.get(f"/api/v1/portal/governance")
