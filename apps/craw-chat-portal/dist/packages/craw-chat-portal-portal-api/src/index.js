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

function resolveProtectedPortalToken(token = readPortalSessionToken()) {
  return token;
}

export async function loginPortalUser(credentials) {
  return activePortalDataSource.loginPortalUser(credentials);
}

export async function bootstrapPortalSession(token = readPortalSessionToken()) {
  return activePortalDataSource.bootstrapPortalSession(token);
}

export async function getPortalWorkspace(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalWorkspace(token);
}

export async function getPortalHome() {
  return activePortalDataSource.getPortalHome();
}

export async function getPortalAuth() {
  return activePortalDataSource.getPortalAuth();
}

export async function getPortalDashboard(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalDashboard(token);
}

export async function getPortalConversationsBoard(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalConversationsBoard(token);
}

export async function getPortalRealtimeBoard(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalRealtimeBoard(token);
}

export async function getPortalMediaBoard(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalMediaBoard(token);
}

export async function getPortalAutomationBoard(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalAutomationBoard(token);
}

export async function getPortalGovernanceBoard(token = resolveProtectedPortalToken()) {
  return activePortalDataSource.getPortalGovernanceBoard(token);
}
