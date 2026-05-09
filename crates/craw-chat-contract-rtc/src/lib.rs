use craw_chat_contract_core::ContractError;
use std::collections::BTreeMap;

use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtcStateRecord {
    pub tenant_id: String,
    pub rtc_session_id: String,
    pub session: RtcSession,
    pub signals: Vec<RtcSignalEvent>,
    pub updated_at: String,
}

impl RtcStateRecord {
    pub fn merge_monotonic(self, next: Self) -> Self {
        let session = if rtc_session_state_rank(&next.session.state)
            >= rtc_session_state_rank(&self.session.state)
        {
            next.session
        } else {
            self.session
        };
        let mut signals_by_seq = BTreeMap::new();
        for signal in self.signals.into_iter().chain(next.signals) {
            signals_by_seq.insert(signal.signal_seq, signal);
        }
        Self {
            tenant_id: next.tenant_id,
            rtc_session_id: next.rtc_session_id,
            session,
            signals: signals_by_seq.into_values().collect(),
            updated_at: self.updated_at.max(next.updated_at),
        }
    }
}

fn rtc_session_state_rank(state: &RtcSessionState) -> u8 {
    match state {
        RtcSessionState::Started => 0,
        RtcSessionState::Rejected => 1,
        RtcSessionState::Accepted => 2,
        RtcSessionState::Ended => 3,
    }
}

pub trait RtcStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError>;

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError>;

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::message::Sender;

    fn rtc_state_record(
        state: RtcSessionState,
        updated_at: &str,
        signals: Vec<RtcSignalEvent>,
    ) -> RtcStateRecord {
        RtcStateRecord {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            session: RtcSession {
                tenant_id: "t_demo".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
                initiator_id: "u_demo".into(),
                initiator_kind: "user".into(),
                provider_plugin_id: Some("webrtc".into()),
                provider_session_id: Some("ps_demo".into()),
                access_endpoint: Some("wss://rtc.example.test/session/ps_demo".into()),
                provider_region: Some("cn-shanghai".into()),
                state,
                signaling_stream_id: Some("st_demo".into()),
                artifact_message_id: None,
                started_at: "2026-05-06T00:00:00.000Z".into(),
                ended_at: None,
            },
            signals,
            updated_at: updated_at.into(),
        }
    }

    fn rtc_signal_event(signal_seq: u64) -> RtcSignalEvent {
        RtcSignalEvent {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            signal_seq,
            conversation_id: Some("c_demo".into()),
            rtc_mode: "voice".into(),
            signal_type: format!("rtc.signal.{signal_seq}"),
            schema_ref: Some("webrtc.signal.v1".into()),
            payload: format!("{{\"seq\":{signal_seq}}}"),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            signaling_stream_id: Some("st_demo".into()),
            occurred_at: format!("2026-05-06T00:00:0{signal_seq}.000Z"),
        }
    }

    #[test]
    fn test_rtc_state_record_merge_preserves_accepted_session_over_stale_reject() {
        let accepted = rtc_state_record(
            RtcSessionState::Accepted,
            "2026-05-06T00:00:03.000Z",
            vec![rtc_signal_event(1), rtc_signal_event(2)],
        );
        let stale_reject = rtc_state_record(
            RtcSessionState::Rejected,
            "2026-05-06T00:00:02.000Z",
            vec![rtc_signal_event(1)],
        );

        let merged = accepted.merge_monotonic(stale_reject);

        assert_eq!(merged.session.state, RtcSessionState::Accepted);
        assert_eq!(merged.updated_at, "2026-05-06T00:00:03.000Z");
        assert_eq!(
            merged
                .signals
                .iter()
                .map(|signal| signal.signal_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }
}
