use super::*;

pub(super) async fn create_media_upload(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateUploadRequest>,
) -> Result<Json<MediaUploadMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    let expires_in_seconds = request.expires_in_seconds;
    let request_key = media_create_upload_request_key(&auth, request.media_asset_id.as_str());
    let outcome = state
        .media_runtime
        .create_upload_with_outcome(&auth, request)?;
    let upload =
        state
            .media_runtime
            .prepare_upload_session(&auth, &outcome.asset, expires_in_seconds)?;
    Ok(Json(MediaUploadMutationResponse::from_outcome(
        outcome,
        request_key,
        Some(upload),
    )))
}

pub(super) async fn complete_media_upload(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteUploadRequest>,
) -> Result<Json<MediaUploadMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    let request_key = media_complete_upload_request_key(&auth, media_asset_id.as_str());
    Ok(Json(MediaUploadMutationResponse::from_outcome(
        state.media_runtime.complete_upload_with_outcome(
            &auth,
            media_asset_id.as_str(),
            request,
        )?,
        request_key,
        None,
    )))
}

pub(super) async fn get_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
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
    let auth = access::resolve_active_auth_context(&state, &headers)?;
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
    let auth = access::resolve_active_auth_context(&state, &headers)?;
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
    let auth = access::resolve_active_auth_context(&state, &headers)?;
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
