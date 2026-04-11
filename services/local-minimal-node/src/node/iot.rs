use super::*;

const DEVICE_SCOPE_KIND: &str = "device";
const DEVICE_TELEMETRY_STREAM_TYPE: &str = "device.telemetry";
const DEVICE_TELEMETRY_FRAME_TYPE: &str = "telemetry";
const DEVICE_TELEMETRY_SCHEMA_REF: &str = "cc.device.telemetry.v1";
const DEVICE_COMMAND_STREAM_TYPE: &str = "device.command";
const DEVICE_COMMAND_FRAME_TYPE: &str = "command";
const DEVICE_COMMAND_SCHEMA_REF: &str = "cc.device.command.v1";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IngestIotProtocolUplinkRequest {
    device_id: Option<String>,
    channel: String,
    payload: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IngestIotProtocolDownlinkRequest {
    device_id: String,
    channel: String,
    payload_json: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IotProtocolDownlinkResponse {
    frame: im_domain_core::stream::StreamFrame,
    protocol_payload: String,
}

pub(super) async fn get_iot_access_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_platform_contracts::ProviderHealthSnapshot>, ApiError> {
    let _auth = resolve_auth_context(&headers)?;
    Ok(Json(state.iot_access_provider_health()))
}

pub(super) async fn get_iot_protocol_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_platform_contracts::ProviderHealthSnapshot>, ApiError> {
    let _auth = resolve_auth_context(&headers)?;
    Ok(Json(state.iot_protocol_provider_health()))
}

pub(super) async fn ingest_iot_protocol_uplink(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<IngestIotProtocolUplinkRequest>,
) -> Result<Json<im_domain_core::stream::StreamFrame>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_iot_protocol_uplink_actor_preflight(&auth)?;
    let preflight_device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    access::ensure_iot_protocol_uplink_access(&state, &auth, preflight_device_id.as_str())?;

    let envelope = state.iot_protocol_adapter.decode_uplink(
        im_platform_contracts::IotProtocolDecodeRequest {
            tenant_id: auth.tenant_id.clone(),
            device_id: Some(preflight_device_id.clone()),
            channel: request.channel,
            payload: request.payload,
        },
    )?;

    access::ensure_iot_protocol_uplink_decoded_device_matches_preflight(
        preflight_device_id.as_str(),
        envelope.device_id.as_str(),
    )?;
    access::ensure_iot_protocol_uplink_access(&state, &auth, envelope.device_id.as_str())?;

    let stream_id = device_telemetry_stream_id(envelope.device_id.as_str());
    let session = state.streaming_runtime.open_stream(
        &auth,
        OpenStreamRequest {
            stream_id: stream_id.clone(),
            stream_type: DEVICE_TELEMETRY_STREAM_TYPE.into(),
            scope_kind: DEVICE_SCOPE_KIND.into(),
            scope_id: envelope.device_id.clone(),
            durability_class: "durableSession".into(),
            schema_ref: Some(DEVICE_TELEMETRY_SCHEMA_REF.into()),
        },
    )?;

    let frame = state.streaming_runtime.append_frame(
        &auth,
        session.stream_id.as_str(),
        AppendStreamFrameRequest {
            frame_seq: session.last_frame_seq + 1,
            frame_type: DEVICE_TELEMETRY_FRAME_TYPE.into(),
            schema_ref: Some(DEVICE_TELEMETRY_SCHEMA_REF.into()),
            encoding: "json".into(),
            payload: envelope.payload_json,
            attributes: envelope.attributes,
        },
    )?;
    effects::publish_realtime_stream_frame_event(&state, &auth, &frame)?;

    Ok(Json(frame))
}

pub(super) async fn ingest_iot_protocol_downlink(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<IngestIotProtocolDownlinkRequest>,
) -> Result<Json<IotProtocolDownlinkResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_iot_protocol_downlink_access(&state, &auth, request.device_id.as_str())?;

    let protocol_payload = state.iot_protocol_adapter.encode_downlink(
        im_platform_contracts::IotProtocolEncodeRequest {
            tenant_id: auth.tenant_id.clone(),
            device_id: request.device_id.clone(),
            channel: request.channel,
            payload_json: request.payload_json.clone(),
        },
    )?;

    let stream_id = device_command_stream_id(request.device_id.as_str());
    let session = state.streaming_runtime.open_stream(
        &auth,
        OpenStreamRequest {
            stream_id: stream_id.clone(),
            stream_type: DEVICE_COMMAND_STREAM_TYPE.into(),
            scope_kind: DEVICE_SCOPE_KIND.into(),
            scope_id: request.device_id.clone(),
            durability_class: "durableSession".into(),
            schema_ref: Some(DEVICE_COMMAND_SCHEMA_REF.into()),
        },
    )?;

    let frame = state.streaming_runtime.append_frame(
        &auth,
        session.stream_id.as_str(),
        AppendStreamFrameRequest {
            frame_seq: session.last_frame_seq + 1,
            frame_type: DEVICE_COMMAND_FRAME_TYPE.into(),
            schema_ref: Some(DEVICE_COMMAND_SCHEMA_REF.into()),
            encoding: "json".into(),
            payload: request.payload_json,
            attributes: BTreeMap::new(),
        },
    )?;
    effects::publish_realtime_stream_frame_event(&state, &auth, &frame)?;

    Ok(Json(IotProtocolDownlinkResponse {
        frame,
        protocol_payload,
    }))
}

fn device_telemetry_stream_id(device_id: &str) -> String {
    format!("st_device_telemetry_{device_id}")
}

fn device_command_stream_id(device_id: &str) -> String {
    format!("st_device_command_{device_id}")
}
