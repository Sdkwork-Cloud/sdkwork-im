import { createPortalSdkClient } from '../sdk/createPortalSdkClient.js';

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

function normalizeSessionFromLogin(payload) {
  if (payload === null || typeof payload !== 'object' || Array.isArray(payload)) {
    throw new TypeError('Portal login payload must be an object.');
  }

  return {
    token: payload.accessToken,
    refreshToken: payload.refreshToken ?? null,
    expiresAt: payload.expiresAt ?? null,
    user: payload.user ?? null,
    workspace: payload.workspace ?? null,
  };
}

function normalizeSessionFromMe(token, payload) {
  if (payload === null || typeof payload !== 'object' || Array.isArray(payload)) {
    throw new TypeError('Portal session payload must be an object.');
  }

  return {
    token,
    user: payload.user ?? null,
    workspace: payload.workspace ?? null,
  };
}

export const httpPortalDataSource = {
  async loginPortalUser(credentials) {
    const normalizedCredentials = normalizePortalCredentials(credentials);
    const client = await createPortalSdkClient();
    const payload = await client.portal.login({
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
      const client = await createPortalSdkClient({ authToken: token });
      const payload = await client.portal.getCurrentSession();
      return normalizeSessionFromMe(token, payload);
    } catch (error) {
      if (error?.httpStatus === 401) {
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
