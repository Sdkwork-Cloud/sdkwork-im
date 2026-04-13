use std::sync::Arc;
use std::sync::Mutex;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{
    ContractError, DeviceAccessOwnerBindingRequest, DeviceAccessProvider, DeviceAccessRegistration,
    DeviceAccessRegistrationRequest, ProviderDomain, ProviderHealthSnapshot,
    ProviderPluginDescriptor,
};
use tower::ServiceExt;

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
        ProviderHealthSnapshot::healthy("iot-access-recording", "2026-04-09T00:00:00Z")
    }
}

#[tokio::test]
async fn test_session_resume_and_heartbeat_call_injected_device_access_provider_once() {
    let provider = RecordingDeviceAccessProvider::default();
    let app = session_gateway::build_app_with_device_access_provider(Arc::new(provider.clone()));

    let resume_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_device_provider")
                .header("x-user-id", "u_device_provider")
                .header("x-session-id", "s_device_provider")
                .header("x-device-id", "d_device_provider")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume_response.status(), StatusCode::OK);

    let resume_body = resume_response
        .into_body()
        .collect()
        .await
        .expect("resume body should collect")
        .to_bytes();
    let resume_json: serde_json::Value =
        serde_json::from_slice(&resume_body).expect("resume response should be valid json");
    assert_eq!(resume_json["deviceId"], "d_device_provider");

    let heartbeat_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_device_provider")
                .header("x-user-id", "u_device_provider")
                .header("x-session-id", "s_device_provider")
                .header("x-device-id", "d_device_provider")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("heartbeat request should succeed");
    assert_eq!(heartbeat_response.status(), StatusCode::OK);

    let recorded = provider.recorded_state();
    assert_eq!(recorded.register_requests.len(), 1);
    assert_eq!(
        recorded.register_requests[0],
        DeviceAccessRegistrationRequest {
            tenant_id: "t_device_provider".into(),
            device_id: "d_device_provider".into(),
            product_id: "session-gateway-device".into(),
            credential_kind: "session".into(),
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
}

#[tokio::test]
async fn test_conflicting_session_resume_does_not_call_provider_for_second_owner() {
    let provider = RecordingDeviceAccessProvider::default();
    let app = session_gateway::build_app_with_device_access_provider(Arc::new(provider.clone()));

    let first_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_device_provider")
                .header("x-user-id", "u_owner_a")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_owner_a")
                .header("x-device-id", "d_conflict_provider")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("first owner resume should succeed");
    assert_eq!(first_resume.status(), StatusCode::OK);

    let conflicting_resume = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_device_provider")
                .header("x-user-id", "u_owner_b")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_owner_b")
                .header("x-device-id", "d_conflict_provider")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("conflicting owner resume should return response");
    assert_eq!(conflicting_resume.status(), StatusCode::CONFLICT);

    let body = conflicting_resume
        .into_body()
        .collect()
        .await
        .expect("conflicting owner resume body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("conflicting owner resume should be valid json");
    assert_eq!(json["code"], "device_scope_conflict");

    let recorded = provider.recorded_state();
    assert_eq!(recorded.register_requests.len(), 1);
    assert_eq!(recorded.bind_owner_requests.len(), 1);
}
