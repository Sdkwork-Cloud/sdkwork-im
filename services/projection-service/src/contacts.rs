use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use im_domain_events::CommitEnvelope;
use im_domain_events::social::{DirectChatBoundPayload, FriendshipActivatedPayload};

use crate::model::ContactDirectChatBindingView;
use crate::{ContactView, TimelineProjectionService};

use super::projection::ProjectionError;
use super::scope::principal_scope_key;

impl TimelineProjectionService {
    pub fn contacts(&self, tenant_id: &str, owner_user_id: &str) -> Vec<ContactView> {
        let scope = contact_runtime_scope(tenant_id, owner_user_id);
        let items = self
            .lock_contact_store("contacts")
            .get(scope.as_str())
            .map(|scope_contacts| scope_contacts.values().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        ordered_contact_views(items)
    }

    pub(super) fn apply_friendship_activated(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: FriendshipActivatedPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let binding = payload
            .direct_chat_id
            .as_ref()
            .and_then(|direct_chat_id| self.direct_chat_binding(direct_chat_id));

        self.upsert_friendship_contact(
            event.tenant_id.as_str(),
            payload.user_low_id.as_str(),
            payload.user_high_id.as_str(),
            &payload,
            binding.as_ref(),
        );
        self.upsert_friendship_contact(
            event.tenant_id.as_str(),
            payload.user_high_id.as_str(),
            payload.user_low_id.as_str(),
            &payload,
            binding.as_ref(),
        );

        Ok(())
    }

    pub(super) fn apply_direct_chat_bound(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ProjectionError> {
        let payload: DirectChatBoundPayload =
            serde_json::from_str(&event.payload).map_err(ProjectionError::InvalidPayload)?;
        let binding = ContactDirectChatBindingView {
            direct_chat_id: payload.direct_chat_id.clone(),
            conversation_id: payload.conversation_id.clone(),
            bound_at: payload.bound_at.clone(),
        };
        self.lock_direct_chat_bindings("apply_direct_chat_bound")
            .insert(binding.direct_chat_id.clone(), binding.clone());

        let mut contacts = self.lock_contact_store("apply_direct_chat_bound");
        for (scope, scope_contacts) in contacts.iter_mut() {
            let Some((tenant_id, _)) = parse_contact_scope(scope.as_str()) else {
                continue;
            };
            if tenant_id != event.tenant_id {
                continue;
            }
            for contact in scope_contacts.values_mut() {
                if contact.direct_chat_id.as_deref() == Some(payload.direct_chat_id.as_str()) {
                    contact.conversation_id = Some(payload.conversation_id.clone());
                    contact.last_interaction_at = max_rfc3339(
                        contact.last_interaction_at.as_str(),
                        payload.bound_at.as_str(),
                    )
                    .to_owned();
                }
            }
        }

        Ok(())
    }

    fn upsert_friendship_contact(
        &self,
        tenant_id: &str,
        owner_user_id: &str,
        target_user_id: &str,
        payload: &FriendshipActivatedPayload,
        binding: Option<&ContactDirectChatBindingView>,
    ) {
        let scope = contact_runtime_scope(tenant_id, owner_user_id);
        let key = contact_entry_key("friendship", target_user_id);
        let mut contacts = self.lock_contact_store("upsert_friendship_contact");
        let contact = contacts
            .entry(scope)
            .or_default()
            .entry(key)
            .or_insert_with(|| ContactView {
                tenant_id: tenant_id.to_owned(),
                owner_user_id: owner_user_id.to_owned(),
                target_user_id: target_user_id.to_owned(),
                contact_type: "friendship".into(),
                relationship_state: "active".into(),
                friendship_id: payload.friendship_id.clone(),
                direct_chat_id: payload.direct_chat_id.clone(),
                conversation_id: None,
                established_at: payload.established_at.clone(),
                last_interaction_at: payload.established_at.clone(),
            });

        contact.relationship_state = "active".into();
        contact.friendship_id = payload.friendship_id.clone();
        contact.direct_chat_id = payload
            .direct_chat_id
            .clone()
            .or_else(|| contact.direct_chat_id.clone());
        contact.established_at = std::cmp::min(
            contact.established_at.clone(),
            payload.established_at.clone(),
        );
        contact.last_interaction_at = max_rfc3339(
            contact.last_interaction_at.as_str(),
            payload.established_at.as_str(),
        )
        .to_owned();

        if let Some(binding) = binding {
            contact.conversation_id = Some(binding.conversation_id.clone());
            contact.last_interaction_at = max_rfc3339(
                contact.last_interaction_at.as_str(),
                binding.bound_at.as_str(),
            )
            .to_owned();
        }
    }

    fn direct_chat_binding(&self, direct_chat_id: &str) -> Option<ContactDirectChatBindingView> {
        self.lock_direct_chat_bindings("direct_chat_binding")
            .get(direct_chat_id)
            .cloned()
    }

    fn lock_contact_store(
        &self,
        operation: &'static str,
    ) -> MutexGuard<'_, HashMap<String, HashMap<String, ContactView>>> {
        lock_contacts_mutex(&self.contacts, "contact store", operation)
    }

    fn lock_direct_chat_bindings(
        &self,
        operation: &'static str,
    ) -> MutexGuard<'_, HashMap<String, ContactDirectChatBindingView>> {
        lock_contacts_mutex(
            &self.direct_chat_bindings,
            "contact direct chat binding store",
            operation,
        )
    }
}

pub(super) fn contact_runtime_scope(tenant_id: &str, owner_user_id: &str) -> String {
    principal_scope_key(tenant_id, owner_user_id)
}

pub(super) fn contact_entry_key(contact_type: &str, target_user_id: &str) -> String {
    format!("{contact_type}:{target_user_id}")
}

pub(super) fn contact_snapshot_items(
    scope_contacts: &HashMap<String, ContactView>,
) -> Vec<ContactView> {
    ordered_contact_views(scope_contacts.values().cloned().collect::<Vec<_>>())
}

pub(super) fn contact_map_from_items(items: Vec<ContactView>) -> HashMap<String, ContactView> {
    items
        .into_iter()
        .map(|item| {
            (
                contact_entry_key(item.contact_type.as_str(), item.target_user_id.as_str()),
                item,
            )
        })
        .collect()
}

pub(super) fn ordered_contact_views(mut items: Vec<ContactView>) -> Vec<ContactView> {
    items.sort_by(|left, right| {
        right
            .last_interaction_at
            .cmp(&left.last_interaction_at)
            .then_with(|| left.target_user_id.cmp(&right.target_user_id))
    });
    items
}

pub(super) fn parse_contact_scope(scope: &str) -> Option<(&str, &str)> {
    scope.split_once(':')
}

fn max_rfc3339<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left >= right { left } else { right }
}

fn lock_contacts_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
    operation: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!(
                "warning: recovering poisoned projection-service {lock_name} lock during {operation}"
            );
            poisoned.into_inner()
        }
    }
}
