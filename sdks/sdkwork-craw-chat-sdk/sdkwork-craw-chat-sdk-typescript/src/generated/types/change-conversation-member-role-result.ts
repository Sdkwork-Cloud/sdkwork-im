import type { ConversationMember } from './conversation-member.js';

export interface ChangeConversationMemberRoleResult {
  eventId: string;
  changedAt: string;
  previousMember: ConversationMember;
  updatedMember: ConversationMember;
}
