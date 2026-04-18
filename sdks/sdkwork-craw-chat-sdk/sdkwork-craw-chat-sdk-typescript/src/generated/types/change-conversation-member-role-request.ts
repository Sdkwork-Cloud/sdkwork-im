import type { MembershipRole } from './membership-role.js';

export interface ChangeConversationMemberRoleRequest {
  memberId: string;
  role: MembershipRole;
}
