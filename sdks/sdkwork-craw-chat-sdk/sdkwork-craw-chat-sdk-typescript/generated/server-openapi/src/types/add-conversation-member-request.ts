import type { MembershipRole } from './membership-role';

export interface AddConversationMemberRequest {
  principalId: string;
  principalKind: string;
  role: MembershipRole;
}
