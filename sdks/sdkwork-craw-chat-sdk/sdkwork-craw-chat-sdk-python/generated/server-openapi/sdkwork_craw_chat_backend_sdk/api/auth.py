from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import PortalLoginRequest, PortalLoginResponse, PortalMeResponse

class AuthApi:
    """auth API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def login(self, body: PortalLoginRequest) -> PortalLoginResponse:
        """Sign in to the tenant portal"""
        return self._client.post(f"/api/v1/auth/login", json=body)

    def me(self) -> PortalMeResponse:
        """Read the current portal session"""
        return self._client.get(f"/api/v1/auth/me")
