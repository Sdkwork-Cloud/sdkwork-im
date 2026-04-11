use crate::{ConversationMemberDirectoryEntry, TimelineProjectionService};
use im_domain_core::conversation::MembershipRole;

use super::scope_key;

impl TimelineProjectionService {
    pub fn member_directory(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Vec<ConversationMemberDirectoryEntry> {
        let mut items = self
            .members
            .lock()
            .expect("member store should lock")
            .get(scope_key(tenant_id, conversation_id).as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| member.tenant_id == tenant_id && member.is_active())
                    .map(ConversationMemberDirectoryEntry::from_member)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        items.sort_by(|left, right| {
            member_directory_role_rank(&left.role)
                .cmp(&member_directory_role_rank(&right.role))
                .then_with(|| left.joined_at.cmp(&right.joined_at))
                .then_with(|| left.principal_id.cmp(&right.principal_id))
        });
        items
    }
}

fn member_directory_role_rank(role: &MembershipRole) -> u8 {
    match role {
        MembershipRole::Owner => 0,
        MembershipRole::Admin => 1,
        MembershipRole::Member => 2,
        MembershipRole::Guest => 3,
    }
}
