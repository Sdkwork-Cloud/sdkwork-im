import { activePortalDataSource } from './runtime/activeDataSource.js';
import {
  CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN,
  createCrawChatPortalUserCenterSessionStore,
  createCrawChatPortalUserCenterTokenStore,
} from './userCenter.js';
import { resolveCrawChatPortalProtectedToken } from './validation.js';

export * from './userCenter.js';
export * from './userCenterRuntime.js';
export * from './validation.js';

export function createUserCenterSessionStore(
  storagePlan = CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN,
) {
  return createCrawChatPortalUserCenterSessionStore(storagePlan);
}

export function createUserCenterTokenStore(
  storagePlan = CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN,
) {
  return createCrawChatPortalUserCenterTokenStore(storagePlan);
}

export function readPortalSessionToken() {
  return createUserCenterSessionStore().readSessionToken();
}

export function persistPortalSessionToken(token) {
  createUserCenterSessionStore().persistSessionToken(token);
}

export function clearPortalSessionToken() {
  createUserCenterSessionStore().clearSessionToken();
}

export function readPortalTokenBundle() {
  return createUserCenterTokenStore().readTokenBundle();
}

export function persistPortalTokenBundle(bundle) {
  createUserCenterTokenStore().persistTokenBundle(bundle);
}

export function clearPortalTokenBundle() {
  createUserCenterTokenStore().clearTokenBundle();
}

function normalizePortalTokenValue(value) {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : null;
}

function extractPortalTokenBundle(value) {
  if (!value || typeof value !== 'object') {
    return {};
  }

  const tokenType =
    normalizePortalTokenValue(value.tokenType)
    ?? normalizePortalTokenValue(value.token_type);

  return {
    ...(normalizePortalTokenValue(value.accessToken)
      ?? normalizePortalTokenValue(value.access_token)
      ? {
          accessToken:
            normalizePortalTokenValue(value.accessToken)
            ?? normalizePortalTokenValue(value.access_token),
        }
      : {}),
    ...(normalizePortalTokenValue(value.authToken)
      ?? normalizePortalTokenValue(value.auth_token)
      ? {
          authToken:
            normalizePortalTokenValue(value.authToken)
            ?? normalizePortalTokenValue(value.auth_token),
        }
      : {}),
    ...(normalizePortalTokenValue(value.refreshToken)
      ?? normalizePortalTokenValue(value.refresh_token)
      ? {
          refreshToken:
            normalizePortalTokenValue(value.refreshToken)
            ?? normalizePortalTokenValue(value.refresh_token),
        }
      : {}),
    ...(normalizePortalTokenValue(value.sessionToken)
      ?? normalizePortalTokenValue(value.session_token)
      ?? normalizePortalTokenValue(value.sessionId)
      ?? normalizePortalTokenValue(value.session_id)
      ?? normalizePortalTokenValue(value.token)
      ? {
          sessionToken:
            normalizePortalTokenValue(value.sessionToken)
            ?? normalizePortalTokenValue(value.session_token)
            ?? normalizePortalTokenValue(value.sessionId)
            ?? normalizePortalTokenValue(value.session_id)
            ?? normalizePortalTokenValue(value.token),
        }
      : {}),
    ...(tokenType ? { tokenType } : {}),
  };
}

function persistPortalAuthPayloadTokens(value) {
  const bundle = extractPortalTokenBundle(value);
  if (
    bundle.accessToken
    || bundle.authToken
    || bundle.refreshToken
    || bundle.sessionToken
    || bundle.tokenType
  ) {
    persistPortalTokenBundle(bundle);
  }
}

const resolveProtectedPortalToken = (token = readPortalSessionToken()) => (
  resolveCrawChatPortalProtectedToken({
    providedToken: token,
    tokenBundle: readPortalTokenBundle(),
  })
);

export async function loginPortalUser(credentials) {
  const session = await activePortalDataSource.loginPortalUser(credentials);
  persistPortalAuthPayloadTokens(session);
  return session;
}

export async function bootstrapPortalSession(token = readPortalSessionToken()) {
  const session = await activePortalDataSource.bootstrapPortalSession(token);
  persistPortalAuthPayloadTokens(session);
  return session;
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
