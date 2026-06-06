use im_adapter_rtc_tencent::{TENCENT_RTC_PLUGIN_ID, TencentRtcProvider, TencentRtcProviderConfig};
use im_platform_contracts::{RtcCallbackRequest, RtcCreateSessionRequest, RtcProviderPort};

#[test]
fn test_tencent_rtc_provider_implements_contract_surface() {
    let provider = TencentRtcProvider::new(TencentRtcProviderConfig {
        access_endpoint: "wss://rtc.tencent.local/session".into(),
        region: "ap-guangzhou".into(),
    });

    let descriptor = provider.descriptor();
    assert_eq!(descriptor.plugin_id, TENCENT_RTC_PLUGIN_ID);
    assert_eq!(descriptor.provider_kind, "tencent");
    assert_eq!(
        descriptor.required_capabilities,
        vec!["session", "credential", "callback", "health"]
    );
    assert_eq!(
        descriptor.optional_capabilities,
        vec!["recording", "artifact"]
    );

    let session = provider
        .create_session(RtcCreateSessionRequest {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            conversation_id: Some("c_demo".into()),
            rtc_mode: "voice".into(),
            initiator_id: "u_demo".into(),
        })
        .expect("tencent rtc create_session should succeed");
    assert_eq!(session.provider_session_id, "tencent:rtc_demo");
    assert_eq!(
        session.access_endpoint.as_deref(),
        Some("wss://rtc.tencent.local/session")
    );
    assert_eq!(session.region.as_deref(), Some("ap-guangzhou"));

    let credential = provider
        .issue_participant_credential("t_demo", "rtc_demo", "u_peer")
        .expect("tencent rtc credential should succeed");
    assert_eq!(
        credential.credential,
        "tencent-token:t_demo:rtc_demo:u_peer"
    );

    let callback = provider
        .map_provider_callback(RtcCallbackRequest {
            rtc_session_id: "rtc_demo".into(),
            callback_type: "room-ended".into(),
            payload_json: "{\"reason\":\"host_left\"}".into(),
        })
        .expect("tencent rtc callback mapping should succeed");
    assert_eq!(callback.event_type, "room-ended");

    let artifact = provider
        .export_recording_artifact("t_demo", "rtc_demo")
        .expect("tencent rtc artifact export should succeed")
        .expect("tencent rtc artifact should exist");
    assert_eq!(
        artifact.drive.drive_uri,
        "drive://spaces/space_rtc_recordings/nodes/node_rtc_demo"
    );
    assert_eq!(
        artifact.resource.uri.as_deref(),
        Some("drive://spaces/space_rtc_recordings/nodes/node_rtc_demo")
    );
    assert_eq!(artifact.media_role, "rtc_recording");

    let health = provider.provider_health_snapshot();
    assert_eq!(health.plugin_id, TENCENT_RTC_PLUGIN_ID);
    assert_eq!(health.status, "healthy");
    assert_eq!(health.details["providerKind"], "tencent");
}
