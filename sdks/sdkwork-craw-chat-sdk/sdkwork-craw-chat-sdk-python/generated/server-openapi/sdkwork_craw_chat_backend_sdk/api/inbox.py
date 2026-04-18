from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import InboxResponse

class InboxApi:
    """inbox API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def get_inbox(self) -> InboxResponse:
        """Get inbox entries"""
        return self._client.get(f"/api/v1/inbox")
