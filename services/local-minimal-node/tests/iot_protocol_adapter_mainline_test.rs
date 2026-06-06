use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::request::Builder as RequestBuilder;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{
    ContractError, IotProtocolAdapter, IotProtocolDecodeRequest, IotProtocolEncodeRequest,
    IotProtocolEnvelope, ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
};
use tower::ServiceExt;

async fn json_body(response: axum::response::Response) -> serde_json::Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response body should be valid json")
}

fn device_actor(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "device")
        .header("x-sdkwork-device-id", "d_sensor")
        .header("x-sdkwork-session-id", "s_sensor")
}

fn owner_actor(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "user")
        .header("x-sdkwork-device-id", "d_console")
        .header("x-sdkwork-session-id", "s_console")
}

fn owner_actor_without_device(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "user")
        .header("x-sdkwork-session-id", "s_console")
}

#[tokio::test]
async fn test_local_minimal_profile_iot_protocol_uplink_enters_device_telemetry_mainline() {
    let app = local_minimal_node::build_default_app();

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let ingest_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/uplink"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "channel":"devices/d_sensor/telemetry",
                    "payload":"{\"temperature\":21.5}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol uplink request should return response");

    assert_eq!(ingest_response.status(), StatusCode::OK);
    let ingest_json = json_body(ingest_response).await;
    assert_eq!(ingest_json["streamType"], "device.telemetry");
    assert_eq!(ingest_json["scopeKind"], "device");
    assert_eq!(ingest_json["scopeId"], "d_sensor");
    assert_eq!(ingest_json["frameSeq"], 1);
    assert_eq!(ingest_json["frameType"], "telemetry");
    assert_eq!(ingest_json["schemaRef"], "cc.device.telemetry.v1");
    assert_eq!(ingest_json["sender"]["id"], "d_sensor");
    assert_eq!(ingest_json["sender"]["kind"], "device");
    assert_eq!(ingest_json["attributes"]["protocol"], "mqtt");
    assert_eq!(
        ingest_json["attributes"]["topic"],
        "devices/d_sensor/telemetry"
    );

    let stream_id = ingest_json["streamId"]
        .as_str()
        .expect("telemetry stream id should exist");

    let list_response = app
        .clone()
        .oneshot(
            owner_actor(Request::builder().uri(format!(
                "/im/v3/api/streams/{stream_id}/frames?afterFrameSeq=0&limit=10"
            )))
            .header("x-sdkwork-permission-scope", "device.telemetry.read")
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .expect("telemetry stream list should return response");

    assert_eq!(list_response.status(), StatusCode::OK);
    let list_json = json_body(list_response).await;
    let items = list_json["items"]
        .as_array()
        .expect("telemetry items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["payload"], "{\"temperature\":21.5}");
    assert_eq!(items[0]["attributes"]["protocol"], "mqtt");
}

#[derive(Clone, Default)]
struct RecordingIotProtocolAdapter {
    recorded_requests: Arc<Mutex<Vec<IotProtocolDecodeRequest>>>,
    recorded_downlink_requests: Arc<Mutex<Vec<IotProtocolEncodeRequest>>>,
}

impl RecordingIotProtocolAdapter {
    fn recorded_requests(&self) -> Vec<IotProtocolDecodeRequest> {
        self.recorded_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .clone()
    }

    fn recorded_downlink_requests(&self) -> Vec<IotProtocolEncodeRequest> {
        self.recorded_downlink_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .clone()
    }
}

impl IotProtocolAdapter for RecordingIotProtocolAdapter {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "iot-protocol-recording",
            ProviderDomain::IotProtocol,
            "recording",
            "Recording IoT Protocol",
        )
        .with_required_capabilities(["uplink"])
    }

    fn protocol_key(&self) -> &'static str {
        "recording"
    }

    fn decode_uplink(
        &self,
        request: IotProtocolDecodeRequest,
    ) -> Result<IotProtocolEnvelope, ContractError> {
        self.recorded_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .push(request.clone());

        Ok(IotProtocolEnvelope {
            tenant_id: request.tenant_id,
            device_id: request
                .device_id
                .expect("test request should include device id in auth context"),
            channel: request.channel.clone(),
            payload_json: "{\"normalized\":true}".into(),
            attributes: BTreeMap::from([
                ("protocol".into(), "recording".into()),
                ("topic".into(), request.channel),
            ]),
        })
    }

    fn encode_downlink(&self, request: IotProtocolEncodeRequest) -> Result<String, ContractError> {
        self.recorded_downlink_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .push(request.clone());

        Ok(format!(
            "encoded:{}:{}:{}",
            request.device_id, request.channel, request.payload_json
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("iot-protocol-recording", "2026-04-09T00:00:00Z")
    }
}

#[derive(Clone, Default)]
struct PayloadInferringIotProtocolAdapter {
    recorded_requests: Arc<Mutex<Vec<IotProtocolDecodeRequest>>>,
}

impl PayloadInferringIotProtocolAdapter {
    fn recorded_requests(&self) -> Vec<IotProtocolDecodeRequest> {
        self.recorded_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .clone()
    }
}

impl IotProtocolAdapter for PayloadInferringIotProtocolAdapter {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "iot-protocol-payload-inferring",
            ProviderDomain::IotProtocol,
            "payload-inferring",
            "Payload Inferring IoT Protocol",
        )
        .with_required_capabilities(["uplink"])
    }

    fn protocol_key(&self) -> &'static str {
        "payload-inferring"
    }

    fn decode_uplink(
        &self,
        request: IotProtocolDecodeRequest,
    ) -> Result<IotProtocolEnvelope, ContractError> {
        self.recorded_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .push(request.clone());

        let payload = serde_json::from_str::<serde_json::Value>(&request.payload)
            .expect("payload-inferring test adapter expects json payload");
        let device_id = payload["deviceId"]
            .as_str()
            .expect("payload-inferring test adapter expects payload.deviceId")
            .to_owned();

        Ok(IotProtocolEnvelope {
            tenant_id: request.tenant_id,
            device_id,
            channel: request.channel.clone(),
            payload_json: "{\"normalized\":true}".into(),
            attributes: BTreeMap::from([
                ("protocol".into(), "payload-inferring".into()),
                ("topic".into(), request.channel),
            ]),
        })
    }

    fn encode_downlink(&self, _request: IotProtocolEncodeRequest) -> Result<String, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "payload-inferring adapter does not implement downlink".into(),
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("iot-protocol-payload-inferring", "2026-04-09T00:00:00Z")
    }
}

