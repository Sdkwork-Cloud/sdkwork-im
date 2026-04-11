import type { ConversationMember } from './conversation-member';

export interface TransferConversationOwnerResult {
  eventId: string;
  transferredAt: string;
  previousOwner: ConversationMember;
  newOwner: ConversationMember;
}
