use std::sync::{Arc, RwLock};

use control_plane_api::SocialControlQuery;
use projection_service::TimelineProjectionService;
use session_gateway::{RealtimeRuntimeError, RealtimeScopeAccessPolicy};

pub(super) fn direct_chat_realtime_policy(
    projection_service: Arc<TimelineProjectionService>,
) -> Arc<DirectChatRealtimePolicy> {
    Arc::new(DirectChatRealtimePolicy {
        projection_service,
        social_query: RwLock::new(None),
    })
}

pub(super) struct DirectChatRealtimePolicy {
    projection_service: Arc<TimelineProjectionService>,
    social_query: RwLock<Option<Arc<SocialControlQuery>>>,
}

impl DirectChatRealtimePolicy {
    pub(super) fn bind_social_query(&self, social_query: Arc<SocialControlQuery>) {
        let mut guard = self
            .social_query
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *guard = Some(social_query);
    }

    fn conversation_access_error(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Option<RealtimeRuntimeError> {
        if self
            .projection_service
            .is_archived_direct_chat_conversation(tenant_id, conversation_id)
        {
            return Some(RealtimeRuntimeError {
                code: "conversation_archived",
                message: format!("direct chat conversation is archived: {conversation_id}"),
            });
        }

        let direct_chat_id = self
            .projection_service
            .direct_chat_id_for_conversation(tenant_id, conversation_id)?;
        let social_query = self
            .social_query
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()?;
        social_query
            .active_direct_chat_access_block(tenant_id, direct_chat_id.as_str())
            .map(|user_block| RealtimeRuntimeError {
                code: "conversation_blocked",
                message: format!(
                    "direct chat conversation is blocked by user block {}: {conversation_id}",
                    user_block.block_id
                ),
            })
    }
}

impl RealtimeScopeAccessPolicy for DirectChatRealtimePolicy {
    fn validate_subscription_scope(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope_type: &str,
        scope_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        if scope_type == "user" && (principal_kind != "user" || principal_id != scope_id) {
            return Err(RealtimeRuntimeError {
                code: "realtime_scope_access_denied",
                message: format!("principal cannot subscribe to user realtime scope: {scope_id}"),
            });
        }

        if scope_type == "conversation"
            && let Some(error) = self.conversation_access_error(tenant_id, scope_id)
        {
            return Err(error);
        }

        Ok(())
    }

    fn is_event_visible(
        &self,
        tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
        event: &im_domain_core::realtime::RealtimeEvent,
    ) -> bool {
        event.scope_type != "conversation"
            || self
                .conversation_access_error(tenant_id, event.scope_id.as_str())
                .is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_scope_subscriptions_are_limited_to_current_user() {
        let policy = DirectChatRealtimePolicy {
            projection_service: Arc::new(TimelineProjectionService::default()),
            social_query: RwLock::new(None),
        };

        assert!(
            policy
                .validate_subscription_scope("t_demo", "u_alice", "user", "user", "u_alice")
                .is_ok()
        );

        let error = policy
            .validate_subscription_scope("t_demo", "u_alice", "user", "user", "u_bob")
            .expect_err("user scope subscription to another user must be rejected");
        assert_eq!(error.code, "realtime_scope_access_denied");

        let service_error = policy
            .validate_subscription_scope("t_demo", "svc_social", "service", "user", "u_alice")
            .expect_err("non-user principals must not subscribe to user scopes");
        assert_eq!(service_error.code, "realtime_scope_access_denied");
    }
}
