use super::*;

pub(super) async fn create_media_upload(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateUploadRequest>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.media_runtime.create_upload(&auth, request)?))
}

pub(super) async fn complete_media_upload(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteUploadRequest>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.media_runtime.complete_upload(
        &auth,
        media_asset_id.as_str(),
        request,
    )?))
}

pub(super) async fn get_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .media_runtime
            .get_asset(&auth, media_asset_id.as_str())?,
    ))
}

pub(super) async fn get_media_download_url(
    Path(media_asset_id): Path<String>,
    Query(query): Query<media_service::DownloadUrlQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<media_service::MediaDownloadUrlResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.media_runtime.download_url(
        &auth,
        media_asset_id.as_str(),
        query.expires_in_seconds.unwrap_or(3600),
    )?))
}

pub(super) async fn get_media_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_platform_contracts::ProviderHealthSnapshot>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .media_runtime
            .provider_health_snapshot(auth.tenant_id.as_str())?,
    ))
}

pub(super) async fn attach_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AttachMediaRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let asset = state
        .media_runtime
        .get_asset(&auth, media_asset_id.as_str())?;
    if asset.processing_state != MediaProcessingState::Ready {
        return Err(ApiError::bad_request(
            "media_asset_not_ready",
            format!("media asset is not ready to attach: {media_asset_id}"),
        ));
    }

    let body = effects::build_message_body(
        request.summary,
        request.text,
        vec![ContentPart::media(MediaPart {
            media_asset_id: asset.media_asset_id.clone(),
            resource: Some(asset.resource.clone()),
        })],
        request.render_hints,
    )?;

    let result = effects::post_message_with_side_effects(
        &state,
        &auth,
        request.conversation_id,
        request.client_msg_id,
        MessageType::Standard,
        body,
    )?;

    Ok(Json(result))
}
