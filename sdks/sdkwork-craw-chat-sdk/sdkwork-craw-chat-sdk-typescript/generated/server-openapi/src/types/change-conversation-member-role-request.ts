import type { MembershipRole } from './membership-role';

export interface ChangeConversationMemberRoleRequest {
  memberId: string;
  role: MembershipRole;
}
