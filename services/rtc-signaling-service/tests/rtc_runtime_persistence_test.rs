use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use craw_chat_contract_core::ContractError;
use http_body_util::BodyExt;
use im_adapter_rtc_aliyun::AliyunRtcProvider;
use im_adapter_rtc_tencent::TencentRtcProvider;
use im_adapter_rtc_volcengine::VolcengineRtcProvider;
use im_adapters_local_memory::MemoryRtcStateStore;
use im_auth_context::AuthContext;
use im_platform_contracts::{
    ObjectStorageDownloadUrlRequest, ObjectStorageObjectDescriptor, ObjectStorageProvider,
    ObjectStoragePutRequest, ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
    RtcCallbackEvent, RtcCallbackRequest, RtcCreateSessionRequest, RtcParticipantCredential,
    RtcProviderPort, RtcRecordingArtifact, RtcSessionHandle, StaticProviderRegistry,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_runtime_restores_rtc_state_on_rebuild_with_shared_store() {
    let rtc_store = Arc::new(MemoryRtcStateStore::default());
    let app_before = rtc_signaling_service::build_app(Arc::new(
        rtc_signaling_service::RtcRuntime::with_store(rtc_store.clone()),
    ));

    let create_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_rebuild",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let invite_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/invite")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_rtc_rebuild"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite should succeed");
    assert_eq!(invite_response.status(), StatusCode::OK);

    let offer_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"offer\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("offer should succeed");
    assert_eq!(offer_response.status(), StatusCode::OK);

    let app_after = rtc_signaling_service::build_app(Arc::new(
        rtc_signaling_service::RtcRuntime::with_store(rtc_store),
    ));

    let accept_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("x-device-id", "d_peer")
                .header("x-session-id", "s_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_rebuild_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept after rebuild should return response");
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("accept should be valid json");
    assert_eq!(accept_json["state"], "accepted");
    assert_eq!(accept_json["signalingStreamId"], "st_rtc_rebuild");
    assert_eq!(accept_json["artifactMessageId"], "msg_rtc_rebuild_accept");

    let answer_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("x-device-id", "d_peer")
                .header("x-session-id", "s_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.answer",
                        "schemaRef":"webrtc.answer.v1",
                        "payload":"{\"sdp\":\"answer\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("answer after rebuild should return response");
    assert_eq!(answer_response.status(), StatusCode::OK);
    let answer_body = answer_response
        .into_body()
        .collect()
        .await
        .expect("answer body should collect")
        .to_bytes();
    let answer_json: serde_json::Value =
        serde_json::from_slice(&answer_body).expect("answer should be valid json");
    assert_eq!(answer_json["signalType"], "rtc.answer");
    assert_eq!(answer_json["signalingStreamId"], "st_rtc_rebuild");

    let end_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/end")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_rebuild_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end after rebuild should return response");
    assert_eq!(end_response.status(), StatusCode::OK);
}

#[derive(Clone, Default)]
struct TrackingRtcProvider {
    created_sessions: Arc<Mutex<Vec<String>>>,
    issued_credentials: Arc<Mutex<Vec<String>>>,
    closed_sessions: Arc<Mutex<Vec<String>>>,
}

#[derive(Clone)]
struct TrackingObjectStorageProvider {
    plugin_id: String,
    endpoint: String,
    signed_requests: Arc<Mutex<Vec<ObjectStorageDownloadUrlRequest>>>,
}

impl TrackingObjectStorageProvider {
    fn new(plugin_id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            endpoint: endpoint.into(),
            signed_requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn signed_requests(&self) -> Vec<ObjectStorageDownloadUrlRequest> {
        self.signed_requests
            .lock()
            .expect("tracking object storage provider should lock")
            .clone()
    }
}

impl ObjectStorageProvider for TrackingObjectStorageProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            self.plugin_id.clone(),
            ProviderDomain::ObjectStorage,
            "test",
            format!("{} Object Storage", self.plugin_id),
        )
        .with_required_capabilities(["s3", "presign"])
    }

    fn put_object(
        &self,
        request: ObjectStoragePutRequest,
    ) -> Result<ObjectStorageObjectDescriptor, ContractError> {
        Ok(ObjectStorageObjectDescriptor {
            bucket: request.bucket,
            object_key: request.object_key,
            content_length: request.content_length,
            etag: Some("etag-demo".into()),
        })
    }

    fn signed_download_url(
        &self,
        request: ObjectStorageDownloadUrlRequest,
    ) -> Result<String, ContractError> {
        self.signed_requests
            .lock()
            .expect("tracking object storage provider should lock")
            .push(request.clone());
        Ok(format!(
            "{}/{}/{}?provider={}&expires={}",
            self.endpoint.trim_end_matches('/'),
            request.bucket,
            request.object_key,
            self.plugin_id,
            request.expires_in_seconds
        ))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(self.plugin_id.clone(), "2026-04-08T00:00:00Z")
    }
}

