import type { ConversationMember } from './conversation-member.js';

export interface TransferConversationOwnerResult {
  eventId: string;
  transferredAt: string;
  previousOwner: ConversationMember;
  newOwner: ConversationMember;
}
