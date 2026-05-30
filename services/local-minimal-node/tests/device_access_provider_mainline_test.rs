use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{
    ContractError, DeviceAccessOwnerBindingRequest, DeviceAccessProvider, DeviceAccessRegistration,
    DeviceAccessRegistrationRequest, ProviderDomain, ProviderHealthSnapshot,
    ProviderPluginDescriptor,
};
use tower::ServiceExt;

static UNIQUE_RUNTIME_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let counter = UNIQUE_RUNTIME_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_device_access_provider_runtime_{unique}_{counter}"
    ))
}

#[derive(Debug, Default)]
struct RecordingDeviceAccessProviderState {
    register_requests: Vec<DeviceAccessRegistrationRequest>,
    bind_owner_requests: Vec<DeviceAccessOwnerBindingRequest>,
}

#[derive(Clone, Default)]
struct RecordingDeviceAccessProvider {
    state: Arc<Mutex<RecordingDeviceAccessProviderState>>,
}

impl RecordingDeviceAccessProvider {
    fn recorded_state(&self) -> RecordingDeviceAccessProviderState {
        let guard = self
            .state
            .lock()
            .expect("device access provider state should lock");
        RecordingDeviceAccessProviderState {
            register_requests: guard.register_requests.clone(),
            bind_owner_requests: guard.bind_owner_requests.clone(),
        }
    }
}

impl DeviceAccessProvider for RecordingDeviceAccessProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "iot-access-recording",
            ProviderDomain::IotAccess,
            "recording",
            "Recording Device Access",
        )
        .with_required_capabilities(["registry", "binding"])
    }

    fn register_device(
        &self,
        request: DeviceAccessRegistrationRequest,
    ) -> Result<DeviceAccessRegistration, ContractError> {
        self.state
            .lock()
            .expect("device access provider state should lock")
            .register_requests
            .push(request.clone());
        Ok(DeviceAccessRegistration {
            tenant_id: request.tenant_id,
            device_id: request.device_id,
            product_id: request.product_id,
            owner_principal_id: request.owner_principal_id,
            credential_secret: Some("recording-secret".into()),
            assigned_protocols: vec!["mqtt".into(), "xiaozhi".into()],
        })
    }

    fn bind_owner(&self, request: DeviceAccessOwnerBindingRequest) -> Result<bool, ContractError> {
        self.state
            .lock()
            .expect("device access provider state should lock")
            .bind_owner_requests
            .push(request);
        Ok(true)
    }

    fn disable_device(&self, _tenant_id: &str, _device_id: &str) -> Result<bool, ContractError> {
        Ok(true)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("iot-access-recording", "2026-04-08T00:00:00Z")
    }
}

#[tokio::test]
async fn test_device_register_route_calls_injected_device_access_provider() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let provider = RecordingDeviceAccessProvider::default();
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_device_access_provider(
        runtime_dir.as_path(),
        Arc::new(provider.clone()),
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/devices/register")
                .header("x-sdkwork-tenant-id", "t_device_provider")
                .header("x-sdkwork-user-id", "u_device_provider")
                .header("x-sdkwork-actor-kind", "device")
                .header("x-sdkwork-device-id", "d_device_provider")
                .header("x-sdkwork-session-id", "s_device_provider")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("device register request should succeed");
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("device register body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("device register response should be valid json");
    assert_eq!(json["tenantId"], "t_device_provider");
    assert_eq!(json["principalId"], "u_device_provider");
    assert_eq!(json["deviceId"], "d_device_provider");

    let recorded = provider.recorded_state();
    assert_eq!(recorded.register_requests.len(), 1);
    assert_eq!(
        recorded.register_requests[0],
        DeviceAccessRegistrationRequest {
            tenant_id: "t_device_provider".into(),
            device_id: "d_device_provider".into(),
            product_id: "local-minimal-device".into(),
            credential_kind: "device_route".into(),
            owner_principal_id: Some("u_device_provider".into()),
        }
    );
    assert_eq!(recorded.bind_owner_requests.len(), 1);
    assert_eq!(
        recorded.bind_owner_requests[0],
        DeviceAccessOwnerBindingRequest {
            tenant_id: "t_device_provider".into(),
            device_id: "d_device_provider".into(),
            owner_principal_id: "u_device_provider".into(),
            session_id: Some("s_device_provider".into()),
        }
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_conflicting_device_register_does_not_call_provider_for_second_owner() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let provider = RecordingDeviceAccessProvider::default();
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_device_access_provider(
        runtime_dir.as_path(),
        Arc::new(provider.clone()),
    );

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/devices/register")
                .header("x-sdkwork-tenant-id", "t_device_provider")
                .header("x-sdkwork-user-id", "u_owner_a")
                .header("x-sdkwork-actor-kind", "device")
                .header("x-sdkwork-device-id", "d_conflict_provider")
                .header("x-sdkwork-session-id", "s_owner_a")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("first owner device register request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/devices/register")
                .header("x-sdkwork-tenant-id", "t_device_provider")
                .header("x-sdkwork-user-id", "u_owner_b")
                .header("x-sdkwork-actor-kind", "device")
                .header("x-sdkwork-device-id", "d_conflict_provider")
                .header("x-sdkwork-session-id", "s_owner_b")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("conflicting device register request should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);

    let body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting register body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("conflicting register response should be valid json");
    assert_eq!(json["code"], "device_scope_conflict");

    let recorded = provider.recorded_state();
    assert_eq!(recorded.register_requests.len(), 1);
    assert_eq!(recorded.bind_owner_requests.len(), 1);

    let _ = fs::remove_dir_all(runtime_dir);
}
