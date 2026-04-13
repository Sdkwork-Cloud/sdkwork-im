import assert from 'node:assert/strict';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

const appRoot = path.resolve('apps/craw-chat-portal');

function storageDouble() {
  const store = new Map();

  return {
    getItem(key) {
      return store.has(key) ? store.get(key) : null;
    },
    setItem(key, value) {
      store.set(key, String(value));
    },
    removeItem(key) {
      store.delete(key);
    },
  };
}

function createWindowDouble({ pathname = '/', search = '', localStorage = storageDouble() } = {}) {
  const listeners = new Map();

  const windowDouble = {
    localStorage,
    location: {
      pathname,
      search,
    },
    history: {
      pushState(_state, _title, nextPath) {
        const [nextPathname, nextSearch = ''] = String(nextPath).split('?');
        windowDouble.location.pathname = nextPathname || '/';
        windowDouble.location.search = nextSearch ? `?${nextSearch}` : '';
      },
      replaceState(_state, _title, nextPath) {
        const [nextPathname, nextSearch = ''] = String(nextPath).split('?');
        windowDouble.location.pathname = nextPathname || '/';
        windowDouble.location.search = nextSearch ? `?${nextSearch}` : '';
      },
    },
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
    removeEventListener(type) {
      listeners.delete(type);
    },
    dispatchEvent(event) {
      const listener = listeners.get(event.type);
      if (listener) {
        listener(event);
      }
      return true;
    },
  };

  return windowDouble;
}

function createRootDouble() {
  const listeners = new Map();

  return {
    innerHTML: '',
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
    removeEventListener(type) {
      listeners.delete(type);
    },
    dispatchEvent(type, event) {
      const listener = listeners.get(type);
      if (listener) {
        return listener(event);
      }
      return undefined;
    },
  };
}

function createDocumentDouble(selectors = {}) {
  return {
    documentElement: {
      dataset: {},
    },
    querySelector(selector) {
      return selectors[selector] ?? null;
    },
    querySelectorAll() {
      return [];
    },
  };
}

async function flushAsyncWork(iterations = 4) {
  for (let index = 0; index < iterations; index += 1) {
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}

test('portal auth store forwards explicit credentials to the portal-api login seam', async () => {
  const authStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalAuthStore.js'),
    ).href
  );
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const localStorage = storageDouble();
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;
  let capturedCredentials = null;
  let capturedWorkspaceToken = null;

  global.window = {
    localStorage,
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async loginPortalUser(credentials) {
      capturedCredentials = credentials;
      return {
        token: 'portal_access_token_demo',
        user: {
          name: 'Lin Tao',
          role: 'Tenant Operations Lead',
          email: 'lin.tao@nebula-commerce.example',
        },
      };
    },
    async getPortalWorkspace() {
      capturedWorkspaceToken = arguments[0];
      return {
        name: 'Nebula Commerce IM',
        slug: 'nebula-commerce-im',
        tier: 'Enterprise',
        region: 'CN-East / Multi-AZ',
        supportPlan: 'Platinum',
        seats: 84,
        activeBrands: 12,
        uptime: '99.983%',
      };
    },
  });

  try {
    const store = authStoreModule.createPortalAuthStore();
    await store.signIn({
      tenantId: 't_demo',
      login: 'ops_demo',
      password: 'Portal#2026',
    });

    assert.deepEqual(capturedCredentials, {
      tenantId: 't_demo',
      login: 'ops_demo',
      password: 'Portal#2026',
    });
    assert.equal(capturedWorkspaceToken, 'portal_access_token_demo');
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), 'portal_access_token_demo');
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
  }
});

test('portal app reads login form credentials and routes real sign-in through the auth store', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;
  const root = createRootDouble();
  let capturedCredentials = null;
  let capturedWorkspaceToken = null;

  global.window = createWindowDouble({
    pathname: '/login',
    search: '?redirect=%2Fconsole%2Fdashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[name="tenantId"]': { value: 't_demo' },
    '[name="login"]': { value: 'ops_demo' },
    '[name="password"]': { value: 'Portal#2026' },
  });
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async loginPortalUser(credentials) {
      capturedCredentials = credentials;
      return {
        token: 'portal_access_token_live',
        user: {
          name: 'Lin Tao',
          role: 'Tenant Operations Lead',
          email: 'lin.tao@nebula-commerce.example',
        },
      };
    },
    async getPortalWorkspace(token) {
      capturedWorkspaceToken = token;
      return {
        name: 'Nebula Commerce IM',
        slug: 'nebula-commerce-im',
        tier: 'Enterprise',
        region: 'CN-East / Multi-AZ',
        supportPlan: 'Platinum',
        seats: 84,
        activeBrands: 12,
        uptime: '99.983%',
      };
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(6);

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'portal-sign-in' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork(8);

    assert.deepEqual(capturedCredentials, {
      tenantId: 't_demo',
      login: 'ops_demo',
      password: 'Portal#2026',
    });
    assert.equal(capturedWorkspaceToken, 'portal_access_token_live');
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), 'portal_access_token_live');
    assert.equal(global.window.location.pathname, '/console/dashboard');

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal auth page renders tenant, login, and password inputs for real sign-in', async () => {
  const authPageModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-auth/src/index.js'),
    ).href
  );

  const html = await authPageModule.renderPortalAuthPage();

  assert.match(html, /name="tenantId"/);
  assert.match(html, /name="login"/);
  assert.match(html, /name="password"/);
  assert.match(html, /data-command="portal-sign-in"/);
});
