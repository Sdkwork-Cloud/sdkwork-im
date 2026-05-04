import { createPortalSdkClient } from '../sdk/createPortalSdkClient.js';
import {
  createCrawChatPortalCanonicalUserCenterConfig,
  createCrawChatPortalUserCenterRuntimeClient,
  createUserCenterTokenStore,
} from '../../userCenterRuntime.js';

function isNonEmptyString(value) {
  return typeof value === 'string' && value.trim().length > 0;
}

function normalizePortalCredentials(credentials) {
  if (credentials === null || typeof credentials !== 'object' || Array.isArray(credentials)) {
    throw new TypeError('Portal credentials must be provided as an object.');
  }

  const normalized = {
    tenantId: String(credentials.tenantId ?? '').trim(),
    login: String(credentials.login ?? '').trim(),
    password: String(credentials.password ?? ''),
  };

  if (!isNonEmptyString(normalized.tenantId) || !isNonEmptyString(normalized.login) || normalized.password.length === 0) {
    throw new TypeError('Portal credentials must include tenantId, login, and password.');
  }

  return normalized;
}

function normalizeOptionalToken(value) {
  return isNonEmptyString(value) ? value.trim() : null;
}

function extractPortalTokenBundle(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return {};
  }

  return {
    ...(normalizeOptionalToken(value.accessToken ?? value.access_token)
      ? { accessToken: normalizeOptionalToken(value.accessToken ?? value.access_token) }
      : {}),
    ...(normalizeOptionalToken(value.authToken ?? value.auth_token)
      ? { authToken: normalizeOptionalToken(value.authToken ?? value.auth_token) }
      : {}),
    ...(normalizeOptionalToken(value.refreshToken ?? value.refresh_token)
      ? { refreshToken: normalizeOptionalToken(value.refreshToken ?? value.refresh_token) }
      : {}),
    ...(normalizeOptionalToken(
      value.sessionToken
        ?? value.session_token
        ?? value.sessionId
        ?? value.session_id
        ?? value.token,
    )
      ? {
          sessionToken: normalizeOptionalToken(
            value.sessionToken
              ?? value.session_token
              ?? value.sessionId
              ?? value.session_id
              ?? value.token,
          ),
        }
      : {}),
    ...(normalizeOptionalToken(value.tokenType ?? value.token_type)
      ? { tokenType: normalizeOptionalToken(value.tokenType ?? value.token_type) }
      : {}),
  };
}

function resolvePortalSessionToken(tokenBundle, fallbackToken = null) {
  return (
    tokenBundle.authToken
    ?? tokenBundle.accessToken
    ?? tokenBundle.sessionToken
    ?? normalizeOptionalToken(fallbackToken)
  );
}

function resolvePortalSessionMetadata(payload) {
  const expiresAt =
    payload.expiresAt
    ?? payload.expires_at
    ?? null;

  return {
    expiresAt,
    user: payload.user ?? null,
    workspace: payload.workspace ?? null,
  };
}

function createPortalUserCenterTokenStore() {
  const runtimeConfig = createCrawChatPortalCanonicalUserCenterConfig();
  return createUserCenterTokenStore(runtimeConfig.storagePlan, {
    bundleMemoryCache: false,
  });
}

function normalizeSessionFromLogin(payload) {
  if (payload === null || typeof payload !== 'object' || Array.isArray(payload)) {
    throw new TypeError('Portal login payload must be an object.');
  }

  const tokenBundle = extractPortalTokenBundle(payload);
  const token = resolvePortalSessionToken(tokenBundle);
  if (!token) {
    throw new TypeError('Portal login payload must include a protected token.');
  }

  return {
    token,
    ...(tokenBundle.accessToken ? { accessToken: tokenBundle.accessToken } : {}),
    ...(tokenBundle.authToken ? { authToken: tokenBundle.authToken } : {}),
    ...(tokenBundle.refreshToken ? { refreshToken: tokenBundle.refreshToken } : {}),
    ...(tokenBundle.sessionToken ? { sessionToken: tokenBundle.sessionToken } : {}),
    ...(tokenBundle.tokenType ? { tokenType: tokenBundle.tokenType } : {}),
    ...resolvePortalSessionMetadata(payload),
  };
}

function normalizeSessionFromMe(token, payload) {
  if (payload === null || typeof payload !== 'object' || Array.isArray(payload)) {
    throw new TypeError('Portal session payload must be an object.');
  }

  const tokenBundle = extractPortalTokenBundle(payload);
  const resolvedToken = resolvePortalSessionToken(tokenBundle, token);
  if (!resolvedToken) {
    throw new TypeError('Portal session payload must include a protected token.');
  }

  return {
    token: resolvedToken,
    ...(tokenBundle.accessToken ? { accessToken: tokenBundle.accessToken } : {}),
    ...(tokenBundle.authToken ? { authToken: tokenBundle.authToken } : {}),
    ...(tokenBundle.refreshToken ? { refreshToken: tokenBundle.refreshToken } : {}),
    ...(tokenBundle.sessionToken ? { sessionToken: tokenBundle.sessionToken } : {}),
    ...(tokenBundle.tokenType ? { tokenType: tokenBundle.tokenType } : {}),
    ...resolvePortalSessionMetadata(payload),
  };
}

export const httpPortalDataSource = {
  async loginPortalUser(credentials) {
    const normalizedCredentials = normalizePortalCredentials(credentials);
    const tokenStore = createPortalUserCenterTokenStore();
    const client = createCrawChatPortalUserCenterRuntimeClient({}, {
      tokenStore,
    });
    const payload = await client.loginSession({
      tenantId: normalizedCredentials.tenantId,
      login: normalizedCredentials.login,
      password: normalizedCredentials.password,
      clientKind: 'portal_operator',
    });
    return normalizeSessionFromLogin(payload);
  },
  async bootstrapPortalSession(token) {
    if (!isNonEmptyString(token)) {
      return null;
    }

    try {
      const tokenStore = createPortalUserCenterTokenStore();
      const existingBundle = tokenStore.readTokenBundle();
      if (!isNonEmptyString(existingBundle.authToken) && !isNonEmptyString(existingBundle.accessToken)) {
        tokenStore.persistTokenBundle({
          accessToken: token.trim(),
        });
      }

      const client = createCrawChatPortalUserCenterRuntimeClient({}, {
        tokenStore,
      });
      const payload = await client.bootstrapSession();
      return normalizeSessionFromMe(token, payload);
    } catch (error) {
      if (error?.httpStatus === 401 || error?.status === 401) {
        return null;
      }
      throw error;
    }
  },
  async getPortalWorkspace(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getWorkspace();
  },
  async getPortalHome() {
    const client = await createPortalSdkClient();
    return client.portal.getHome();
  },
  async getPortalAuth() {
    const client = await createPortalSdkClient();
    return client.portal.getAuth();
  },
  async getPortalDashboard(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getDashboard();
  },
  async getPortalConversationsBoard(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getConversations();
  },
  async getPortalRealtimeBoard(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getRealtime();
  },
  async getPortalMediaBoard(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getMedia();
  },
  async getPortalAutomationBoard(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getAutomation();
  },
  async getPortalGovernanceBoard(token) {
    const client = await createPortalSdkClient({ authToken: token });
    return client.portal.getGovernance();
  },
};
