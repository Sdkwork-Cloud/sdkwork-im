import type { MembershipRole } from './membership-role.js';
import type { MembershipState } from './membership-state.js';
import type { StringMap } from './string-map.js';

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
