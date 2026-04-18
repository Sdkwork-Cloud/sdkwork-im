from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AttachMediaRequest, CompleteUploadRequest, CreateUploadRequest, MediaAsset, MediaDownloadUrlResponse, PostMessageResult

class MediaApi:
    """media API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def create_media_upload(self, body: CreateUploadRequest) -> MediaAsset:
        """Create a media upload record"""
        return self._client.post(f"/api/v1/media/uploads", json=body)

    def complete_media_upload(self, media_asset_id: str, body: CompleteUploadRequest) -> MediaAsset:
        """Complete a media upload"""
        return self._client.post(f"/api/v1/media/uploads/{media_asset_id}/complete", json=body)

    def get_media_download_url(self, media_asset_id: str, params: Optional[Dict[str, Any]] = None) -> MediaDownloadUrlResponse:
        """Issue a signed media download URL"""
        return self._client.get(f"/api/v1/media/{media_asset_id}/download-url", params=params)

    def get_media_asset(self, media_asset_id: str) -> MediaAsset:
        """Get a media asset by id"""
        return self._client.get(f"/api/v1/media/{media_asset_id}")

    def attach_media_asset(self, media_asset_id: str, body: AttachMediaRequest) -> PostMessageResult:
        """Attach a ready media asset as a conversation message"""
        return self._client.post(f"/api/v1/media/{media_asset_id}/attach", json=body)
