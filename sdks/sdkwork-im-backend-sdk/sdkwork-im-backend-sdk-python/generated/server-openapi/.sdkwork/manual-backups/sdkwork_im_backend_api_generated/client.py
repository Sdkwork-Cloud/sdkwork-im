from .http_client import HttpClient, SdkConfig
from .api.ops import OpsApi
from .api.audit import AuditApi
from .api.automation import AutomationApi
from .api.control import ControlApi
from .api.admin import AdminApi


class SdkworkBackendClient:
    """sdkwork-im-backend-sdk SDK Client."""

    def __init__(self, config: SdkConfig):
        self._client = HttpClient(config)
        self.ops: OpsApi
        self.audit: AuditApi
        self.automation: AutomationApi
        self.control: ControlApi
        self.admin: AdminApi

        # Initialize API modules
        self.ops = OpsApi(self._client)
        self.audit = AuditApi(self._client)
        self.automation = AutomationApi(self._client)
        self.control = ControlApi(self._client)
        self.admin = AdminApi(self._client)


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
