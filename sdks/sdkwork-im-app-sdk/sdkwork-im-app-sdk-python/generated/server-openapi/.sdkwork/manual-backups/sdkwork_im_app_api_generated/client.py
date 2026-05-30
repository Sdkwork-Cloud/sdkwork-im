from .http_client import HttpClient, SdkConfig
from .api.portal import PortalApi
from .api.device import DeviceApi
from .api.presence import PresenceApi
from .api.realtime import RealtimeApi
from .api.social import SocialApi
from .api.chat import ChatApi
from .api.media import MediaApi
from .api.stream import StreamApi
from .api.rtc import RtcApi
from .api.notification import NotificationApi
from .api.automation import AutomationApi


class SdkworkAppClient:
    """sdkwork-im-app-sdk SDK Client."""

    def __init__(self, config: SdkConfig):
        self._client = HttpClient(config)
        self.portal: PortalApi
        self.device: DeviceApi
        self.presence: PresenceApi
        self.realtime: RealtimeApi
        self.social: SocialApi
        self.chat: ChatApi
        self.media: MediaApi
        self.stream: StreamApi
        self.rtc: RtcApi
        self.notification: NotificationApi
        self.automation: AutomationApi

        # Initialize API modules
        self.portal = PortalApi(self._client)
        self.device = DeviceApi(self._client)
        self.presence = PresenceApi(self._client)
        self.realtime = RealtimeApi(self._client)
        self.social = SocialApi(self._client)
        self.chat = ChatApi(self._client)
        self.media = MediaApi(self._client)
        self.stream = StreamApi(self._client)
        self.rtc = RtcApi(self._client)
        self.notification = NotificationApi(self._client)
        self.automation = AutomationApi(self._client)


    def set_auth_token(self, token: str) -> 'SdkworkAppClient':
        """Set auth token for authentication."""
        self._client.set_auth_token(token)
        return self

    def set_access_token(self, token: str) -> 'SdkworkAppClient':
        """Set access token for authentication."""
        self._client.set_access_token(token)
        return self

    def set_header(self, key: str, value: str) -> 'SdkworkAppClient':
        """Set custom header."""
        self._client.set_header(key, value)
        return self

    @property
    def http(self) -> HttpClient:
        """Get the underlying HTTP client."""
        return self._client


def create_client(config: SdkConfig) -> SdkworkAppClient:
    """Create a new SDK client instance."""
    return SdkworkAppClient(config)
