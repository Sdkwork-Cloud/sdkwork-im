from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AckRealtimeEventsRequest, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest

class RealtimeApi:
    """realtime API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def sync_realtime_subscriptions(self, body: SyncRealtimeSubscriptionsRequest) -> RealtimeSubscriptionSnapshot:
        """Replace realtime subscriptions for the current device"""
        return self._client.post(f"/api/v1/realtime/subscriptions/sync", json=body)

    def list_realtime_events(self, params: Optional[Dict[str, Any]] = None) -> RealtimeEventWindow:
        """Pull realtime events for the current device"""
        return self._client.get(f"/api/v1/realtime/events", params=params)

    def ack_realtime_events(self, body: AckRealtimeEventsRequest) -> RealtimeAckState:
        """Ack realtime events for the current device"""
        return self._client.post(f"/api/v1/realtime/events/ack", json=body)
