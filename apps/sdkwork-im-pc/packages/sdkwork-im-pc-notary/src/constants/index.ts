/**
 * Shared constants for the notary package
 */
import { createDefaultAvatar } from '@sdkwork/im-pc-chat';

/**
 * Default avatar for notary video call overlay
 * Used in both index.tsx and CreateNotaryTaskView.tsx
 */
export const DEFAULT_NOTARY_CALLER_AVATAR = createDefaultAvatar('user');

/**
 * Empty 1x1 transparent GIF for print template placeholders
 * Used when party identity media URLs are not available
 */
export const EMPTY_NOTARY_PRINT_IMAGE_URL = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw==';
