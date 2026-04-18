import type { MembershipRole } from './membership-role.js';

export interface AddConversationMemberRequest {
  principalId: string;
  principalKind: string;
  role: MembershipRole;
}
