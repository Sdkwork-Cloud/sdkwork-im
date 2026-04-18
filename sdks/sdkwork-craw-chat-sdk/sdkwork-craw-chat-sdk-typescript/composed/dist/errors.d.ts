export type CrawChatSdkErrorCode = 'backend_client_missing' | 'backend_client_resolution_failed' | 'api_base_url_required' | 'device_id_required' | 'media_asset_id_required' | 'media_reference_required' | 'media_upload_session_invalid' | 'upload_fetch_required' | 'upload_request_failed' | 'websocket_url_required' | 'websocket_factory_required' | 'websocket_not_connected' | 'websocket_closed' | 'websocket_request_timeout' | 'websocket_transport_error' | 'websocket_protocol_error' | 'ccp_auth_required';
export declare class CrawChatSdkError extends Error {
    readonly code: CrawChatSdkErrorCode;
    readonly details?: Record<string, unknown> | undefined;
    readonly name = "CrawChatSdkError";
    constructor(code: CrawChatSdkErrorCode, message: string, details?: Record<string, unknown> | undefined);
}
//# sourceMappingURL=errors.d.ts.map