use std::any::type_name;

use craw_chat_contract_admin::{AdminCapabilityProfileRecord, AdminCapabilityProfileStore};
use craw_chat_contract_agent::AutomationExecutionStore;
use craw_chat_contract_control::{
    PresenceStateRecord, PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore,
};
use craw_chat_contract_core::{
    ContractError, MetadataStore, ObjectDescriptor, ObjectPutRequest, ObjectStore,
};
use craw_chat_contract_iot::{DeviceTwinRecord, DeviceTwinStore};
use craw_chat_contract_message::{CommitJournal, CommitPosition, TimelineProjectionStore};
use craw_chat_contract_notification::{NotificationTaskRecord, NotificationTaskStore};
use craw_chat_contract_rtc::{RtcStateRecord, RtcStateStore};
use craw_chat_contract_stream::{StreamStateRecord, StreamStateStore};

struct NullAdminStore;
struct NullMetadataStore;
struct NullObjectStore;
struct NullCommitJournal;
struct NullProjectionStore;
struct NullCheckpointStore;
struct NullDisconnectFenceStore;
struct NullSubscriptionStore;
struct NullPresenceStore;
struct NullStreamStore;
struct NullRtcStore;
struct NullNotificationStore;
struct NullAutomationStore;
struct NullIotStore;

impl AdminCapabilityProfileStore for NullAdminStore {
    fn load_profile(
        &self,
        _tenant_id: &str,
        _profile_id: &str,
    ) -> Result<Option<AdminCapabilityProfileRecord>, ContractError> {
        Ok(None)
    }

    fn save_profile(&self, _record: AdminCapabilityProfileRecord) -> Result<(), ContractError> {
        Ok(())
    }
}

impl MetadataStore for NullMetadataStore {
    fn put_snapshot(&self, _scope: &str, _key: &str, _value: &str) -> Result<(), ContractError> {
        Ok(())
    }

    fn load_snapshot(&self, _scope: &str, _key: &str) -> Result<Option<String>, ContractError> {
        Ok(None)
    }
}

impl ObjectStore for NullObjectStore {
    fn put(&self, request: ObjectPutRequest) -> Result<ObjectDescriptor, ContractError> {
        Ok(ObjectDescriptor {
            object_key: request.object_key,
            content_length: request.content_length,
        })
    }
}

impl CommitJournal for NullCommitJournal {
    fn append(
        &self,
        _envelope: im_domain_events::CommitEnvelope,
    ) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("message", 1))
    }
}

impl TimelineProjectionStore for NullProjectionStore {
    fn upsert_timeline_entry(
        &self,
        _conversation_id: &str,
        _message_seq: u64,
        _payload: &str,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn load_timeline(&self, _conversation_id: &str) -> Result<Vec<(u64, String)>, ContractError> {
        Ok(Vec::new())
    }
}

impl RealtimeCheckpointStore for NullCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(None)
    }

    fn save_checkpoint(&self, _record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        Ok(())
    }
}

impl RealtimeDisconnectFenceStore for NullDisconnectFenceStore {
    fn load_fence(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
        Ok(None)
    }

