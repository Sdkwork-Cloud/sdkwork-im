use std::any::type_name;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use craw_chat_contract_agent::{AgentSubject, AgentSubjectRecord, AgentSubjectStore};
use craw_chat_contract_core::ContractError;
use craw_chat_contract_iot::{DeviceSubject, DeviceSubjectRecord, DeviceSubjectStore};
use im_domain_core::message::Sender;

#[derive(Clone, Default)]
struct MemoryAgentSubjectStore {
    records: Arc<Mutex<HashMap<String, AgentSubjectRecord>>>,
}

impl AgentSubjectStore for MemoryAgentSubjectStore {
    fn load_subject(
        &self,
        tenant_id: &str,
        agent_id: &str,
    ) -> Result<Option<AgentSubjectRecord>, ContractError> {
        Ok(self
            .records
            .lock()
            .expect("agent subject store should lock")
            .get(format!("{tenant_id}:{agent_id}").as_str())
            .cloned())
    }

    fn save_subject(&self, record: AgentSubjectRecord) -> Result<(), ContractError> {
        self.records
            .lock()
            .expect("agent subject store should lock")
            .insert(
                format!("{}:{}", record.tenant_id, record.agent.agent_id),
                record,
            );
        Ok(())
    }
}

#[derive(Clone, Default)]
struct MemoryDeviceSubjectStore {
    records: Arc<Mutex<HashMap<String, DeviceSubjectRecord>>>,
}

impl DeviceSubjectStore for MemoryDeviceSubjectStore {
    fn load_subject(
        &self,
        tenant_id: &str,
        device_id: &str,
    ) -> Result<Option<DeviceSubjectRecord>, ContractError> {
        Ok(self
            .records
            .lock()
            .expect("device subject store should lock")
            .get(format!("{tenant_id}:{device_id}").as_str())
            .cloned())
    }

    fn save_subject(&self, record: DeviceSubjectRecord) -> Result<(), ContractError> {
        self.records
            .lock()
            .expect("device subject store should lock")
            .insert(
                format!("{}:{}", record.tenant_id, record.device.device_id),
                record,
            );
        Ok(())
    }
}

#[test]
fn test_step08_agent_and_device_subject_records_materialize_unified_sender_snapshots() {
    let agent_store = MemoryAgentSubjectStore::default();
    let device_store = MemoryDeviceSubjectStore::default();

    let agent = AgentSubject {
        agent_id: "ag_demo".into(),
        session_id: Some("s_agent".into()),
        metadata: BTreeMap::from([
            ("agentMode".into(), "assistant".into()),
            ("capabilityProfileId".into(), "stable-agent".into()),
        ]),
    };
    let device = DeviceSubject {
        device_id: "d_demo".into(),
        owner_principal_id: Some("u_owner".into()),
        session_id: Some("sess_device".into()),
        metadata: BTreeMap::from([
            ("deviceType".into(), "sensor".into()),
            ("binding".into(), "ccp/mqtt/1".into()),
        ]),
    };

    let agent_sender = agent.sender(Some("cm_agent".into()));
    assert_eq!(
        agent_sender,
        Sender {
            id: "ag_demo".into(),
            kind: "agent".into(),
            member_id: Some("cm_agent".into()),
            device_id: None,
            session_id: Some("s_agent".into()),
            metadata: BTreeMap::from([
                ("agentMode".into(), "assistant".into()),
                ("capabilityProfileId".into(), "stable-agent".into()),
            ]),
        }
    );

    let device_sender = device.sender(Some("cm_device".into()));
    assert_eq!(
        device_sender,
        Sender {
            id: "d_demo".into(),
            kind: "device".into(),
            member_id: Some("cm_device".into()),
            device_id: Some("d_demo".into()),
            session_id: Some("sess_device".into()),
            metadata: BTreeMap::from([
                ("deviceType".into(), "sensor".into()),
                ("binding".into(), "ccp/mqtt/1".into()),
            ]),
        }
    );

    let agent_record = AgentSubjectRecord {
        tenant_id: "t_demo".into(),
        agent: agent.clone(),
        updated_at: "2026-04-07T00:00:00Z".into(),
    };
    let device_record = DeviceSubjectRecord {
        tenant_id: "t_demo".into(),
        device: device.clone(),
        updated_at: "2026-04-07T00:00:00Z".into(),
    };

    agent_store
        .save_subject(agent_record.clone())
        .expect("agent subject save should succeed");
    device_store
        .save_subject(device_record.clone())
        .expect("device subject save should succeed");

    assert_eq!(
        agent_store
            .load_subject("t_demo", "ag_demo")
            .expect("agent subject load should succeed"),
        Some(agent_record)
    );
    assert_eq!(
        device_store
            .load_subject("t_demo", "d_demo")
            .expect("device subject load should succeed"),
        Some(device_record)
    );

    assert_eq!(
        type_name::<AgentSubjectRecord>(),
        type_name::<im_platform_contracts::AgentSubjectRecord>()
    );
    assert_eq!(
        type_name::<DeviceSubjectRecord>(),
        type_name::<im_platform_contracts::DeviceSubjectRecord>()
    );
}