#[derive(Clone, Default)]
struct MismatchingEnvelopeIotProtocolAdapter {
    recorded_requests: Arc<Mutex<Vec<IotProtocolDecodeRequest>>>,
}

impl MismatchingEnvelopeIotProtocolAdapter {
    fn recorded_requests(&self) -> Vec<IotProtocolDecodeRequest> {
        self.recorded_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .clone()
    }
}

impl IotProtocolAdapter for MismatchingEnvelopeIotProtocolAdapter {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "iot-protocol-mismatching-envelope",
            ProviderDomain::IotProtocol,
            "mismatching-envelope",
            "Mismatching Envelope IoT Protocol",
        )
        .with_required_capabilities(["uplink"])
    }

    fn protocol_key(&self) -> &'static str {
        "mismatching-envelope"
    }

    fn decode_uplink(
        &self,
        request: IotProtocolDecodeRequest,
    ) -> Result<IotProtocolEnvelope, ContractError> {
        self.recorded_requests
            .lock()
            .expect("iot protocol adapter state should lock")
            .push(request.clone());

        Ok(IotProtocolEnvelope {
            tenant_id: request.tenant_id,
            device_id: "d_other".into(),
            channel: request.channel.clone(),
            payload_json: "{\"normalized\":true}".into(),
            attributes: BTreeMap::from([
                ("protocol".into(), "mismatching-envelope".into()),
                ("topic".into(), request.channel),
            ]),
        })
    }

    fn encode_downlink(&self, _request: IotProtocolEncodeRequest) -> Result<String, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "mismatching-envelope adapter does not implement downlink".into(),
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("iot-protocol-mismatching-envelope", "2026-04-09T00:00:00Z")
    }
}

