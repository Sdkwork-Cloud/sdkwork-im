import { activePortalDataSource } from './runtime/activeDataSource.js';

const PORTAL_SESSION_STORAGE_KEY = 'craw-chat-portal.session.v1';

function withBrowserStorage(storageKey, onAvailable, fallback = null) {
  if (typeof window === 'undefined') {
    return fallback;
  }

  try {
    const storage = window[storageKey];
    if (!storage) {
      return fallback;
    }

    return onAvailable(storage);
  } catch {
    return fallback;
  }
}

function withSessionStorage(onAvailable, fallback = null) {
  return withBrowserStorage('sessionStorage', onAvailable, fallback);
}

function withLegacyLocalStorage(onAvailable, fallback = null) {
  return withBrowserStorage('localStorage', onAvailable, fallback);
}

function isValidPortalSessionToken(token) {
  return typeof token === 'string' && token.trim().length > 0;
}

function readSessionStorageToken() {
  return withSessionStorage(
    (storage) => storage.getItem(PORTAL_SESSION_STORAGE_KEY),
    null,
  );
}

function readLegacyLocalStorageToken() {
  return withLegacyLocalStorage(
    (storage) => storage.getItem(PORTAL_SESSION_STORAGE_KEY),
    null,
  );
}

function removeSessionStorageToken() {
  withSessionStorage((storage) => {
    storage.removeItem(PORTAL_SESSION_STORAGE_KEY);
    return null;
  });
}

function removeLegacyLocalStorageToken() {
  withLegacyLocalStorage((storage) => {
    storage.removeItem(PORTAL_SESSION_STORAGE_KEY);
    return null;
  });
}

function writeSessionStorageToken(token) {
  return withSessionStorage(
    (storage) => {
      storage.setItem(PORTAL_SESSION_STORAGE_KEY, token);
      return true;
    },
    false,
  );
}

export function readPortalSessionToken() {
  const sessionToken = readSessionStorageToken();

  if (isValidPortalSessionToken(sessionToken)) {
    return sessionToken;
  }

  if (sessionToken !== null) {
    removeSessionStorageToken();
  }

  const legacyToken = readLegacyLocalStorageToken();
  if (isValidPortalSessionToken(legacyToken)) {
    if (writeSessionStorageToken(legacyToken)) {
      removeLegacyLocalStorageToken();
    }

    return legacyToken;
  }

  if (legacyToken !== null) {
    removeLegacyLocalStorageToken();
  }

  return null;
}

export function persistPortalSessionToken(token) {
  if (!isValidPortalSessionToken(token)) {
    throw new TypeError('Portal session token must be a non-empty string.');
  }

  if (writeSessionStorageToken(token)) {
    removeLegacyLocalStorageToken();
  }
}

export function clearPortalSessionToken() {
  removeSessionStorageToken();
  removeLegacyLocalStorageToken();
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
