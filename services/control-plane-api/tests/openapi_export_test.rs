#[test]
fn test_library_exports_control_plane_openapi_document() {
    let document =
        control_plane_api::export_openapi_document().expect("openapi document should export");

    assert_eq!(document["openapi"], "3.0.3");
    assert_eq!(document["info"]["title"], "Control Plane API");
    assert!(document["paths"]["/backend/v3/api/control/protocol_registry"].is_object());
}

#[test]
fn test_control_plane_openapi_uses_sdkwork_dual_token_security() {
    let document =
        control_plane_api::export_openapi_document().expect("openapi document should export");

    let schemes = document
        .pointer("/components/securitySchemes")
        .and_then(serde_json::Value::as_object)
        .expect("control plane OpenAPI should expose security schemes");

    assert!(
        schemes.get("bearerAuth").is_none(),
        "legacy bearerAuth must not be emitted by backend API OpenAPI"
    );
    assert_eq!(
        schemes
            .get("AuthToken")
            .and_then(|scheme| scheme.get("type"))
            .and_then(serde_json::Value::as_str),
        Some("http")
    );
    assert_eq!(
        schemes
            .get("AccessToken")
            .and_then(|scheme| scheme.get("name"))
            .and_then(serde_json::Value::as_str),
        Some("Access-Token")
    );

    let protected_operation = document
        .pointer("/paths/~1backend~1v3~1api~1control~1protocol_registry/get/security")
        .and_then(serde_json::Value::as_array)
        .expect("protected operation should define security");
    assert!(
        protected_operation
            .iter()
            .any(|entry| entry.get("AuthToken").is_some()
                && entry.get("AccessToken").is_some()),
        "backend API protected operations must require SDKWork AuthToken plus AccessToken"
    );
}
