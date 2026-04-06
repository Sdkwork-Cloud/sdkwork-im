use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore};
use tower::ServiceExt;

#[derive(Clone)]
struct FailingCheckpointStore {
    fail_on_load: bool,
    fail_on_save: bool,
}

impl RealtimeCheckpointStore for FailingCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        if self.fail_on_load {
            return Err(ContractError::Unavailable(
                "synthetic checkpoint load failure".into(),
            ));
        }
        Ok(None)
    }

    fn save_checkpoint(&self, _record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        if self.fail_on_save {
            return Err(ContractError::Unavailable(
                "synthetic checkpoint save failure".into(),
            ));
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_realtime_events_returns_503_when_checkpoint_store_load_fails() {
    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        Arc::new(
            session_gateway::RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
                FailingCheckpointStore {
                    fail_on_load: true,
                    fail_on_save: false,
                },
            )),
        ),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should return a response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(value["code"], "checkpoint_store_unavailable");
}

#[tokio::test]
async fn test_realtime_ack_returns_503_when_checkpoint_store_save_fails() {
    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        Arc::new(
            session_gateway::RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
                FailingCheckpointStore {
                    fail_on_load: false,
                    fail_on_save: true,
                },
            )),
        ),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/events/ack")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_pad")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("request should return a response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(value["code"], "checkpoint_store_unavailable");
}