    fn save_fence(&self, _record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn clear_fence(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(false)
    }
}

impl RealtimeSubscriptionStore for NullSubscriptionStore {
    fn load_subscriptions(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError> {
        Ok(None)
    }

    fn save_subscriptions(&self, _record: RealtimeSubscriptionRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn clear_subscriptions(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(false)
    }
}

impl PresenceStateStore for NullPresenceStore {
    fn load_state(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError> {
        Ok(None)
    }

    fn save_state(&self, _record: PresenceStateRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn list_states_for_principal(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError> {
        Ok(Vec::new())
    }
}

impl StreamStateStore for NullStreamStore {
    fn load_state(
        &self,
        _tenant_id: &str,
        _stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        Ok(None)
    }

    fn save_state(&self, _record: StreamStateRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn clear_state(&self, _tenant_id: &str, _stream_id: &str) -> Result<bool, ContractError> {
        Ok(false)
    }
}

impl RtcStateStore for NullRtcStore {
    fn load_state(
        &self,
        _tenant_id: &str,
        _rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError> {
        Ok(None)
    }

    fn save_state(&self, _record: RtcStateRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn clear_state(&self, _tenant_id: &str, _rtc_session_id: &str) -> Result<bool, ContractError> {
        Ok(false)
    }
}

impl NotificationTaskStore for NullNotificationStore {
    fn load_task(
        &self,
        _tenant_id: &str,
        _notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(None)
    }

    fn save_task(&self, _record: NotificationTaskRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn list_tasks_for_recipient(
        &self,
        _tenant_id: &str,
        _recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        Ok(Vec::new())
    }
}

impl AutomationExecutionStore for NullAutomationStore {
    fn load_execution(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _execution_id: &str,
    ) -> Result<Option<craw_chat_contract_agent::AutomationExecutionRecord>, ContractError> {
        Ok(None)
    }

    fn save_execution(
        &self,
        _record: craw_chat_contract_agent::AutomationExecutionRecord,
    ) -> Result<(), ContractError> {
        Ok(())
    }
}

impl DeviceTwinStore for NullIotStore {
    fn load_twin(
        &self,
        _tenant_id: &str,
        _device_id: &str,
    ) -> Result<Option<DeviceTwinRecord>, ContractError> {
        Ok(None)
    }

    fn save_twin(&self, _record: DeviceTwinRecord) -> Result<(), ContractError> {
        Ok(())
    }
}

#[test]
fn test_step03_contract_split_exposes_real_crates_and_keeps_compatibility_facade() {
    let admin_store = NullAdminStore;
    let metadata = NullMetadataStore;
    let object_store = NullObjectStore;
    let journal = NullCommitJournal;
    let projection = NullProjectionStore;
    let checkpoint_store = NullCheckpointStore;
    let disconnect_fence_store = NullDisconnectFenceStore;
    let subscription_store = NullSubscriptionStore;
    let presence_store = NullPresenceStore;
    let stream_store = NullStreamStore;
    let rtc_store = NullRtcStore;
    let notification_store = NullNotificationStore;
    let automation_store = NullAutomationStore;
    let iot_store = NullIotStore;

    admin_store
        .save_profile(AdminCapabilityProfileRecord {
            tenant_id: "t_demo".into(),
            profile_id: "default".into(),
            release_channel: "stable".into(),
            capability_keys: vec!["session.resume".into(), "payload.json".into()],
            updated_at: "2026-04-07T00:00:00Z".into(),
        })
        .expect("admin profile save should succeed");

    metadata
        .put_snapshot("tenant", "key", "value")
        .expect("metadata snapshot should succeed");
    metadata
        .load_snapshot("tenant", "key")
        .expect("metadata snapshot load should succeed");
    let descriptor = object_store
        .put(ObjectPutRequest {
            object_key: "media/demo.png".into(),
            content_length: 8,
        })
        .expect("object put should succeed");
    let position = journal
        .append(im_domain_events::CommitEnvelope::minimal(
            "evt_contract_split",
            "t_demo",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        ))
        .expect("journal append should succeed");
    projection
        .upsert_timeline_entry("c_demo", 1, "{}")
        .expect("projection update should succeed");
    projection
        .load_timeline("c_demo")
        .expect("projection load should succeed");

    assert_eq!(descriptor.object_key, "media/demo.png");
    assert_eq!(position.cursor(), "message:1");

    assert_eq!(
        type_name::<AdminCapabilityProfileRecord>(),
        type_name::<im_platform_contracts::AdminCapabilityProfileRecord>()
    );
    assert_eq!(
        type_name::<CommitPosition>(),
        type_name::<im_platform_contracts::CommitPosition>()
    );
    assert_eq!(
        type_name::<DeviceTwinRecord>(),
        type_name::<im_platform_contracts::DeviceTwinRecord>()
    );
    assert_eq!(
        type_name::<RealtimeCheckpointRecord>(),
        type_name::<im_platform_contracts::RealtimeCheckpointRecord>()
    );
    assert_eq!(
        type_name::<RealtimeDisconnectFenceRecord>(),
        type_name::<im_platform_contracts::RealtimeDisconnectFenceRecord>()
    );
    assert_eq!(
        type_name::<RealtimeSubscriptionRecord>(),
        type_name::<im_platform_contracts::RealtimeSubscriptionRecord>()
    );
    assert_eq!(
        type_name::<PresenceStateRecord>(),
        type_name::<im_platform_contracts::PresenceStateRecord>()
    );
    assert_eq!(
        type_name::<StreamStateRecord>(),
        type_name::<im_platform_contracts::StreamStateRecord>()
    );
    assert_eq!(
        type_name::<RtcStateRecord>(),
        type_name::<im_platform_contracts::RtcStateRecord>()
    );
    assert_eq!(
        type_name::<NotificationTaskRecord>(),
        type_name::<im_platform_contracts::NotificationTaskRecord>()
    );
    assert_eq!(
        type_name::<craw_chat_contract_agent::AutomationExecutionRecord>(),
        type_name::<im_platform_contracts::AutomationExecutionRecord>()
    );

    checkpoint_store
        .load_checkpoint("t_demo", "u_demo", "d_demo")
        .expect("checkpoint load should succeed");
    disconnect_fence_store
        .clear_fence("t_demo", "u_demo", "d_demo")
        .expect("disconnect fence clear should succeed");
    subscription_store
        .clear_subscriptions("t_demo", "u_demo", "d_demo")
        .expect("subscription clear should succeed");
    presence_store
        .list_states_for_principal("t_demo", "u_demo")
        .expect("presence listing should succeed");
    stream_store
        .clear_state("t_demo", "stream_demo")
        .expect("stream clear should succeed");
    rtc_store
        .clear_state("t_demo", "rtc_demo")
        .expect("rtc clear should succeed");
    notification_store
        .list_tasks_for_recipient("t_demo", "u_demo")
        .expect("notification listing should succeed");
    automation_store
        .load_execution("t_demo", "user", "u_demo", "exec_demo")
        .expect("automation load should succeed");
    iot_store
        .save_twin(DeviceTwinRecord {
            tenant_id: "t_demo".into(),
            device_id: "device_demo".into(),
            desired_state_json: "{\"mode\":\"auto\"}".into(),
            reported_state_json: "{\"mode\":\"auto\"}".into(),
            updated_at: "2026-04-07T00:00:00Z".into(),
        })
        .expect("iot twin save should succeed");
}
