#[test]
fn test_library_exports_control_plane_openapi_document() {
    let document =
        control_plane_api::export_openapi_document().expect("openapi document should export");

    assert_eq!(document["openapi"], "3.0.3");
    assert_eq!(document["info"]["title"], "Craw Chat Admin Control Plane API");
    assert!(document["paths"]["/api/v1/control/protocol-registry"].is_object());
}
