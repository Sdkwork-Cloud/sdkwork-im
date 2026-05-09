#[test]
fn test_media_runtime_keys_use_segment_safe_encoding() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    assert!(
        source.contains("fn encode_media_key_segments"),
        "media runtime keys need a single segment-safe encoder for assets and idempotency"
    );
    assert!(
        source.contains("encode_media_key_segments([tenant_id, media_asset_id])"),
        "media asset scope keys must use segment-safe length-prefixed encoding"
    );
    assert!(
        source.contains("encode_media_key_segments([\n        auth.tenant_id.as_str(),"),
        "media upload idempotency keys must use the segment-safe media key encoder"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{media_asset_id}\")"),
        "media asset scope keys must not use delimiter-composed tenant/asset ids"
    );
    for forbidden in ["{}:{}:{}:create:{}", "{}:{}:{}:complete:{}"] {
        assert!(
            !source.contains(forbidden),
            "media upload idempotency keys must not use delimiter-composed format string: {forbidden}"
        );
    }
}
