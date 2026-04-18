from typing import Any, Dict, List, Optional
from ..http_client import HttpClient
from ..models import AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest, OpenStreamRequest, StreamFrame, StreamFrameWindow, StreamSession

class StreamApi:
    """stream API client."""
    
    def __init__(self, client: HttpClient):
        self._client = client

    def open(self, body: OpenStreamRequest) -> StreamSession:
        """Open a stream session"""
        return self._client.post(f"/api/v1/streams", json=body)

    def list_stream_frames(self, stream_id: str, params: Optional[Dict[str, Any]] = None) -> StreamFrameWindow:
        """List stream frames"""
        return self._client.get(f"/api/v1/streams/{stream_id}/frames", params=params)

    def append_stream_frame(self, stream_id: str, body: AppendStreamFrameRequest) -> StreamFrame:
        """Append a frame to a stream"""
        return self._client.post(f"/api/v1/streams/{stream_id}/frames", json=body)

    def checkpoint(self, stream_id: str, body: CheckpointStreamRequest) -> StreamSession:
        """Checkpoint a stream session"""
        return self._client.post(f"/api/v1/streams/{stream_id}/checkpoint", json=body)

    def complete(self, stream_id: str, body: CompleteStreamRequest) -> StreamSession:
        """Complete a stream session"""
        return self._client.post(f"/api/v1/streams/{stream_id}/complete", json=body)

    def abort(self, stream_id: str, body: AbortStreamRequest) -> StreamSession:
        """Abort a stream session"""
        return self._client.post(f"/api/v1/streams/{stream_id}/abort", json=body)
