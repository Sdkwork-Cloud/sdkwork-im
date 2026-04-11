#[test]
fn test_local_disk_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "adapters/local-disk/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_local_disk_state_store_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileStreamStateStore {",
        "pub struct FileRtcStateStore {",
        "pub struct FilePresenceStateStore {",
        "impl StreamStateStore for FileStreamStateStore {",
        "impl RtcStateStore for FileRtcStateStore {",
        "impl PresenceStateStore for FilePresenceStateStore {",
        "pub fn validate_stream_state_store_file(",
        "pub fn validate_rtc_state_store_file(",
        "pub fn validate_presence_state_store_file(",
        "fn stream_scope_key(",
        "fn rtc_scope_key(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "adapters/local-disk/src/lib.rs should not keep state-store symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_disk_ops_store_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileNotificationTaskStore {",
        "pub struct FileAutomationExecutionStore {",
        "impl NotificationTaskStore for FileNotificationTaskStore {",
        "impl AutomationExecutionStore for FileAutomationExecutionStore {",
        "pub fn validate_notification_task_store_file(",
        "pub fn validate_automation_execution_store_file(",
        "fn notification_scope_key(",
        "fn execution_scope_key(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "adapters/local-disk/src/lib.rs should not keep ops-store symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_disk_metadata_and_projection_store_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "pub struct FileMetadataStore {",
        "pub struct FileTimelineProjectionStore {",
        "impl MetadataStore for FileMetadataStore {",
        "impl TimelineProjectionStore for FileTimelineProjectionStore {",
        "pub fn validate_metadata_store_file(",
        "pub fn validate_timeline_projection_store_file(",
        "fn snapshot_key(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "adapters/local-disk/src/lib.rs should not keep metadata/projection-store symbol: {forbidden_symbol}"
        );
    }
}
