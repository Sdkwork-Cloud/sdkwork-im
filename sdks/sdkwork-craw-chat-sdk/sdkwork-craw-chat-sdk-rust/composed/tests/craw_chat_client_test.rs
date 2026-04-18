use std::collections::HashMap;

use craw_chat_sdk::{
    build_json_rtc_signal,
    build_text_message,
    build_text_stream_frame,
    CrawChatClient,
    JsonRtcSignalOptions,
    PostTextOptions,
    SdkworkConfig,
    SdkworkError,
    TextFrameOptions,
};
use serde_json::json;
use sdkwork_craw_chat_backend_sdk::SdkworkBackendClient;

#[test]
fn craw_chat_client_wraps_generated_backend_client() -> Result<(), Box<dyn std::error::Error>> {
    let backend = SdkworkBackendClient::new_with_base_url("http://127.0.0.1:18090")?;
    let sdk = CrawChatClient::new(backend.clone());
    sdk.set_auth_token("token");
    let _ = sdk.backend_client();
    Ok(())
}

#[test]
fn craw_chat_client_can_be_built_from_base_url() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = CrawChatClient::new_with_base_url("http://127.0.0.1:18090")?;
    sdk.set_auth_token("token");
    Ok(())
}

#[test]
fn craw_chat_client_reexports_generated_setup_types() {
    let _config = SdkworkConfig::new("http://127.0.0.1:18090");
    let _error_type_name = std::any::type_name::<SdkworkError>();
}

#[test]
fn craw_chat_client_exposes_modules_and_builders() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = CrawChatClient::new_with_base_url("http://127.0.0.1:18090")?;

    let _ = sdk.session();
    let _ = sdk.presence();
    let _ = sdk.realtime();
    let _ = sdk.devices();
    let _ = sdk.inbox();
    let _ = sdk.messages();
    let _ = sdk.media();

    let render_hints = HashMap::from([("tone".to_string(), "friendly".to_string())]);
    let request = build_text_message(
        "hello world",
        PostTextOptions {
            client_msg_id: Some("client-1".into()),
            summary: Some("Greeting".into()),
            render_hints: Some(render_hints),
            ..Default::default()
        },
    );
    assert_eq!(request.text.as_deref(), Some("hello world"));

    let frame = build_text_stream_frame(
        7,
        "partial chunk",
        TextFrameOptions {
            schema_ref: Some("urn:craw-chat:stream:text".into()),
            attributes: Some(HashMap::from([("role".to_string(), "assistant".to_string())])),
            ..Default::default()
        },
    );
    assert_eq!(frame.frame_type, "text");
    assert_eq!(frame.encoding, "text/plain; charset=utf-8");

    let signal = build_json_rtc_signal(
        "offer",
        &json!({"sdp":"v=0","type":"offer"}),
        JsonRtcSignalOptions {
            schema_ref: Some("urn:craw-chat:rtc:signal".into()),
            signaling_stream_id: Some("signal-stream-1".into()),
            ..Default::default()
        },
    );
    assert_eq!(signal.signal_type, "offer");

    let conversations = sdk.conversations();
    let streams = sdk.streams();
    let rtc = sdk.rtc();
    let offer_payload = json!({"sdp":"v=0","type":"offer"});

    let _post_text = conversations.post_text(
        "conversation-1",
        "hello world",
        PostTextOptions::default(),
    );
    let _append_text_frame = streams.append_text_frame(
        "stream-1",
        7,
        "partial chunk",
        TextFrameOptions::default(),
    );
    let _post_json_signal = rtc.post_json_signal(
        "rtc-session-1",
        "offer",
        &offer_payload,
        JsonRtcSignalOptions::default(),
    );

    Ok(())
}
