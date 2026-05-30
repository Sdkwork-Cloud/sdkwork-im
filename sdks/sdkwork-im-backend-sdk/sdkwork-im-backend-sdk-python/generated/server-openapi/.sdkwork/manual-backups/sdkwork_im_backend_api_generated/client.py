from .http_client import HttpClient, SdkConfig
from .api.ops import OpsApi
from .api.audit import AuditApi
from .api.provider import ProviderApi
from .api.iot import IotApi
from .api.rtc import RtcApi
from .api.automation import AutomationApi


class SdkworkBackendClient:
    """sdkwork-im-backend-sdk SDK Client."""

    def __init__(self, config: SdkConfig):
        self._client = HttpClient(config)
        self.ops: OpsApi
        self.audit: AuditApi
        self.provider: ProviderApi
        self.iot: IotApi
        self.rtc: RtcApi
        self.automation: AutomationApi

        # Initialize API modules
        self.ops = OpsApi(self._client)
        self.audit = AuditApi(self._client)
        self.provider = ProviderApi(self._client)
        self.iot = IotApi(self._client)
        self.rtc = RtcApi(self._client)
        self.automation = AutomationApi(self._client)


    def set_auth_token(self, token: str) -> 'SdkworkBackendClient':
        """Set auth token for authentication."""
        self._client.set_auth_token(token)
        return self

    def set_access_token(self, token: str) -> 'SdkworkBackendClient':
        """Set access token for authentication."""
        self._client.set_access_token(token)
        return self

    def set_header(self, key: str, value: str) -> 'SdkworkBackendClient':
        """Set custom header."""
        self._client.set_header(key, value)
        return self

    @property
    def http(self) -> HttpClient:
        """Get the underlying HTTP client."""
        return self._client


def create_client(config: SdkConfig) -> SdkworkBackendClient:
    """Create a new SDK client instance."""
    return SdkworkBackendClient(config)
