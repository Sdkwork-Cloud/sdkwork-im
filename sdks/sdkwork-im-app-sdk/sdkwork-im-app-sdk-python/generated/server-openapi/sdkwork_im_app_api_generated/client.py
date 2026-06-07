from .http_client import HttpClient, SdkConfig
from .api.automation import AutomationApi
from .api.device import DeviceApi
from .api.notification import NotificationApi
from .api.portal import PortalApi
from .api.provider import ProviderApi
from .api.iot import IotApi


class SdkworkImAppClient:
    """sdkwork-im-app-sdk SDK Client."""

    def __init__(self, config: SdkConfig):
        self._client = HttpClient(config)
        self.automation: AutomationApi
        self.device: DeviceApi
        self.notification: NotificationApi
        self.portal: PortalApi
        self.provider: ProviderApi
        self.iot: IotApi

        # Initialize API modules
        self.automation = AutomationApi(self._client)
        self.device = DeviceApi(self._client)
        self.notification = NotificationApi(self._client)
        self.portal = PortalApi(self._client)
        self.provider = ProviderApi(self._client)
        self.iot = IotApi(self._client)
    def set_auth_token(self, token: str) -> 'SdkworkImAppClient':
        """Set auth token for authentication."""
        self._client.set_auth_token(token)
        return self

    def set_access_token(self, token: str) -> 'SdkworkImAppClient':
        """Set access token for authentication."""
        self._client.set_access_token(token)
        return self

    def set_header(self, key: str, value: str) -> 'SdkworkImAppClient':
        """Set custom header."""
        self._client.set_header(key, value)
        return self

    @property
    def http(self) -> HttpClient:
        """Get the underlying HTTP client."""
        return self._client


def create_client(config: SdkConfig) -> SdkworkImAppClient:
    """Create a new SDK client instance."""
    return SdkworkImAppClient(config)

SdkworkAppClient = SdkworkImAppClient