#[tokio::test]
async fn test_iot_protocol_uplink_route_uses_injected_iot_protocol_adapter() {
    let adapter = RecordingIotProtocolAdapter::default();
    let app =
        local_minimal_node::build_default_app_with_iot_protocol_adapter(Arc::new(adapter.clone()));

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let ingest_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/uplink"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "channel":"devices/d_sensor/telemetry",
                    "payload":"{\"raw\":1}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol uplink request should return response");

    assert_eq!(ingest_response.status(), StatusCode::OK);
    let ingest_json = json_body(ingest_response).await;
    assert_eq!(ingest_json["payload"], "{\"normalized\":true}");
    assert_eq!(ingest_json["attributes"]["protocol"], "recording");

    let recorded_requests = adapter.recorded_requests();
    assert_eq!(recorded_requests.len(), 1);
    assert_eq!(recorded_requests[0].tenant_id, "t_demo");
    assert_eq!(recorded_requests[0].device_id.as_deref(), Some("d_sensor"));
    assert_eq!(recorded_requests[0].channel, "devices/d_sensor/telemetry");
    assert_eq!(recorded_requests[0].payload, "{\"raw\":1}");
}

#[tokio::test]
async fn test_local_minimal_profile_iot_protocol_downlink_enters_device_command_mainline() {
    let app = local_minimal_node::build_default_app();

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let downlink_response = app
        .clone()
        .oneshot(
            owner_actor(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/downlink"),
            )
            .header("x-sdkwork-permission-scope", "device.command.send")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "deviceId":"d_sensor",
                    "channel":"devices/d_sensor/commands",
                    "payloadJson":"{\"command\":\"lock\"}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol downlink request should return response");

    assert_eq!(downlink_response.status(), StatusCode::OK);
    let downlink_json = json_body(downlink_response).await;
    assert_eq!(downlink_json["frame"]["streamType"], "device.command");
    assert_eq!(downlink_json["frame"]["scopeKind"], "device");
    assert_eq!(downlink_json["frame"]["scopeId"], "d_sensor");
    assert_eq!(downlink_json["frame"]["frameSeq"], 1);
    assert_eq!(downlink_json["frame"]["frameType"], "command");
    assert_eq!(downlink_json["frame"]["schemaRef"], "cc.device.command.v1");
    assert_eq!(downlink_json["frame"]["sender"]["id"], "u_owner");
    assert_eq!(downlink_json["frame"]["sender"]["kind"], "user");

    let protocol_payload = serde_json::from_str::<serde_json::Value>(
        downlink_json["protocolPayload"]
            .as_str()
            .expect("protocol payload should be returned as string"),
    )
    .expect("default adapter should return json protocol payload");
    assert_eq!(protocol_payload["protocol"], "mqtt");
    assert_eq!(protocol_payload["topic"], "devices/d_sensor/commands");
    assert_eq!(protocol_payload["deviceId"], "d_sensor");
    assert_eq!(protocol_payload["payload"]["command"], "lock");

    let stream_id = downlink_json["frame"]["streamId"]
        .as_str()
        .expect("command stream id should exist");

    let list_response = app
        .clone()
        .oneshot(
            device_actor(Request::builder().uri(format!(
                "/im/v3/api/streams/{stream_id}/frames?afterFrameSeq=0&limit=10"
            )))
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .expect("device command stream list should return response");

    assert_eq!(list_response.status(), StatusCode::OK);
    let list_json = json_body(list_response).await;
    let items = list_json["items"]
        .as_array()
        .expect("command items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["payload"], "{\"command\":\"lock\"}");
}

#[tokio::test]
async fn test_iot_protocol_downlink_route_uses_injected_iot_protocol_adapter() {
    let adapter = RecordingIotProtocolAdapter::default();
    let app =
        local_minimal_node::build_default_app_with_iot_protocol_adapter(Arc::new(adapter.clone()));

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let downlink_response = app
        .clone()
        .oneshot(
            owner_actor(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/downlink"),
            )
            .header("x-sdkwork-permission-scope", "device.command.send")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "deviceId":"d_sensor",
                    "channel":"devices/d_sensor/commands",
                    "payloadJson":"{\"command\":\"unlock\"}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol downlink request should return response");

    assert_eq!(downlink_response.status(), StatusCode::OK);
    let downlink_json = json_body(downlink_response).await;
    assert_eq!(
        downlink_json["protocolPayload"],
        "encoded:d_sensor:devices/d_sensor/commands:{\"command\":\"unlock\"}"
    );
    assert_eq!(
        downlink_json["frame"]["payload"],
        "{\"command\":\"unlock\"}"
    );

    let recorded_requests = adapter.recorded_downlink_requests();
    assert_eq!(recorded_requests.len(), 1);
    assert_eq!(recorded_requests[0].tenant_id, "t_demo");
    assert_eq!(recorded_requests[0].device_id, "d_sensor");
    assert_eq!(recorded_requests[0].channel, "devices/d_sensor/commands");
    assert_eq!(
        recorded_requests[0].payload_json,
        "{\"command\":\"unlock\"}"
    );
}

#[tokio::test]
async fn test_iot_protocol_uplink_known_device_preflight_rejects_before_adapter_decode() {
    let adapter = RecordingIotProtocolAdapter::default();
    let app =
        local_minimal_node::build_default_app_with_iot_protocol_adapter(Arc::new(adapter.clone()));

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let ingest_response = app
        .clone()
        .oneshot(
            owner_actor_without_device(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/uplink"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "deviceId":"d_sensor",
                    "channel":"devices/d_sensor/telemetry",
                    "payload":"{\"raw\":1}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol uplink request should return response");

    assert_eq!(ingest_response.status(), StatusCode::FORBIDDEN);
    let ingest_json = json_body(ingest_response).await;
    assert_eq!(ingest_json["code"], "device_permission_denied");
    assert_eq!(adapter.recorded_requests().len(), 0);
}

#[tokio::test]
async fn test_iot_protocol_uplink_non_device_actor_without_request_device_id_rejects_before_decode()
{
    let adapter = PayloadInferringIotProtocolAdapter::default();
    let app =
        local_minimal_node::build_default_app_with_iot_protocol_adapter(Arc::new(adapter.clone()));

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let ingest_response = app
        .clone()
        .oneshot(
            owner_actor_without_device(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/uplink"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "channel":"devices/d_sensor/telemetry",
                    "payload":"{\"deviceId\":\"d_sensor\",\"raw\":1}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol uplink request should return response");

    assert_eq!(ingest_response.status(), StatusCode::FORBIDDEN);
    let ingest_json = json_body(ingest_response).await;
    assert_eq!(ingest_json["code"], "device_permission_denied");
    assert_eq!(adapter.recorded_requests().len(), 0);
}

#[tokio::test]
async fn test_iot_protocol_uplink_request_device_mismatch_rejects_before_adapter_decode() {
    let adapter = RecordingIotProtocolAdapter::default();
    let app =
        local_minimal_node::build_default_app_with_iot_protocol_adapter(Arc::new(adapter.clone()));

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let ingest_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/uplink"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "deviceId":"d_other",
                    "channel":"devices/d_other/telemetry",
                    "payload":"{\"raw\":1}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol uplink request should return response");

    assert_eq!(ingest_response.status(), StatusCode::BAD_REQUEST);
    let ingest_json = json_body(ingest_response).await;
    assert_eq!(ingest_json["code"], "device_id_mismatch");
    assert_eq!(adapter.recorded_requests().len(), 0);
}

#[tokio::test]
async fn test_iot_protocol_uplink_decoded_device_mismatch_returns_bad_request_after_decode() {
    let adapter = MismatchingEnvelopeIotProtocolAdapter::default();
    let app =
        local_minimal_node::build_default_app_with_iot_protocol_adapter(Arc::new(adapter.clone()));

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let ingest_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/iot/protocol/uplink"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "channel":"devices/d_sensor/telemetry",
                    "payload":"{\"raw\":1}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("iot protocol uplink request should return response");

    assert_eq!(ingest_response.status(), StatusCode::BAD_REQUEST);
    let ingest_json = json_body(ingest_response).await;
    assert_eq!(ingest_json["code"], "device_id_mismatch");
    assert_eq!(adapter.recorded_requests().len(), 1);
}
