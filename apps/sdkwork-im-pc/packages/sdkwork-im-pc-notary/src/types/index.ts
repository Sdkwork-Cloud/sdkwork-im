/**
 * Shared type exports for the notary package
 * Re-exports from @sdkwork/im-pc-types and defines local types
 */
export type { NotaryTask, Party, NotaryDocument, TimelineEvent } from '@sdkwork/im-pc-types';
export type { NotaryStaffOption, CreateNotaryTaskInput } from '../services/NotaryService';

/**
 * Form state for party editing in PartyDrawer
 */
export type PartyFormState = {
  name: string;
  phone: string;
  identityId: string;
  address: string;
  role: string;
  gender: string;
  birthDate: string;
  remarks: string;
  identityValidDateStart: string;
  identityValidDateEnd: string;
};

/**
 * Local file attachment (not yet persisted to server)
 * Used in PartyDrawer and CreateNotaryTaskView
 */
export type LocalAttachment = {
  id: string;
  url: string;
  name: string;
  file: File;
  type?: 'image' | 'video' | 'pdf' | 'file';
  partyId?: string;
};

/**
 * Stats displayed in the notary header
 */
export interface NotaryStats {
  pendingCount: number;
  completedCount: number;
  rejectedCount: number;
  totalCount: number;
}

/**
 * Video call state shared between index.tsx and CreateNotaryTaskView.tsx
 */
export interface ActiveCallState {
  isOpen: boolean;
  name: string;
  conversationId?: string;
  inviteUrl?: string;
}

/**
 * Media preview state shared between views
 */
export interface MediaPreviewState {
  isOpen: boolean;
  type: 'image' | 'video';
  url: string;
  name: string;
}

/**
 * Party identity media URLs for print overlay
 */
export interface PartyIdentityMediaUrls {
  identityFrontUrl?: string;
  identityBackUrl?: string;
  faceImageUrl?: string;
}
