from typing import Any, Dict, List, Optional
from ..http_client import HttpClient

def _append_query_string(path: str, raw_query_string: str) -> str:
    query = raw_query_string.lstrip('?')
    if not query:
        return path
    separator = '&' if '?' in path else '?'
    return f"{path}{separator}{query}"





class IotApi:
    """iot iot API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.access_provider_health = IotAccessProviderHealthApi(client)
        self.protocol_provider_health = IotProtocolProviderHealthApi(client)
        self.protocol = IotProtocolApi(client)


class IotAccessProviderHealthApi:
    """iot iot.access_provider_health API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve IoT access provider health"""
        return self._client.get(f"/app/v3/api/iot/access/provider_health")

class IotProtocolProviderHealthApi:
    """iot iot.protocol_provider_health API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def retrieve(self) -> Dict[str, Any]:
        """Retrieve IoT protocol provider health"""
        return self._client.get(f"/app/v3/api/iot/protocol/provider_health")

class IotProtocolApi:
    """iot iot.protocol API client."""

    def __init__(self, client: HttpClient):
        self._client = client
        self.uplink = IotProtocolUplinkApi(client)
        self.downlink = IotProtocolDownlinkApi(client)


class IotProtocolUplinkApi:
    """iot iot.protocol.uplink API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> Dict[str, Any]:
        """Ingest IoT protocol uplink"""
        return self._client.post(f"/app/v3/api/iot/protocol/uplink")

class IotProtocolDownlinkApi:
    """iot iot.protocol.downlink API client."""

    def __init__(self, client: HttpClient):
        self._client = client


    def create(self) -> Dict[str, Any]:
        """Ingest IoT protocol downlink"""
        return self._client.post(f"/app/v3/api/iot/protocol/downlink")
