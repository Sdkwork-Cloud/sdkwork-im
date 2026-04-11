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

      const workspace = await getPortalWorkspace();
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
    async signIn() {
      const session = await loginPortalUser();

      if (!isValidPortalSession(session)) {
        clearPortalSessionToken();
        store.setState(defaultState);
        throw new TypeError('Portal sign-in session payload is invalid.');
      }

      const workspace = await getPortalWorkspace();
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