impl TrackingRtcProvider {
    fn created_sessions(&self) -> Vec<String> {
        self.created_sessions
            .lock()
            .expect("tracking provider should lock")
            .clone()
    }

    fn issued_credentials(&self) -> Vec<String> {
        self.issued_credentials
            .lock()
            .expect("tracking provider should lock")
            .clone()
    }

    fn closed_sessions(&self) -> Vec<String> {
        self.closed_sessions
            .lock()
            .expect("tracking provider should lock")
            .clone()
    }
}

impl RtcProviderPort for TrackingRtcProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "rtc-volcengine",
            ProviderDomain::Rtc,
            "volcengine",
            "Volcengine RTC",
        )
        .with_default_selected(true)
        .with_required_capabilities(["session", "credential", "callback", "health"])
    }

    fn create_session(
        &self,
        request: RtcCreateSessionRequest,
    ) -> Result<RtcSessionHandle, ContractError> {
        self.created_sessions
            .lock()
            .expect("tracking provider should lock")
            .push(request.rtc_session_id.clone());
        Ok(RtcSessionHandle {
            tenant_id: request.tenant_id,
            rtc_session_id: request.rtc_session_id,
            provider_session_id: "volc-room-demo".into(),
            access_endpoint: Some("wss://rtc.volcengine.example/session".into()),
            region: Some("cn-beijing".into()),
        })
    }

    fn close_session(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError> {
        let _ = tenant_id;
        self.closed_sessions
            .lock()
            .expect("tracking provider should lock")
            .push(rtc_session_id.into());
        Ok(true)
    }

    fn issue_participant_credential(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
        participant_id: &str,
    ) -> Result<RtcParticipantCredential, ContractError> {
        let _ = tenant_id;
        self.issued_credentials
            .lock()
            .expect("tracking provider should lock")
            .push(format!("{rtc_session_id}:{participant_id}"));
        Ok(RtcParticipantCredential {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            participant_id: participant_id.into(),
            credential: "credential-demo".into(),
            expires_at: "2026-04-08T12:00:00Z".into(),
        })
    }

    fn refresh_participant_credential(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
        participant_id: &str,
    ) -> Result<RtcParticipantCredential, ContractError> {
        self.issue_participant_credential(tenant_id, rtc_session_id, participant_id)
    }

    fn map_provider_callback(
        &self,
        request: RtcCallbackRequest,
    ) -> Result<RtcCallbackEvent, ContractError> {
        Ok(RtcCallbackEvent {
            rtc_session_id: request.rtc_session_id,
            event_type: request.callback_type,
            participant_id: None,
            payload_json: request.payload_json,
        })
    }

    fn export_recording_artifact(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcRecordingArtifact>, ContractError> {
        Ok(Some(RtcRecordingArtifact {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            bucket: "rtc-artifacts".into(),
            object_key: "rtc/demo.mp4".into(),
            storage_provider: None,
            playback_url: None,
        }))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = BTreeMap::new();
        details.insert("providerKind".into(), "volcengine".into());
        ProviderHealthSnapshot {
            plugin_id: "rtc-volcengine".into(),
            status: "healthy".into(),
            checked_at: "2026-04-08T00:00:00Z".into(),
            details,
        }
    }
}

fn demo_auth_context() -> AuthContext {
    AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: Some("d_demo".into()),
        permissions: Default::default(),
    }
}

