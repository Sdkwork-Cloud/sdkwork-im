import type { ConversationMember } from './conversation-member';

export interface ChangeConversationMemberRoleResult {
  eventId: string;
  changedAt: string;
  previousMember: ConversationMember;
  updatedMember: ConversationMember;
}
