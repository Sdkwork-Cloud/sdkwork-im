use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use im_app_context::AppContext;
use im_domain_core::media::{MediaResource, MediaResourceType};
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl RecordingJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_complete_upload_appends_media_asset_created_event() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = media_service::MediaRuntime::with_journal(journal.clone());
    let auth = AppContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    runtime
        .create_upload(
            &auth,
            media_service::CreateUploadRequest {
                media_asset_id: "ma_demo".into(),
                bucket: Some("local-media".into()),
                object_key: None,
                expires_in_seconds: None,
                resource: MediaResource {
                    id: None,
                    uuid: Some("res_demo".into()),
                    url: None,
                    bytes: None,
                    local_file: None,
                    base64: None,
                    resource_type: Some(MediaResourceType::Image),
                    mime_type: Some("image/png".into()),
                    size: Some(42),
                    name: Some("demo.png".into()),
                    extension: Some("png".into()),
                    tags: None,
                    metadata: None,
                    prompt: Some("poster".into()),
                },
            },
        )
        .expect("create upload should succeed");

    runtime
        .complete_upload(
            &auth,
            "ma_demo",
            media_service::CompleteUploadRequest {
                bucket: "local-media".into(),
                object_key: "tenant/t_demo/ma_demo/demo.png".into(),
                storage_provider: Some("object-storage-volcengine".into()),
                url: "https://cdn.example.com/ma_demo/demo.png".into(),
                checksum: Some("sha256:demo".into()),
            },
        )
        .expect("complete upload should succeed");

    let events = journal.recorded();
    assert_eq!(events.len(), 1);

    let event = &events[0];
    assert_eq!(event.event_type, "media.asset.created");
    assert_eq!(event.aggregate_type, AggregateType::MediaAsset);
    assert_eq!(event.aggregate_id, "ma_demo");
    assert_eq!(event.scope_type, "media_asset");
    assert_eq!(event.scope_id, "ma_demo");
    assert_eq!(event.actor.actor_id, "u_demo");
    assert_eq!(event.actor.actor_kind, "user");
    assert_eq!(event.actor.actor_session_id.as_deref(), Some("s_demo"));
    assert_eq!(
        event.payload_schema.as_deref(),
        Some("media.asset.created.v1")
    );

    let payload: serde_json::Value =
        serde_json::from_str(&event.payload).expect("payload should be valid json");
    assert_eq!(payload["mediaAssetId"], "ma_demo");
    assert_eq!(payload["principalId"], "u_demo");
    assert_eq!(payload["principalKind"], "user");
    assert_eq!(payload["processingState"], "ready");
    assert_eq!(payload["storageProvider"], "object-storage-volcengine");
    let download_url = payload["resource"]["url"]
        .as_str()
        .expect("event payload should contain provider download url");
    assert!(download_url.contains("object-storage-volcengine"));
    assert!(download_url.contains("expires=3600"));
}

#[test]
fn test_media_asset_created_events_use_distinct_commit_timestamps() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = media_service::MediaRuntime::with_journal(journal.clone());
    let auth = AppContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    runtime
        .create_upload(
            &auth,
            media_service::CreateUploadRequest {
                media_asset_id: "ma_event_one".into(),
                bucket: Some("local-media".into()),
                object_key: None,
                expires_in_seconds: None,
                resource: MediaResource {
                    id: None,
                    uuid: Some("res_event_one".into()),
                    url: None,
                    bytes: None,
                    local_file: None,
                    base64: None,
                    resource_type: Some(MediaResourceType::Image),
                    mime_type: Some("image/png".into()),
                    size: Some(42),
                    name: Some("one.png".into()),
                    extension: Some("png".into()),
                    tags: None,
                    metadata: None,
                    prompt: None,
                },
            },
        )
        .expect("first create upload should succeed");
    runtime
        .complete_upload(
            &auth,
            "ma_event_one",
            media_service::CompleteUploadRequest {
                bucket: "local-media".into(),
                object_key: "tenant/t_demo/ma_event_one/one.png".into(),
                storage_provider: Some("object-storage-volcengine".into()),
                url: "https://cdn.example.com/ma_event_one/one.png".into(),
                checksum: None,
            },
        )
        .expect("first complete upload should succeed");

    sleep(Duration::from_millis(20));

    runtime
        .create_upload(
            &auth,
            media_service::CreateUploadRequest {
                media_asset_id: "ma_event_two".into(),
                bucket: Some("local-media".into()),
                object_key: None,
                expires_in_seconds: None,
                resource: MediaResource {
                    id: None,
                    uuid: Some("res_event_two".into()),
                    url: None,
                    bytes: None,
                    local_file: None,
                    base64: None,
                    resource_type: Some(MediaResourceType::Image),
                    mime_type: Some("image/png".into()),
                    size: Some(42),
                    name: Some("two.png".into()),
                    extension: Some("png".into()),
                    tags: None,
                    metadata: None,
                    prompt: None,
                },
            },
        )
        .expect("second create upload should succeed");
    runtime
        .complete_upload(
            &auth,
            "ma_event_two",
            media_service::CompleteUploadRequest {
                bucket: "local-media".into(),
                object_key: "tenant/t_demo/ma_event_two/two.png".into(),
                storage_provider: Some("object-storage-volcengine".into()),
                url: "https://cdn.example.com/ma_event_two/two.png".into(),
                checksum: None,
            },
        )
        .expect("second complete upload should succeed");

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert!(events[0].committed_at < events[1].committed_at);
    assert!(events[0].occurred_at < events[1].occurred_at);
}