#[test]
fn test_runtime_routes_create_credential_and_end_through_selected_rtc_provider() {
    let rtc_store = Arc::new(MemoryRtcStateStore::default());
    let provider = Arc::new(TrackingRtcProvider::default());
    let descriptor = provider.descriptor();
    let runtime = rtc_signaling_service::RtcRuntime::with_store_and_provider_registry(
        rtc_store,
        Arc::new(StaticProviderRegistry::new([descriptor.clone()])),
        [(
            descriptor.plugin_id.clone(),
            provider.clone() as Arc<dyn RtcProviderPort>,
        )],
    );
    let auth = demo_auth_context();

    let session = runtime
        .create_session(
            &auth,
            rtc_signaling_service::CreateRtcSessionRequest {
                rtc_session_id: "rtc_provider_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
            },
        )
        .expect("rtc session should be created through provider");
    assert_eq!(session.rtc_session_id, "rtc_provider_demo");
    assert_eq!(
        session.provider_plugin_id.as_deref(),
        Some("rtc-volcengine")
    );
    assert_eq!(
        session.provider_session_id.as_deref(),
        Some("volc-room-demo")
    );
    assert_eq!(
        session.access_endpoint.as_deref(),
        Some("wss://rtc.volcengine.example/session")
    );
    assert_eq!(session.provider_region.as_deref(), Some("cn-beijing"));

    let credential = runtime
        .issue_participant_credential(&auth, "rtc_provider_demo", "u_peer")
        .expect("rtc credential should be issued through provider");
    assert_eq!(credential.participant_id, "u_peer");
    assert_eq!(credential.tenant_id, "t_demo");

    let health = runtime
        .provider_health_snapshot("t_demo")
        .expect("rtc provider health should be available");
    assert_eq!(health.plugin_id, "rtc-volcengine");

    runtime
        .end_session(
            &auth,
            "rtc_provider_demo",
            rtc_signaling_service::UpdateRtcSessionRequest {
                artifact_message_id: None,
            },
        )
        .expect("rtc session should close through provider");

    assert_eq!(provider.created_sessions(), vec!["rtc_provider_demo"]);
    assert_eq!(
        provider.issued_credentials(),
        vec!["rtc_provider_demo:u_peer"]
    );
    assert_eq!(provider.closed_sessions(), vec!["rtc_provider_demo"]);
}

#[test]
fn test_runtime_can_route_to_tenant_selected_builtin_rtc_providers() {
    let rtc_store = Arc::new(MemoryRtcStateStore::default());
    let registry = StaticProviderRegistry::platform_default()
        .with_tenant_override("t_aliyun", ProviderDomain::Rtc, "rtc-aliyun")
        .with_tenant_override("t_tencent", ProviderDomain::Rtc, "rtc-tencent");
    let runtime = rtc_signaling_service::RtcRuntime::with_store_and_provider_registry(
        rtc_store,
        Arc::new(registry),
        [
            (
                "rtc-volcengine".into(),
                Arc::new(VolcengineRtcProvider::default()) as Arc<dyn RtcProviderPort>,
            ),
            (
                "rtc-aliyun".into(),
                Arc::new(AliyunRtcProvider::default()) as Arc<dyn RtcProviderPort>,
            ),
            (
                "rtc-tencent".into(),
                Arc::new(TencentRtcProvider::default()) as Arc<dyn RtcProviderPort>,
            ),
        ],
    );

    let mut aliyun_auth = demo_auth_context();
    aliyun_auth.tenant_id = "t_aliyun".into();
    let aliyun_session = runtime
        .create_session(
            &aliyun_auth,
            rtc_signaling_service::CreateRtcSessionRequest {
                rtc_session_id: "rtc_aliyun_demo".into(),
                conversation_id: None,
                rtc_mode: "voice".into(),
            },
        )
        .expect("aliyun rtc session should be created");
    assert_eq!(
        aliyun_session.provider_plugin_id.as_deref(),
        Some("rtc-aliyun")
    );
    assert_eq!(
        aliyun_session.provider_session_id.as_deref(),
        Some("aliyun:rtc_aliyun_demo")
    );

    let mut tencent_auth = demo_auth_context();
    tencent_auth.tenant_id = "t_tencent".into();
    let tencent_session = runtime
        .create_session(
            &tencent_auth,
            rtc_signaling_service::CreateRtcSessionRequest {
                rtc_session_id: "rtc_tencent_demo".into(),
                conversation_id: None,
                rtc_mode: "voice".into(),
            },
        )
        .expect("tencent rtc session should be created");
    assert_eq!(
        tencent_session.provider_plugin_id.as_deref(),
        Some("rtc-tencent")
    );
    assert_eq!(
        tencent_session.provider_session_id.as_deref(),
        Some("tencent:rtc_tencent_demo")
    );
}

