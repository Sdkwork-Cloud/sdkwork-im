use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_control_plane_exposes_protocol_governance_snapshot_to_control_readers() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/protocol-governance")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-actor-kind", "user")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("protocol governance request should return a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("protocol governance body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("protocol governance body should be valid json");

    assert_eq!(
        json["capabilityProfile"]["profileId"],
        "control-plane-stable"
    );
    assert_eq!(
        json["quotaProfile"]["maxConcurrentSessionsPerTenant"],
        20_000
    );
    assert_eq!(json["rolloutPolicy"]["releaseChannel"], "stable");
    assert_eq!(json["rolloutPolicy"]["trafficPercent"], 100);
    assert_eq!(json["killSwitch"]["active"], true);
    assert_eq!(json["effectiveSnapshot"]["killSwitchActive"], true);

    let enabled_capabilities = json["effectiveSnapshot"]["enabledCapabilities"]
        .as_array()
        .expect("effective snapshot should return enabled capabilities");
    assert!(
        !enabled_capabilities
            .iter()
            .any(|value| value == "payload.cbor"),
        "effective snapshot should remove kill-switched capabilities"
    );

    let precedence = json["effectiveSnapshot"]["precedence"]
        .as_array()
        .expect("effective snapshot should expose precedence order");
    assert_eq!(
        precedence.first(),
        Some(&serde_json::json!("emergency_kill_switch"))
    );

    let sdk_compatibility_baseline = json["sdkCompatibilityBaseline"]
        .as_object()
        .expect("protocol governance should expose sdk compatibility baseline");
    assert_eq!(
        sdk_compatibility_baseline["appSdkFacade"],
        serde_json::json!("sdkwork-im-sdk")
    );
    assert_eq!(
        sdk_compatibility_baseline["adminSdkFacade"],
        serde_json::json!("sdkwork-control-plane-sdk")
    );
    assert_eq!(
        sdk_compatibility_baseline["protocolRegistryPath"],
        serde_json::json!("/api/v1/control/protocol-registry")
    );
    assert_eq!(
        sdk_compatibility_baseline["protocolGovernancePath"],
        serde_json::json!("/api/v1/control/protocol-governance")
    );
    assert_eq!(
        sdk_compatibility_baseline["matrixClientTypes"],
        serde_json::json!(["backend", "desktop", "mobile", "web"])
    );

    let business_policy_vocabulary = json["businessPolicyVocabulary"]
        .as_object()
        .expect("protocol governance should expose business policy vocabulary");
    assert_eq!(
        business_policy_vocabulary["policyVersionField"],
        serde_json::json!("policy_version")
    );
    assert_eq!(
        business_policy_vocabulary["capabilityFlagsField"],
        serde_json::json!("capability_flags")
    );
    assert_eq!(
        business_policy_vocabulary["historyVisibilityField"],
        serde_json::json!("history_visibility")
    );
    assert_eq!(
        business_policy_vocabulary["retentionPolicyRefField"],
        serde_json::json!("retention_policy_ref")
    );
    assert_eq!(
        business_policy_vocabulary["historyVisibilityModes"],
        serde_json::json!(["joined", "world_readable"])
    );
    assert_eq!(
        business_policy_vocabulary["retentionPolicyScopes"],
        serde_json::json!(["tenant", "space", "group", "channel", "thread"])
    );
}
