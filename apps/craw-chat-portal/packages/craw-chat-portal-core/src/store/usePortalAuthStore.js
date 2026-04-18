import {
  bootstrapPortalSession,
  clearPortalSessionToken,
  getPortalWorkspace,
  loginPortalUser,
  persistPortalSessionToken,
} from '../../../craw-chat-portal-portal-api/src/index.js';
import { createStore } from '../lib/createStore.js';

const defaultState = {
  isAuthenticated: false,
  user: null,
  workspace: null,
};

function isNonEmptyString(value) {
  return typeof value === 'string' && value.trim().length > 0;
}

function isDisplayValue(value) {
  return isNonEmptyString(value) || (typeof value === 'number' && Number.isFinite(value));
}

function isValidPortalUser(user) {
  return typeof user === 'object' && user !== null && isNonEmptyString(user.name);
}

function isValidPortalSession(session) {
  return (
    typeof session?.token === 'string' &&
    session.token.length > 0 &&
    isValidPortalUser(session.user)
  );
}

function isValidPortalWorkspace(workspace) {
  return (
    typeof workspace === 'object' &&
    workspace !== null &&
    isNonEmptyString(workspace.name) &&
    isNonEmptyString(workspace.region) &&
    isNonEmptyString(workspace.tier) &&
    isNonEmptyString(workspace.supportPlan) &&
    isNonEmptyString(workspace.uptime) &&
    isDisplayValue(workspace.activeBrands) &&
    isDisplayValue(workspace.seats) &&
    (workspace.slug === undefined || isNonEmptyString(workspace.slug))
  );
}

function normalizePortalSignInCredentials(credentials) {
  if (credentials === null || typeof credentials !== 'object' || Array.isArray(credentials)) {
    throw new TypeError('Portal sign-in credentials must be provided explicitly.');
  }

  const normalized = {
    tenantId: String(credentials.tenantId ?? '').trim(),
    login: String(credentials.login ?? '').trim(),
    password: String(credentials.password ?? ''),
  };

  if (!normalized.tenantId || !normalized.login || normalized.password.length === 0) {
    throw new TypeError('Portal sign-in requires tenantId, login, and password.');
  }

  return normalized;
}

export function createPortalAuthStore() {
  const store = createStore(defaultState);

  return {
    ...store,
    async hydrate() {
      const session = await bootstrapPortalSession();

      if (!isValidPortalSession(session)) {
        clearPortalSessionToken();
        store.setState(defaultState);
        return null;
      }

      const workspace = await getPortalWorkspace(session.token);
      if (!isValidPortalWorkspace(workspace)) {
        clearPortalSessionToken();
        store.setState(defaultState);
        throw new TypeError('Portal workspace payload is invalid.');
      }

      store.setState({
        isAuthenticated: true,
        user: session.user,
        workspace,
      });

      return session;
    },
    async signIn(credentials) {
      const normalizedCredentials = normalizePortalSignInCredentials(credentials);
      const session = await loginPortalUser(normalizedCredentials);

      if (!isValidPortalSession(session)) {
        clearPortalSessionToken();
        store.setState(defaultState);
        throw new TypeError('Portal sign-in session payload is invalid.');
      }

      const workspace = await getPortalWorkspace(session.token);
      if (!isValidPortalWorkspace(workspace)) {
        clearPortalSessionToken();
        store.setState(defaultState);
        throw new TypeError('Portal workspace payload is invalid.');
      }

      persistPortalSessionToken(session.token);
      store.setState({
        isAuthenticated: true,
        user: session.user,
        workspace,
      });
      return session;
    },
    signOut() {
      clearPortalSessionToken();
      store.setState(defaultState);
    },
  };
}
