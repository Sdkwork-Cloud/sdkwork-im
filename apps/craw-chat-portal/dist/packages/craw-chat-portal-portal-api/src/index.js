import { activePortalDataSource } from './runtime/activeDataSource.js';

const PORTAL_SESSION_STORAGE_KEY = 'craw-chat-portal.session.v1';

function withSessionStorage(onAvailable, fallback = null) {
  if (typeof window === 'undefined') {
    return fallback;
  }

  try {
    return onAvailable(window.localStorage);
  } catch {
    return fallback;
  }
}

function isValidPortalSessionToken(token) {
  return typeof token === 'string' && token.trim().length > 0;
}

export function readPortalSessionToken() {
  const token = withSessionStorage(
    (storage) => storage.getItem(PORTAL_SESSION_STORAGE_KEY),
    null,
  );

  if (isValidPortalSessionToken(token)) {
    return token;
  }

  if (token !== null) {
    clearPortalSessionToken();
  }

  return null;
}

export function persistPortalSessionToken(token) {
  if (!isValidPortalSessionToken(token)) {
    throw new TypeError('Portal session token must be a non-empty string.');
  }

  withSessionStorage((storage) => {
    storage.setItem(PORTAL_SESSION_STORAGE_KEY, token);
    return null;
  });
}

export function clearPortalSessionToken() {
  withSessionStorage((storage) => {
    storage.removeItem(PORTAL_SESSION_STORAGE_KEY);
    return null;
  });
}

export async function loginPortalUser() {
  return activePortalDataSource.loginPortalUser();
}

export async function bootstrapPortalSession(token = readPortalSessionToken()) {
  return activePortalDataSource.bootstrapPortalSession(token);
}

export async function getPortalWorkspace() {
  return activePortalDataSource.getPortalWorkspace();
}

export async function getPortalHome() {
  return activePortalDataSource.getPortalHome();
}

export async function getPortalAuth() {
  return activePortalDataSource.getPortalAuth();
}

export async function getPortalDashboard() {
  return activePortalDataSource.getPortalDashboard();
}

export async function getPortalConversationsBoard() {
  return activePortalDataSource.getPortalConversationsBoard();
}

export async function getPortalRealtimeBoard() {
  return activePortalDataSource.getPortalRealtimeBoard();
}

export async function getPortalMediaBoard() {
  return activePortalDataSource.getPortalMediaBoard();
}

export async function getPortalAutomationBoard() {
  return activePortalDataSource.getPortalAutomationBoard();
}

export async function getPortalGovernanceBoard() {
  return activePortalDataSource.getPortalGovernanceBoard();
}
