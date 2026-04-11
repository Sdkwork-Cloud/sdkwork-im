use super::*;
use im_platform_contracts::DeviceTwinRecord;
use im_time::utc_now_rfc3339_millis;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateDeviceTwinDesiredRequest {
    desired_state_json: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateDeviceTwinReportedRequest {
    reported_state_json: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeviceTwinView {
    tenant_id: String,
    device_id: String,
    desired_state_json: String,
    reported_state_json: String,
    updated_at: String,
}

pub(super) async fn get_device_twin(
    Path(device_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DeviceTwinView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_device_twin_read_access(&state, &auth, device_id.as_str())?;

    let record = state
        .device_twin_store
        .load_twin(auth.tenant_id.as_str(), device_id.as_str())?
        .ok_or_else(|| {
            ApiError::not_found(
                "device_twin_not_found",
                format!("device twin not found: {device_id}"),
            )
        })?;

    Ok(Json(DeviceTwinView::from(record)))
}

pub(super) async fn update_device_twin_desired(
    Path(device_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateDeviceTwinDesiredRequest>,
) -> Result<Json<DeviceTwinView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_device_twin_desired_write_access(&state, &auth, device_id.as_str())?;

    let mut record =
        load_or_default_device_twin(&state, auth.tenant_id.as_str(), device_id.as_str())?;
    record.desired_state_json =
        normalize_device_twin_json("desiredStateJson", request.desired_state_json.as_str())?;
    record.updated_at = utc_now_rfc3339_millis();
    state.device_twin_store.save_twin(record.clone())?;

    Ok(Json(DeviceTwinView::from(record)))
}

pub(super) async fn update_device_twin_reported(
    Path(device_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateDeviceTwinReportedRequest>,
) -> Result<Json<DeviceTwinView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_device_twin_reported_write_access(&state, &auth, device_id.as_str())?;

    let mut record =
        load_or_default_device_twin(&state, auth.tenant_id.as_str(), device_id.as_str())?;
    record.reported_state_json =
        normalize_device_twin_json("reportedStateJson", request.reported_state_json.as_str())?;
    record.updated_at = utc_now_rfc3339_millis();
    state.device_twin_store.save_twin(record.clone())?;

    Ok(Json(DeviceTwinView::from(record)))
}

fn load_or_default_device_twin(
    state: &AppState,
    tenant_id: &str,
    device_id: &str,
) -> Result<DeviceTwinRecord, ApiError> {
    Ok(state
        .device_twin_store
        .load_twin(tenant_id, device_id)?
        .unwrap_or_else(|| DeviceTwinRecord {
            tenant_id: tenant_id.into(),
            device_id: device_id.into(),
            desired_state_json: "{}".into(),
            reported_state_json: "{}".into(),
            updated_at: utc_now_rfc3339_millis(),
        }))
}

fn normalize_device_twin_json(field_name: &str, raw: &str) -> Result<String, ApiError> {
    if raw.trim().is_empty() {
        return Err(ApiError::bad_request(
            "device_twin_state_invalid",
            format!("{field_name} must be non-empty valid json"),
        ));
    }

    let json: serde_json::Value = serde_json::from_str(raw).map_err(|error| {
        ApiError::bad_request(
            "device_twin_state_invalid",
            format!("{field_name} must be valid json: {error}"),
        )
    })?;
    Ok(json.to_string())
}

impl From<DeviceTwinRecord> for DeviceTwinView {
    fn from(value: DeviceTwinRecord) -> Self {
        Self {
            tenant_id: value.tenant_id,
            device_id: value.device_id,
            desired_state_json: value.desired_state_json,
            reported_state_json: value.reported_state_json,
            updated_at: value.updated_at,
        }
    }
}
