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

export function getAdminErrorStatus(error: unknown): number | null {
  if (!error || typeof error !== 'object') {
    return null;
  }

  if ('httpStatus' in error && typeof error.httpStatus === 'number' && Number.isFinite(error.httpStatus)) {
    return error.httpStatus;
  }
  if ('status' in error && typeof error.status === 'number' && Number.isFinite(error.status)) {
    return error.status;
  }
  if ('statusCode' in error && typeof error.statusCode === 'number' && Number.isFinite(error.statusCode)) {
    return error.statusCode;
  }
  if (
    'response' in error
    && error.response
    && typeof error.response === 'object'
    && 'status' in error.response
    && typeof error.response.status === 'number'
    && Number.isFinite(error.response.status)
  ) {
    return error.response.status;
  }
  if (
    'cause' in error
    && error.cause
    && typeof error.cause === 'object'
    && 'httpStatus' in error.cause
    && typeof error.cause.httpStatus === 'number'
    && Number.isFinite(error.cause.httpStatus)
  ) {
    return error.cause.httpStatus;
  }
  if (
    'cause' in error
    && error.cause
    && typeof error.cause === 'object'
    && 'status' in error.cause
    && typeof error.cause.status === 'number'
    && Number.isFinite(error.cause.status)
  ) {
    return error.cause.status;
  }
  if (
    'cause' in error
    && error.cause
    && typeof error.cause === 'object'
    && 'statusCode' in error.cause
    && typeof error.cause.statusCode === 'number'
    && Number.isFinite(error.cause.statusCode)
  ) {
    return error.cause.statusCode;
  }

  return null;
}

export function resolveAdminOperatorErrorStatus(error: unknown, fallback: string) {
  if (!(error instanceof Error)) {
    return fallback;
  }

  const message = normalizeMessage(error);
  const status = getAdminErrorStatus(error);

  if (status === 401 || message === ADMIN_SESSION_TOKEN_NOT_FOUND) {
    return 'Operator session expired. Sign in again.';
  }

  if (status === 403) {
    return 'Operator access is not permitted for this action.';
  }

  if (status === 429) {
    return 'Admin workspace is rate limited. Retry in a moment.';
  }

  if (status != null && status >= 500) {
    return 'Live admin backend is temporarily unavailable. Retry in a moment.';
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
