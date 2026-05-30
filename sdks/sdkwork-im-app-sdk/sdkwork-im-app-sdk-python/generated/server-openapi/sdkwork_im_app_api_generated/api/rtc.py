from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class RtcApi:
    """rtc rtc API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.provider_callbacks = RtcProviderCallbacksApi(client)
        self.provider_health = RtcProviderHealthApi(client)


class RtcProviderCallbacksApi:
    """rtc rtc.provider_callbacks API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> Dict[str, Any]:
        """Map RTC provider callback"""
        return self._client.post(f"/app/v3/api/rtc/provider_callbacks")

class RtcProviderHealthApi:
    """rtc rtc.provider_health API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve RTC provider health"""
        return self._client.get(f"/app/v3/api/rtc/provider_health")
