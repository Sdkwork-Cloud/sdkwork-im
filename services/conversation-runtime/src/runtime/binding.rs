use super::*;

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn conversation_business_binding(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Result<ConversationBusinessBinding, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;

        conversation
            .aggregate
            .business_binding()
            .cloned()
            .ok_or_else(|| {
                RuntimeError::ConversationBindingNotFound(format!(
                    "conversation {conversation_id} has no business binding"
                ))
            })
    }

    pub fn conversation_business_binding_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<ConversationBusinessBinding, RuntimeError> {
        if auth.actor_kind != "system" {
            self.require_active_member(
                auth.tenant_id.as_str(),
                conversation_id,
                auth.actor_id.as_str(),
            )?;
        }

        self.conversation_business_binding(auth.tenant_id.as_str(), conversation_id)
    }
}