#[test]
fn test_runtime_signs_recording_artifact_through_selected_object_storage_provider() {
    let rtc_store = Arc::new(MemoryRtcStateStore::default());
    let rtc_provider = Arc::new(TrackingRtcProvider::default());
    let rtc_descriptor = rtc_provider.descriptor();
    let aws_storage = Arc::new(TrackingObjectStorageProvider::new(
        "object-storage-aws",
        "https://storage.aws.local",
    ));
    let volcengine_storage = Arc::new(TrackingObjectStorageProvider::new(
        "object-storage-volcengine",
        "https://tos.volcengine.local",
    ));
    let registry = StaticProviderRegistry::platform_default()
        .with_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
        .with_tenant_override(
            "t_storage_aws",
            ProviderDomain::ObjectStorage,
            "object-storage-aws",
        );
    let runtime =
        rtc_signaling_service::RtcRuntime::with_store_and_provider_registry_and_object_storage_providers(
            rtc_store,
            Arc::new(registry),
            [(
                rtc_descriptor.plugin_id.clone(),
                rtc_provider.clone() as Arc<dyn RtcProviderPort>,
            )],
            [
                (
                    "object-storage-aws".into(),
                    aws_storage.clone() as Arc<dyn ObjectStorageProvider>,
                ),
                (
                    "object-storage-volcengine".into(),
                    volcengine_storage.clone() as Arc<dyn ObjectStorageProvider>,
                ),
            ],
        );

    let mut aws_auth = demo_auth_context();
    aws_auth.tenant_id = "t_storage_aws".into();
    runtime
        .create_session(
            &aws_auth,
            rtc_signaling_service::CreateRtcSessionRequest {
                rtc_session_id: "rtc_recording_storage_aws".into(),
                conversation_id: None,
                rtc_mode: "voice".into(),
            },
        )
        .expect("rtc session should be created");
    let aws_artifact = runtime
        .recording_artifact(&aws_auth, "rtc_recording_storage_aws")
        .expect("aws rtc recording artifact should be signed");
    assert_eq!(aws_artifact.bucket, "rtc-artifacts");
    assert_eq!(aws_artifact.object_key, "rtc/demo.mp4");
    assert_eq!(
        aws_artifact.storage_provider.as_deref(),
        Some("object-storage-aws")
    );
    assert_eq!(
        aws_artifact.playback_url.as_deref(),
        Some(
            "https://storage.aws.local/rtc-artifacts/rtc/demo.mp4?provider=object-storage-aws&expires=3600"
        )
    );

    let default_auth = demo_auth_context();
    runtime
        .create_session(
            &default_auth,
            rtc_signaling_service::CreateRtcSessionRequest {
                rtc_session_id: "rtc_recording_storage_default".into(),
                conversation_id: None,
                rtc_mode: "voice".into(),
            },
        )
        .expect("rtc session should be created");
    let default_artifact = runtime
        .recording_artifact(&default_auth, "rtc_recording_storage_default")
        .expect("default rtc recording artifact should be signed");
    assert_eq!(
        default_artifact.storage_provider.as_deref(),
        Some("object-storage-volcengine")
    );
    assert_eq!(
        default_artifact.playback_url.as_deref(),
        Some(
            "https://tos.volcengine.local/rtc-artifacts/rtc/demo.mp4?provider=object-storage-volcengine&expires=3600"
        )
    );

    assert_eq!(
        aws_storage.signed_requests(),
        vec![ObjectStorageDownloadUrlRequest {
            bucket: "rtc-artifacts".into(),
            object_key: "rtc/demo.mp4".into(),
            expires_in_seconds: 3600,
        }]
    );
    assert_eq!(
        volcengine_storage.signed_requests(),
        vec![ObjectStorageDownloadUrlRequest {
            bucket: "rtc-artifacts".into(),
            object_key: "rtc/demo.mp4".into(),
            expires_in_seconds: 3600,
        }]
    );
}
