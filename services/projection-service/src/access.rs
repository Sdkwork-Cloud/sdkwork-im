use axum::http::StatusCode;
use im_auth_context::AuthContext;
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationReadCursorView, DeviceSyncFeedEntry,
};

use super::{
    ContactView, ConversationMemberDirectoryEntry, ConversationSummaryView,
    MessageInteractionSummaryView, RealtimeFanoutTarget, RegisteredDeviceView,
    TimelineProjectionService, TimelineViewEntry,
};

#[derive(Debug)]
pub struct ProjectionAccessError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceSyncSessionState {
    pub registered_devices: Vec<String>,
    pub latest_sync_seq: Option<u64>,
}

impl ProjectionAccessError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }
}

impl TimelineProjectionService {
    pub fn ensure_active_member_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<(), ProjectionAccessError> {
        if self.is_active_member(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
        ) {
            return Ok(());
        }

        Err(ProjectionAccessError::forbidden(
            "conversation_permission_denied",
            format!(
                "principal is not active conversation member: {}",
                auth.actor_id
            ),
        ))
    }

    pub fn ensure_history_reader_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<(), ProjectionAccessError> {
        let scope = super::scope::scope_key(auth.tenant_id.as_str(), conversation_id);
        let can_read_history = super::lock_projection_mutex(&self.members, "member store")
            .get(scope.as_str())
            .and_then(|scope_members| scope_members.get(auth.actor_id.as_str()))
            .is_some_and(|member| member.can_read_shared_history());

        if can_read_history {
            return Ok(());
        }

        Err(ProjectionAccessError::forbidden(
            "conversation_permission_denied",
            format!(
                "principal cannot read conversation history: {}",
                auth.actor_id
            ),
        ))
    }

    pub fn active_conversation_principal_ids_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<String>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.active_conversation_principal_ids(auth.tenant_id.as_str(), conversation_id))
    }

    pub fn message_posted_notification_principal_ids_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<String>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(
            self.message_posted_notification_principal_ids(
                auth.tenant_id.as_str(),
                conversation_id,
            ),
        )
    }

    pub(crate) fn message_posted_notification_principal_ids(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Vec<String> {
        let scope = super::scope::scope_key(tenant_id, conversation_id);
        let mut principal_ids = super::lock_projection_mutex(&self.members, "member store")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.can_read_shared_history())
                    .map(|member| member.principal_id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        principal_ids.sort();
        principal_ids.dedup();
        principal_ids
    }

    pub fn register_device_from_auth_context(
        &self,
        auth: &AuthContext,
        requested_device_id: Option<String>,
    ) -> Result<RegisteredDeviceView, ProjectionAccessError> {
        let device_id = resolve_requested_device_id(auth, requested_device_id)?;
        Ok(self.register_device(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        ))
    }

    pub fn registered_devices_from_auth_context(
        &self,
        auth: &AuthContext,
    ) -> Vec<RegisteredDeviceView> {
        self.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())
    }

    pub fn device_sync_session_state_from_auth_context(
        &self,
        auth: &AuthContext,
        requested_device_id: Option<&str>,
    ) -> Result<DeviceSyncSessionState, ProjectionAccessError> {
        let registered_devices = self
            .registered_devices_from_auth_context(auth)
            .into_iter()
            .map(|item| item.device_id)
            .collect::<Vec<_>>();
        let latest_sync_seq = match requested_device_id.or(auth.device_id.as_deref()) {
            Some(device_id) => {
                validate_device_scope(auth, device_id)?;
                Some(self.latest_device_sync_seq(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    device_id,
                ))
            }
            None => None,
        };

        Ok(DeviceSyncSessionState {
            registered_devices,
            latest_sync_seq,
        })
    }

    pub fn realtime_fanout_targets_from_auth_context(
        &self,
        auth: &AuthContext,
        principal_ids: impl IntoIterator<Item = String>,
    ) -> Vec<RealtimeFanoutTarget> {
        self.realtime_fanout_targets_for_principals(auth.tenant_id.as_str(), principal_ids)
    }

    pub fn latest_device_sync_seq_from_auth_context(
        &self,
        auth: &AuthContext,
        device_id: &str,
    ) -> Result<u64, ProjectionAccessError> {
        validate_device_scope(auth, device_id)?;
        Ok(self.latest_device_sync_seq(auth.tenant_id.as_str(), auth.actor_id.as_str(), device_id))
    }

    pub fn device_sync_feed_from_auth_context(
        &self,
        auth: &AuthContext,
        device_id: &str,
        after_seq: Option<u64>,
    ) -> Result<Vec<DeviceSyncFeedEntry>, ProjectionAccessError> {
        validate_device_scope(auth, device_id)?;
        Ok(self.device_sync_feed(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id,
            after_seq,
        ))
    }

    pub fn inbox_from_auth_context(&self, auth: &AuthContext) -> Vec<ConversationInboxEntry> {
        self.inbox(auth.tenant_id.as_str(), auth.actor_id.as_str())
    }

    pub fn contacts_from_auth_context(&self, auth: &AuthContext) -> Vec<ContactView> {
        self.contacts(auth.tenant_id.as_str(), auth.actor_id.as_str())
    }

    pub fn timeline_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<TimelineViewEntry>, ProjectionAccessError> {
        self.ensure_history_reader_from_auth_context(auth, conversation_id)?;
        Ok(self.timeline(auth.tenant_id.as_str(), conversation_id))
    }

    pub fn conversation_summary_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Option<ConversationSummaryView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.conversation_summary(auth.tenant_id.as_str(), conversation_id))
    }

    pub fn message_interaction_summary_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<Option<MessageInteractionSummaryView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.message_interaction_summary(auth.tenant_id.as_str(), conversation_id, message_id))
    }

    pub fn pinned_messages_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<MessageInteractionSummaryView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.pinned_messages(auth.tenant_id.as_str(), conversation_id))
    }

    pub fn read_cursor_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Option<ConversationReadCursorView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.read_cursor(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
        ))
    }

    pub fn member_directory_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMemberDirectoryEntry>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.member_directory(auth.tenant_id.as_str(), conversation_id))
    }
}

fn resolve_requested_device_id(
    auth: &AuthContext,
    requested_device_id: Option<String>,
) -> Result<String, ProjectionAccessError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            if requested != bound {
                return Err(ProjectionAccessError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => Ok(requested),
        (None, Some(bound)) => Ok(bound),
        (None, None) => Err(ProjectionAccessError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_scope(auth: &AuthContext, device_id: &str) -> Result<(), ProjectionAccessError> {
    if let Some(bound_device_id) = auth.device_id.as_deref()
        && bound_device_id != device_id
    {
        return Err(ProjectionAccessError::forbidden(
            "device_scope_forbidden",
            format!("device scope forbidden: {device_id}"),
        ));
    }
    Ok(())
}
