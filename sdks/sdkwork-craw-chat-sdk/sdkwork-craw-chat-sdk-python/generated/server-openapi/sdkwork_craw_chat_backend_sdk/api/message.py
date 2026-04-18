from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import EditMessageRequest, MessageMutationResult

class MessageApi:
    """message API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def edit(self, message_id: str, body: EditMessageRequest) -> MessageMutationResult:
        """Edit a posted message"""
        return self._client.post(f"/api/v1/messages/{message_id}/edit", json=body)

    def recall(self, message_id: str) -> MessageMutationResult:
        """Recall a posted message"""
        return self._client.post(f"/api/v1/messages/{message_id}/recall")
