const DEFAULT_PORTAL_API_PORT = '18124';
const DEFAULT_PORTAL_CREDENTIALS = Object.freeze({
  tenantId: 't_demo',
  login: 'ops_demo',
  password: 'Portal#2026',
});

function isNonEmptyString(value) {
  return typeof value === 'string' && value.trim().length > 0;
}

function resolveFetchImplementation() {
  if (typeof window !== 'undefined' && typeof window.fetch === 'function') {
    return window.fetch.bind(window);
  }

  if (typeof fetch === 'function') {
    return fetch;
  }

  throw new Error('Portal HTTP data source requires fetch support.');
}

function resolvePortalApiBaseUrl() {
  if (
    typeof window !== 'undefined' &&
    isNonEmptyString(window.__CRAW_CHAT_PORTAL_API_BASE_URL__)
  ) {
    return window.__CRAW_CHAT_PORTAL_API_BASE_URL__.trim().replace(/\/+$/, '');
  }

  if (typeof window !== 'undefined' && window.location) {
    const hostname = window.location.hostname || '127.0.0.1';
    if (hostname === '127.0.0.1' || hostname === 'localhost') {
      return `http://${hostname}:${DEFAULT_PORTAL_API_PORT}`;
    }
  }

  return `http://127.0.0.1:${DEFAULT_PORTAL_API_PORT}`;
}

function buildUrl(pathname) {
  return `${resolvePortalApiBaseUrl()}${pathname}`;
}

function normalizePortalCredentials(credentials = DEFAULT_PORTAL_CREDENTIALS) {
  if (credentials === undefined) {
    return { ...DEFAULT_PORTAL_CREDENTIALS };
  }

  if (credentials === null || typeof credentials !== 'object' || Array.isArray(credentials)) {
    throw new TypeError('Portal credentials must be an object.');
  }

  return {
    tenantId: String(credentials.tenantId ?? ''),
    login: String(credentials.login ?? ''),
    password: String(credentials.password ?? ''),
  };
}

async function requestJson(pathname, { method = 'GET', token = null, body = null } = {}) {
  const fetchImpl = resolveFetchImplementation();
  const headers = {
    Accept: 'application/json',
  };

  if (body !== null) {
    headers['Content-Type'] = 'application/json';
  }

  if (isNonEmptyString(token)) {
    headers.Authorization = `Bearer ${token.trim()}`;
  }

  const response = await fetchImpl(buildUrl(pathname), {
    method,
    headers,
    body: body === null ? undefined : JSON.stringify(body),
  });

  const text = await response.text();
  const payload = text.length > 0 ? JSON.parse(text) : null;
  if (!response.ok) {
    const message =
      typeof payload?.message === 'string'
        ? payload.message
        : `${method} ${pathname} failed with status ${response.status}`;
    const error = new Error(message);
    error.status = response.status;
    error.payload = payload;
    throw error;
  }

  return payload;
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

async function requestProtectedSnapshot(pathname, token) {
  return requestJson(pathname, { token });
}

export const httpPortalDataSource = {
  async loginPortalUser(credentials) {
    const normalizedCredentials = normalizePortalCredentials(credentials);
    const payload = await requestJson('/api/v1/auth/login', {
      method: 'POST',
      body: {
        tenantId: normalizedCredentials.tenantId,
        login: normalizedCredentials.login,
        password: normalizedCredentials.password,
        clientKind: 'portal_operator',
      },
    });
    return normalizeSessionFromLogin(payload);
  },
  async bootstrapPortalSession(token) {
    if (!isNonEmptyString(token)) {
      return null;
    }

    try {
      const payload = await requestJson('/api/v1/auth/me', { token });
      return normalizeSessionFromMe(token, payload);
    } catch (error) {
      if (error?.status === 401) {
        return null;
      }
      throw error;
    }
  },
  async getPortalWorkspace(token) {
    return requestProtectedSnapshot('/api/v1/portal/workspace', token);
  },
  async getPortalHome() {
    return requestJson('/api/v1/portal/home');
  },
  async getPortalAuth() {
    return requestJson('/api/v1/portal/auth');
  },
  async getPortalDashboard(token) {
    return requestProtectedSnapshot('/api/v1/portal/dashboard', token);
  },
  async getPortalConversationsBoard(token) {
    return requestProtectedSnapshot('/api/v1/portal/conversations', token);
  },
  async getPortalRealtimeBoard(token) {
    return requestProtectedSnapshot('/api/v1/portal/realtime', token);
  },
  async getPortalMediaBoard(token) {
    return requestProtectedSnapshot('/api/v1/portal/media', token);
  },
  async getPortalAutomationBoard(token) {
    return requestProtectedSnapshot('/api/v1/portal/automation', token);
  },
  async getPortalGovernanceBoard(token) {
    return requestProtectedSnapshot('/api/v1/portal/governance', token);
  },
};
