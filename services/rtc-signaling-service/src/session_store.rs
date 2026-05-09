use std::collections::{BTreeSet, HashMap};

use crate::encode_rtc_key_segments;
use im_domain_core::rtc::{RtcSession, RtcSessionState};

#[derive(Default)]
pub struct RtcSessionRuntimeStore {
    sessions_by_id: HashMap<String, RtcSession>,
    sessions_by_conversation: HashMap<String, BTreeSet<String>>,
    sessions_by_state: HashMap<String, BTreeSet<String>>,
}

impl RtcSessionRuntimeStore {
    pub(crate) fn has_session(&self, tenant_id: &str, rtc_session_id: &str) -> bool {
        self.sessions_by_id
            .contains_key(rtc_scope_key(tenant_id, rtc_session_id).as_str())
    }

    pub(crate) fn session(&self, tenant_id: &str, rtc_session_id: &str) -> Option<RtcSession> {
        self.sessions_by_id
            .get(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .cloned()
    }

    pub(crate) fn insert_session(&mut self, session: RtcSession) -> Option<RtcSession> {
        let scope_key = rtc_scope_key(session.tenant_id.as_str(), session.rtc_session_id.as_str());
        if let Some(previous) = self.sessions_by_id.remove(scope_key.as_str()) {
            self.remove_indexes(scope_key.as_str(), &previous);
            let replaced = previous;
            self.add_indexes(scope_key.as_str(), &session);
            self.sessions_by_id.insert(scope_key, session);
            return Some(replaced);
        }

        self.add_indexes(scope_key.as_str(), &session);
        self.sessions_by_id.insert(scope_key, session)
    }

    pub(crate) fn update_session<T>(
        &mut self,
        tenant_id: &str,
        rtc_session_id: &str,
        update: impl FnOnce(&mut RtcSession) -> T,
    ) -> Option<T> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let previous = self.sessions_by_id.get(scope_key.as_str()).cloned()?;
        self.remove_indexes(scope_key.as_str(), &previous);

        let outcome = {
            let session = self
                .sessions_by_id
                .get_mut(scope_key.as_str())
                .expect("rtc session should exist after previous lookup");
            update(session)
        };
        let updated = self
            .sessions_by_id
            .get(scope_key.as_str())
            .expect("rtc session should exist after update")
            .clone();
        self.add_indexes(scope_key.as_str(), &updated);
        Some(outcome)
    }

    pub(crate) fn sessions_for_conversation(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Vec<RtcSession> {
        self.sessions_for_index(
            conversation_key(tenant_id, conversation_id),
            &self.sessions_by_conversation,
        )
    }

    pub(crate) fn sessions_for_state(
        &self,
        tenant_id: &str,
        state: &RtcSessionState,
    ) -> Vec<RtcSession> {
        self.sessions_for_index(state_key(tenant_id, state), &self.sessions_by_state)
    }

    fn sessions_for_index(
        &self,
        index_key: String,
        index: &HashMap<String, BTreeSet<String>>,
    ) -> Vec<RtcSession> {
        index
            .get(index_key.as_str())
            .into_iter()
            .flat_map(|scope_keys| scope_keys.iter())
            .filter_map(|scope_key| self.sessions_by_id.get(scope_key.as_str()).cloned())
            .collect()
    }

    fn add_indexes(&mut self, scope_key: &str, session: &RtcSession) {
        if let Some(conversation_id) = session.conversation_id.as_deref() {
            self.sessions_by_conversation
                .entry(conversation_key(
                    session.tenant_id.as_str(),
                    conversation_id,
                ))
                .or_default()
                .insert(scope_key.to_owned());
        }
        self.sessions_by_state
            .entry(state_key(session.tenant_id.as_str(), &session.state))
            .or_default()
            .insert(scope_key.to_owned());
    }

    fn remove_indexes(&mut self, scope_key: &str, session: &RtcSession) {
        if let Some(conversation_id) = session.conversation_id.as_deref() {
            remove_scope_from_index(
                &mut self.sessions_by_conversation,
                conversation_key(session.tenant_id.as_str(), conversation_id).as_str(),
                scope_key,
            );
        }
        remove_scope_from_index(
            &mut self.sessions_by_state,
            state_key(session.tenant_id.as_str(), &session.state).as_str(),
            scope_key,
        );
    }
}

fn remove_scope_from_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    index_key: &str,
    scope_key: &str,
) {
    let Some(scope_keys) = index.get_mut(index_key) else {
        return;
    };
    scope_keys.remove(scope_key);
    if scope_keys.is_empty() {
        index.remove(index_key);
    }
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    encode_rtc_key_segments([tenant_id, rtc_session_id])
}

fn conversation_key(tenant_id: &str, conversation_id: &str) -> String {
    encode_rtc_key_segments([tenant_id, conversation_id])
}

fn state_key(tenant_id: &str, state: &RtcSessionState) -> String {
    encode_rtc_key_segments([tenant_id, state.as_wire_value()])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(tenant_id: &str, rtc_session_id: &str, conversation_id: Option<&str>) -> RtcSession {
        RtcSession {
            tenant_id: tenant_id.to_owned(),
            rtc_session_id: rtc_session_id.to_owned(),
            conversation_id: conversation_id.map(ToOwned::to_owned),
            rtc_mode: "audio".to_owned(),
            initiator_id: "alice".to_owned(),
            initiator_kind: "user".to_owned(),
            provider_plugin_id: None,
            provider_session_id: None,
            access_endpoint: None,
            provider_region: None,
            state: RtcSessionState::Started,
            signaling_stream_id: None,
            artifact_message_id: None,
            started_at: "2026-05-06T00:00:00Z".to_owned(),
            ended_at: None,
        }
    }

    #[test]
    fn test_session_scope_key_is_segment_safe() {
        let mut store = RtcSessionRuntimeStore::default();
        store.insert_session(session("tenant:a", "b", None));
        store.insert_session(session("tenant", "a:b", None));

        assert_eq!(
            store
                .session("tenant:a", "b")
                .expect("first session should not be overwritten")
                .rtc_session_id,
            "b"
        );
        assert_eq!(
            store
                .session("tenant", "a:b")
                .expect("second session should be retrievable")
                .rtc_session_id,
            "a:b"
        );
    }

    #[test]
    fn test_conversation_index_key_is_segment_safe() {
        let mut store = RtcSessionRuntimeStore::default();
        store.insert_session(session("tenant:a", "rtc-1", Some("room")));
        store.insert_session(session("tenant", "rtc-2", Some("a:room")));

        let first = store.sessions_for_conversation("tenant:a", "room");
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].rtc_session_id, "rtc-1");

        let second = store.sessions_for_conversation("tenant", "a:room");
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].rtc_session_id, "rtc-2");
    }
}
