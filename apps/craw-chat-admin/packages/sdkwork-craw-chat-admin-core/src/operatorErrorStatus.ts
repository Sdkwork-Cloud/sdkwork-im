import { AdminApiError } from 'sdkwork-craw-chat-admin-admin-api';

const ADMIN_REQUEST_FAILED_PREFIX = 'Admin request failed with status ';
const ADMIN_SESSION_TOKEN_NOT_FOUND = 'Admin session token not found';
const TAURI_INVOKE_UNAVAILABLE = 'Tauri invoke bridge is unavailable.';
const TECHNICAL_TRANSPORT_MESSAGE_PATTERNS = [
  /\bTypeError\b/i,
  /\bAxiosError\b/i,
  /\bNetworkError\b/i,
  /\bAbortError\b/i,
  /^fetch failed$/i,
  /\bFailed to fetch\b/i,
  /\bsocket hang up\b/i,
  /\bECONN(?:RESET|REFUSED|ABORTED)\b/i,
  /\bENOTFOUND\b/i,
  /\bEAI_AGAIN\b/i,
  /\bETIMEDOUT\b/i,
  /\bnetwork request failed\b/i,
  /\btimeout(?: of \d+ms exceeded)?\b/i,
  /\bUnexpected token\b/i,
  /\bJSON\.parse\b/i,
  /\bCORS\b/i,
  /\b(?:127\.0\.0\.1|localhost)\b/i,
  /https?:\/\//i,
];

function normalizeMessage(error: Error) {
  return error.message.trim();
}

function isTechnicalTransportMessage(message: string) {
  if (!message) {
    return false;
  }

  if (message.includes('\n') || message.includes('\r')) {
    return true;
  }

  return TECHNICAL_TRANSPORT_MESSAGE_PATTERNS.some((pattern) => pattern.test(message));
}

function resolveOperatorSafeMessage(message: string, fallback: string) {
  if (!message || message.startsWith(ADMIN_REQUEST_FAILED_PREFIX)) {
    return fallback;
  }

  return isTechnicalTransportMessage(message) ? fallback : message;
}

export function resolveAdminOperatorMessage(message: string | null | undefined, fallback: string) {
  if (typeof message !== 'string') {
    return fallback;
  }

  return resolveOperatorSafeMessage(message.trim(), fallback);
}

export function resolveAdminOperatorErrorStatus(error: unknown, fallback: string) {
  if (!(error instanceof Error)) {
    return fallback;
  }

  const message = normalizeMessage(error);

  if (error instanceof AdminApiError) {
    if (error.status === 401 || message === ADMIN_SESSION_TOKEN_NOT_FOUND) {
      return 'Operator session expired. Sign in again.';
    }

    if (error.status === 403) {
      return 'Operator access is not permitted for this action.';
    }

    if (error.status === 429) {
      return 'Admin workspace is rate limited. Retry in a moment.';
    }

    if (error.status >= 500) {
      return 'Live admin backend is temporarily unavailable. Retry in a moment.';
    }

    return resolveAdminOperatorMessage(message, fallback);
  }

  if (message === ADMIN_SESSION_TOKEN_NOT_FOUND) {
    return 'Operator session expired. Sign in again.';
  }

  if (message === TAURI_INVOKE_UNAVAILABLE) {
    return 'Desktop runtime bridge is unavailable. Retry from the web workspace or restart the desktop shell.';
  }

  if (message.startsWith(ADMIN_REQUEST_FAILED_PREFIX)) {
    return fallback;
  }

  return resolveAdminOperatorMessage(message, fallback);
}
