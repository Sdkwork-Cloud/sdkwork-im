import type { MembershipRole } from './membership-role';
import type { MembershipState } from './membership-state';
import type { StringMap } from './string-map';

export interface ConversationMember {
  tenantId: string;
  conversationId: string;
  memberId: string;
  principalId: string;
  principalKind: string;
  role: MembershipRole;
  state: MembershipState;
  invitedBy?: string;
  joinedAt: string;
  removedAt?: string;
  attributes: StringMap;
}
