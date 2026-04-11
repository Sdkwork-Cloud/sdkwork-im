#[test]
fn test_notification_runtime_exposes_public_request_access_owner() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source.contains("pub fn request_notification_from_public_api("),
        "services/notification-service/src/lib.rs should expose a runtime-owned public notification request seam so service entrypoints do not each reimplement cross-recipient access control"
    );

    assert!(
        source.contains(".request_notification_from_public_api(&auth, request, is_bearer_request)"),
        "notification-service HTTP request path should consume the runtime-owned public notification request seam"
    );

    assert!(
        !source.contains(
            "ensure_notification_request_access(&auth, request.recipient_id.as_str(), is_bearer_request)?;"
        ),
        "notification-service HTTP request path should not inline cross-recipient notification access control once NotificationRuntime owns that public-api boundary"
    );
}

#[test]
fn test_notification_runtime_exposes_notification_fanout_owner_seam() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source.contains("pub fn request_notification_fanout("),
        "services/notification-service/src/lib.rs should expose a runtime-owned notification fanout seam so service-edge side-effect paths do not each reimplement per-recipient notification orchestration"
    );
}

#[test]
fn test_notification_runtime_exposes_automation_result_notification_owner_seam() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source.contains("pub fn request_automation_result_notification("),
        "services/notification-service/src/lib.rs should expose a runtime-owned automation result notification seam so service-edge automation paths do not each hand-assemble notification ids, source events, and recipient routing"
    );
}

#[test]
fn test_notification_runtime_exposes_message_posted_notification_owner_seam() {
    let source = include_str!("../src/lib.rs");

    assert!(
        source.contains("pub fn request_message_posted_notifications("),
        "services/notification-service/src/lib.rs should expose a runtime-owned message-posted notification seam so service-edge message side-effect paths do not each hand-assemble category, source-event type, and payload defaults"
    );

    assert!(
        source.contains(".message_posted_notification_principal_ids_from_auth_context("),
        "services/notification-service/src/lib.rs should resolve message-posted recipients through projection-service's message-posted auth-context owner seam instead of expecting service-edge recipient fanout input"
    );

    assert!(
        !source.contains("recipient_ids: request.recipient_ids"),
        "services/notification-service/src/lib.rs should not keep threading message-posted recipient_ids from callers once notification-service owns the projection-backed recipient seam"
    );
}
