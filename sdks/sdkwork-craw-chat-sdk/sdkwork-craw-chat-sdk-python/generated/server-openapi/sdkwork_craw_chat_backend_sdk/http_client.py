from sdkwork.common.core.types import SdkConfig as CommonSdkConfig
from sdkwork.common.http import BaseHttpClient

SdkConfig = CommonSdkConfig
API_KEY_HEADER = 'Authorization'
API_KEY_USE_BEARER = True


class HttpClient(BaseHttpClient):
    """
    SDK HTTP client wrapper based on sdkwork-common.

    Auth headers:
    - api_key -> Authorization: Bearer {api_key}
    - auth_token -> Authorization: Bearer {auth_token}
    - access_token -> Access-Token: {access_token}
    """

    def _update_auth_headers(self) -> None:
        if self._session is None:
            return

        self._session.headers.pop('Authorization', None)
        self._session.headers.pop('Access-Token', None)
        self._session.headers.pop('X-API-Key', None)

        if self._api_key:
            self._session.headers[API_KEY_HEADER] = f'Bearer {self._api_key}' if API_KEY_USE_BEARER else self._api_key
        if self._auth_token:
            self._session.headers['Authorization'] = f'Bearer {self._auth_token}'
        if self._access_token:
            self._session.headers['Access-Token'] = self._access_token

    def set_api_key(self, api_key: str) -> 'HttpClient':
        self._api_key = api_key
        self._auth_token = None
        self._access_token = None
        self._update_auth_headers()
        return self

    def set_auth_token(self, token: str) -> 'HttpClient':
        self._auth_token = token
        if API_KEY_HEADER.lower() != 'authorization':
            self._api_key = None
        self._update_auth_headers()
        return self

    def set_access_token(self, token: str) -> 'HttpClient':
        self._access_token = token
        if API_KEY_HEADER.lower() != 'access-token':
            self._api_key = None
        self._update_auth_headers()
        return self

    def set_header(self, key: str, value: str) -> 'HttpClient':
        self.headers[key] = value
        if self._session is not None:
            self._session.headers[key] = value
        return self
