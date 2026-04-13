use axum::http::StatusCode;
use im_auth_context::AuthContext;
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationReadCursorView, DeviceSyncFeedEntry,
};

use super::{
    ContactView, ConversationMemberDirectoryEntry, ConversationSummaryView,
    MessageInteractionSummaryView, NotificationRecipientView, RealtimeFanoutTarget,
    RegisteredDeviceView, TimelineProjectionService, TimelineViewEntry,
};

const PROJECTION_MAX_DEVICE_ID_BYTES: usize = 256;
const PROJECTION_MAX_CONVERSATION_ID_BYTES: usize = 256;
const PROJECTION_MAX_MESSAGE_ID_BYTES: usize = 256;

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

    fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
        }
    }

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }
}

impl TimelineProjectionService {
    pub fn ensure_active_member_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<(), ProjectionAccessError> {
        validate_conversation_id(conversation_id)?;
        let is_active = self
            .member_snapshot_for_principal_kind(
                auth.tenant_id.as_str(),
                conversation_id,
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )
            .is_some_and(|member| member.is_active());
        if is_active {
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
        validate_conversation_id(conversation_id)?;
        let can_read_history = self
            .member_snapshot_for_principal_kind(
                auth.tenant_id.as_str(),
                conversation_id,
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )
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

    pub fn active_conversation_principal_recipients_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<NotificationRecipientView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(
            super::device_sync::active_conversation_principal_recipients(
                self,
                auth.tenant_id.as_str(),
                conversation_id,
            ),
        )
    }

    pub fn message_posted_notification_recipients_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<NotificationRecipientView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.message_posted_notification_recipients(auth.tenant_id.as_str(), conversation_id))
    }

    pub(crate) fn message_posted_notification_recipients(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Vec<NotificationRecipientView> {
        let scope = super::scope::scope_key(tenant_id, conversation_id);
        let mut recipients = super::lock_projection_mutex(&self.members, "member store")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.can_read_shared_history())
                    .map(|member| NotificationRecipientView {
                        principal_id: member.principal_id.clone(),
                        principal_kind: member.principal_kind.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        recipients.sort();
        recipients.dedup();
        recipients
    }

    pub fn register_device_from_auth_context(
        &self,
        auth: &AuthContext,
        requested_device_id: Option<String>,
    ) -> Result<RegisteredDeviceView, ProjectionAccessError> {
        let device_id =
            self.ensure_device_registration_allowed_from_auth_context(auth, requested_device_id)?;
        Ok(self.register_device_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
        ))
    }

    pub fn ensure_device_registration_allowed_from_auth_context(
        &self,
        auth: &AuthContext,
        requested_device_id: Option<String>,
    ) -> Result<String, ProjectionAccessError> {
        let device_id = resolve_requested_device_id(auth, requested_device_id)?;
        ensure_device_registration_available(self, auth, device_id.as_str())?;
        Ok(device_id)
    }

    pub fn registered_devices_from_auth_context(
        &self,
        auth: &AuthContext,
    ) -> Vec<RegisteredDeviceView> {
        self.registered_devices_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
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
                ensure_device_owned_by_auth_kind(self, auth, device_id)?;
                Some(self.latest_device_sync_seq_for_principal_kind(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
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

    pub fn realtime_fanout_targets_for_recipients_from_auth_context(
        &self,
        auth: &AuthContext,
        recipients: impl IntoIterator<Item = NotificationRecipientView>,
    ) -> Vec<RealtimeFanoutTarget> {
        super::device_sync::realtime_fanout_targets_for_recipients(
            self,
            auth.tenant_id.as_str(),
            recipients,
        )
    }

    pub fn latest_device_sync_seq_from_auth_context(
        &self,
        auth: &AuthContext,
        device_id: &str,
    ) -> Result<u64, ProjectionAccessError> {
        validate_device_scope(auth, device_id)?;
        ensure_device_owned_by_auth_kind(self, auth, device_id)?;
        Ok(self.latest_device_sync_seq_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id,
        ))
    }

    pub fn device_sync_feed_from_auth_context(
        &self,
        auth: &AuthContext,
        device_id: &str,
        after_seq: Option<u64>,
    ) -> Result<Vec<DeviceSyncFeedEntry>, ProjectionAccessError> {
        validate_device_scope(auth, device_id)?;
        ensure_device_owned_by_auth_kind(self, auth, device_id)?;
        Ok(self.device_sync_feed_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id,
            after_seq,
        ))
    }

    pub fn inbox_from_auth_context(&self, auth: &AuthContext) -> Vec<ConversationInboxEntry> {
        self.inbox(auth.tenant_id.as_str(), auth.actor_id.as_str())
            .into_iter()
            .filter(|entry| {
                self.member_snapshot_for_principal_kind(
                    auth.tenant_id.as_str(),
                    entry.conversation_id.as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                )
                .is_some_and(|member| member.member_id == entry.member_id)
            })
            .collect()
    }

    pub fn contacts_from_auth_context(
        &self,
        auth: &AuthContext,
    ) -> Result<Vec<ContactView>, ProjectionAccessError> {
        ensure_user_contact_owner(auth)?;
        Ok(self.contacts(auth.tenant_id.as_str(), auth.actor_id.as_str()))
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
        validate_message_id(message_id)?;
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
        Ok(self.read_cursor_for_principal_kind(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
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
            validate_device_id(requested.as_str())?;
            validate_device_id(bound.as_str())?;
            if requested != bound {
                return Err(ProjectionAccessError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => {
            validate_device_id(requested.as_str())?;
            Ok(requested)
        }
        (None, Some(bound)) => {
            validate_device_id(bound.as_str())?;
            Ok(bound)
        }
        (None, None) => Err(ProjectionAccessError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_scope(auth: &AuthContext, device_id: &str) -> Result<(), ProjectionAccessError> {
    validate_device_id(device_id)?;
    if let Some(bound_device_id) = auth.device_id.as_deref() {
        validate_device_id(bound_device_id)?;
        if bound_device_id != device_id {
            return Err(ProjectionAccessError::forbidden(
                "device_scope_forbidden",
                format!("device scope forbidden: {device_id}"),
            ));
        }
    }
    Ok(())
}

fn ensure_device_registration_available(
    service: &TimelineProjectionService,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ProjectionAccessError> {
    let has_conflict =
        super::lock_projection_mutex(&service.registered_devices, "registered device store")
            .iter()
            .filter(|(scope, devices)| {
                scope.tenant_id == auth.tenant_id.as_str() && devices.contains_key(device_id)
            })
            .filter_map(|(_, devices)| devices.get(device_id))
            .any(|device| {
                device.principal_id != auth.actor_id
                    || effective_registered_device_kind(device.principal_kind.as_deref())
                        != auth.actor_kind
            });

    if has_conflict {
        return Err(ProjectionAccessError::conflict(
            "device_scope_conflict",
            format!("device scope already bound to a different principal: {device_id}"),
        ));
    }

    Ok(())
}

fn ensure_device_owned_by_auth_kind(
    service: &TimelineProjectionService,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ProjectionAccessError> {
    if service
        .registered_devices_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
        .into_iter()
        .any(|device| device.device_id == device_id)
    {
        return Ok(());
    }

    Err(ProjectionAccessError::forbidden(
        "device_scope_forbidden",
        format!("device scope forbidden: {device_id}"),
    ))
}

fn effective_registered_device_kind(principal_kind: Option<&str>) -> &str {
    principal_kind.unwrap_or("user")
}

fn ensure_user_contact_owner(auth: &AuthContext) -> Result<(), ProjectionAccessError> {
    if auth.actor_kind == "user" {
        return Ok(());
    }

    Err(ProjectionAccessError::forbidden(
        "contact_scope_forbidden",
        format!(
            "contact scope forbidden for actor kind: {}",
            auth.actor_kind
        ),
    ))
}

fn validate_device_id(device_id: &str) -> Result<(), ProjectionAccessError> {
    let actual_bytes = device_id.len();
    if actual_bytes > PROJECTION_MAX_DEVICE_ID_BYTES {
        return Err(ProjectionAccessError::payload_too_large(
            "deviceId",
            PROJECTION_MAX_DEVICE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_conversation_id(conversation_id: &str) -> Result<(), ProjectionAccessError> {
    let actual_bytes = conversation_id.len();
    if actual_bytes > PROJECTION_MAX_CONVERSATION_ID_BYTES {
        return Err(ProjectionAccessError::payload_too_large(
            "conversationId",
            PROJECTION_MAX_CONVERSATION_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_message_id(message_id: &str) -> Result<(), ProjectionAccessError> {
    let actual_bytes = message_id.len();
    if actual_bytes > PROJECTION_MAX_MESSAGE_ID_BYTES {
        return Err(ProjectionAccessError::payload_too_large(
            "messageId",
            PROJECTION_MAX_MESSAGE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}
