import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

import './helpers/installMockPortalDefaultDataSource.mjs';
import { resolvePortalAppRoot } from './helpers/portal-paths.mjs';

const appRoot = resolvePortalAppRoot(import.meta.url);
const explicitCredentials = Object.freeze({
  tenantId: 'tenant-alpha',
  login: 'ops.alpha',
  password: 'Sup3rSecret!2026',
});

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

function throwingStorageDouble() {
  return {
    getItem() {
      throw new Error('storage unavailable');
    },
    setItem() {
      throw new Error('storage unavailable');
    },
    removeItem() {
      throw new Error('storage unavailable');
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
  };
}

function createFocusableDouble() {
  return {
    focusCalls: 0,
    focus() {
      this.focusCalls += 1;
    },
  };
}

function createDeferred() {
  let resolve;
  let reject;

  const promise = new Promise((nextResolve, nextReject) => {
    resolve = nextResolve;
    reject = nextReject;
  });

  return { promise, resolve, reject };
}

async function flushAsyncWork(iterations = 4) {
  for (let index = 0; index < iterations; index += 1) {
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}

test('routing helpers honor operator console entry preferences and sanitize login redirects', async () => {
  const navigationModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/navigation.js',
      ),
    ).href
  );

  assert.equal(
    navigationModule.resolveConsoleEntryPath({
      isAuthenticated: true,
      lastConsolePath: '/console/media',
    }),
    '/console/media',
  );

  assert.equal(
    navigationModule.resolveConsoleEntryPath({
      isAuthenticated: true,
      lastConsolePath: '/console/media',
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/governance',
    }),
    '/console/governance',
  );

  assert.equal(
    navigationModule.resolveConsoleEntryPath({
      isAuthenticated: true,
      lastConsolePath: null,
    }),
    '/console/dashboard',
  );

  assert.equal(
    navigationModule.resolveConsoleEntryPath({
      isAuthenticated: false,
      lastConsolePath: '/console/media',
    }),
    '/login?redirect=%2Fconsole%2Fmedia',
  );

  assert.equal(
    navigationModule.resolveConsoleEntryPath({
      isAuthenticated: false,
      lastConsolePath: '/console/media',
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/governance',
    }),
    '/login?redirect=%2Fconsole%2Fgovernance',
  );

  assert.equal(
    navigationModule.resolveLoginRedirectTarget('?redirect=%2Fconsole%2Frealtime'),
    '/console/realtime',
  );

  assert.equal(
    navigationModule.resolveLoginRedirectTarget('?redirect=%2Fconsole%2Frogue'),
    '/console/dashboard',
  );

  assert.equal(
    navigationModule.resolveLoginRedirectTarget('?redirect=https://malicious.example'),
    '/console/dashboard',
  );

  assert.equal(
    navigationModule.resolveLoginRedirectTarget('', {
      lastConsolePath: '/console/realtime',
      consoleEntryMode: 'resume',
      pinnedConsolePath: '/console/automation',
    }),
    '/console/realtime',
  );

  assert.equal(
    navigationModule.resolveLoginRedirectTarget('', {
      lastConsolePath: '/console/realtime',
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/automation',
    }),
    '/console/automation',
  );

  assert.equal(
    navigationModule.resolveConsoleEntryPath({
      isAuthenticated: true,
      lastConsolePath: '/console/rogue',
    }),
    '/console/dashboard',
  );

  assert.equal(
    navigationModule.resolveUnknownPathRedirect({ isAuthenticated: false }),
    '/',
  );
});

test('shell store persists console entry preferences and hidden sidebar modules alongside route and theme preferences', async () => {
  const shellStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalShellStore.js'),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    localStorage: storageDouble(),
  };

  try {
    const firstStore = shellStoreModule.createPortalShellStore();
    firstStore.setTheme('ember');
    firstStore.setConsoleEntryMode('pinned');
    firstStore.setPinnedConsolePath('/console/governance');
    firstStore.toggleHiddenConsolePath('/console/media');
    firstStore.setLastConsolePath('/console/governance');

    const secondStore = shellStoreModule.createPortalShellStore();
    assert.equal(secondStore.getState().theme, 'ember');
    assert.equal(secondStore.getState().consoleEntryMode, 'pinned');
    assert.equal(secondStore.getState().pinnedConsolePath, '/console/governance');
    assert.deepEqual(secondStore.getState().hiddenConsolePaths, ['/console/media']);
    assert.equal(secondStore.getState().lastConsolePath, '/console/governance');
  } finally {
    global.window = originalWindow;
  }
});

test('shell store falls back to defaults when persisted shell preferences are corrupted', async () => {
  const shellStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalShellStore.js'),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    localStorage: {
      getItem() {
        return '{broken-json';
      },
      setItem() {},
      removeItem() {},
    },
  };

  try {
    const store = shellStoreModule.createPortalShellStore();
    assert.deepEqual(store.getState(), {
      settingsOpen: false,
      sidebarCollapsed: false,
      consoleEntryMode: 'resume',
      hiddenConsolePaths: [],
      pinnedConsolePath: '/console/dashboard',
      lastConsolePath: '/console/dashboard',
      theme: 'signal',
    });
  } finally {
    global.window = originalWindow;
  }
});

test('shell store accepts only known themes and console routes from persistence and setters', async () => {
  const shellStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalShellStore.js'),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    localStorage: {
      getItem() {
        return JSON.stringify({
          theme: 'rogue-theme',
          lastConsolePath: '/console/rogue',
          sidebarCollapsed: true,
          consoleEntryMode: 'rogue-mode',
          pinnedConsolePath: '/console/rogue',
          hiddenConsolePaths: ['/console/automation', '/console/rogue'],
        });
      },
      setItem() {},
      removeItem() {},
    },
  };

  try {
    const store = shellStoreModule.createPortalShellStore();

    assert.equal(store.getState().theme, 'signal');
    assert.equal(store.getState().consoleEntryMode, 'resume');
    assert.deepEqual(store.getState().hiddenConsolePaths, ['/console/automation']);
    assert.equal(store.getState().pinnedConsolePath, '/console/dashboard');
    assert.equal(store.getState().lastConsolePath, '/console/dashboard');
    assert.equal(store.getState().sidebarCollapsed, true);

    store.setTheme('atlas');
    store.setConsoleEntryMode('pinned');
    store.setPinnedConsolePath('/console/media');
    store.toggleHiddenConsolePath('/console/governance');
    store.setLastConsolePath('/console/media');
    assert.equal(store.getState().theme, 'atlas');
    assert.equal(store.getState().consoleEntryMode, 'pinned');
    assert.deepEqual(store.getState().hiddenConsolePaths, ['/console/automation', '/console/governance']);
    assert.equal(store.getState().pinnedConsolePath, '/console/media');
    assert.equal(store.getState().lastConsolePath, '/console/media');

    store.setTheme('rogue-theme');
    store.setConsoleEntryMode('rogue-mode');
    store.setPinnedConsolePath('/console/rogue');
    store.toggleHiddenConsolePath('/console/rogue');
    store.setLastConsolePath('/console/rogue');
    assert.equal(store.getState().theme, 'atlas');
    assert.equal(store.getState().consoleEntryMode, 'pinned');
    assert.deepEqual(store.getState().hiddenConsolePaths, ['/console/automation', '/console/governance']);
    assert.equal(store.getState().pinnedConsolePath, '/console/media');
    assert.equal(store.getState().lastConsolePath, '/console/media');
  } finally {
    global.window = originalWindow;
  }
});

test('shell store keeps working when browser storage writes fail', async () => {
  const shellStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalShellStore.js'),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    localStorage: throwingStorageDouble(),
  };

  try {
    const store = shellStoreModule.createPortalShellStore();
    assert.doesNotThrow(() => store.setTheme('ember'));
    assert.doesNotThrow(() => store.setConsoleEntryMode('pinned'));
    assert.doesNotThrow(() => store.setPinnedConsolePath('/console/media'));
    assert.doesNotThrow(() => store.toggleHiddenConsolePath('/console/automation'));
    assert.doesNotThrow(() => store.setLastConsolePath('/console/media'));
    assert.equal(store.getState().theme, 'ember');
    assert.equal(store.getState().consoleEntryMode, 'pinned');
    assert.deepEqual(store.getState().hiddenConsolePaths, ['/console/automation']);
    assert.equal(store.getState().pinnedConsolePath, '/console/media');
    assert.equal(store.getState().lastConsolePath, '/console/media');
  } finally {
    global.window = originalWindow;
  }
});

test('shell store resets customizable shell preferences while preserving the active console route', async () => {
  const shellStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalShellStore.js'),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    localStorage: storageDouble(),
  };

  try {
    const store = shellStoreModule.createPortalShellStore();
    store.setTheme('ember');
    store.setConsoleEntryMode('pinned');
    store.setPinnedConsolePath('/console/media');
    store.toggleHiddenConsolePath('/console/automation');
    store.toggleSidebar();
    store.setLastConsolePath('/console/governance');

    store.resetShellPreferences();

    assert.deepEqual(store.getState(), {
      settingsOpen: false,
      sidebarCollapsed: false,
      consoleEntryMode: 'resume',
      hiddenConsolePaths: [],
      pinnedConsolePath: '/console/dashboard',
      lastConsolePath: '/console/governance',
      theme: 'signal',
    });
  } finally {
    global.window = originalWindow;
  }
});

test('shell store can reset customizable shell preferences while keeping the settings dialog open', async () => {
  const shellStoreModule = await import(
    pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-core/src/store/usePortalShellStore.js'),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    localStorage: storageDouble(),
  };

  try {
    const store = shellStoreModule.createPortalShellStore();
    store.toggleSettings();
    store.setTheme('ember');
    store.setConsoleEntryMode('pinned');
    store.setPinnedConsolePath('/console/media');
    store.toggleHiddenConsolePath('/console/automation');
    store.toggleSidebar();
    store.setLastConsolePath('/console/governance');

    store.resetShellPreferences({ keepSettingsOpen: true });

    assert.deepEqual(store.getState(), {
      settingsOpen: true,
      sidebarCollapsed: false,
      consoleEntryMode: 'resume',
      hiddenConsolePaths: [],
      pinnedConsolePath: '/console/dashboard',
      lastConsolePath: '/console/governance',
      theme: 'signal',
    });
  } finally {
    global.window = originalWindow;
  }
});

test('auth store clears a stale persisted session token when bootstrap cannot restore a session', async () => {
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

  localStorage.setItem('craw-chat-portal.session.v1', 'stale-session-token');

  global.window = {
    localStorage,
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async bootstrapPortalSession(token) {
      assert.equal(token, 'stale-session-token');
      return null;
    },
  });

  try {
    const store = authStoreModule.createPortalAuthStore();
    const session = await store.hydrate();

    assert.equal(session, null);
    assert.deepEqual(store.getState(), {
      isAuthenticated: false,
      user: null,
      workspace: null,
    });
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
  }
});

test('auth store rejects malformed sign-in session payloads before fetching workspace state', async () => {
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
  let workspaceCalls = 0;

  global.window = {
    localStorage,
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async loginPortalUser() {
      return {
        token: '',
        user: null,
      };
    },
    async getPortalWorkspace() {
      workspaceCalls += 1;
      return originalDataSource.getPortalWorkspace();
    },
  });

  try {
    const store = authStoreModule.createPortalAuthStore();

    await assert.rejects(
      () => store.signIn(explicitCredentials),
      {
        name: 'TypeError',
      },
    );

    assert.equal(workspaceCalls, 0);
    assert.deepEqual(store.getState(), {
      isAuthenticated: false,
      user: null,
      workspace: null,
    });
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
  }
});

test('auth store rejects malformed workspace payloads during sign-in before persisting the session token', async () => {
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

  global.window = {
    localStorage,
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalWorkspace() {
      return {
        name: 'Nebula Commerce IM',
        region: 'CN-East / Multi-AZ',
      };
    },
  });

  try {
    const store = authStoreModule.createPortalAuthStore();

    await assert.rejects(
      () => store.signIn(explicitCredentials),
      {
        name: 'TypeError',
      },
    );

    assert.deepEqual(store.getState(), {
      isAuthenticated: false,
      user: null,
      workspace: null,
    });
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
  }
});

test('auth store clears the persisted session token when hydrate receives a malformed workspace payload', async () => {
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

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  global.window = {
    localStorage,
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async bootstrapPortalSession(token) {
      assert.equal(token, 'tenant-demo-session');
      return {
        token,
        user: {
          name: 'Lin Tao',
        },
      };
    },
    async getPortalWorkspace() {
      return {
        name: 'Nebula Commerce IM',
        region: 'CN-East / Multi-AZ',
      };
    },
  });

  try {
    const store = authStoreModule.createPortalAuthStore();

    await assert.rejects(
      () => store.hydrate(),
      {
        name: 'TypeError',
      },
    );

    assert.deepEqual(store.getState(), {
      isAuthenticated: false,
      user: null,
      workspace: null,
    });
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
  }
});

test('portal app consumes the routing helper instead of open-coded console branching', async () => {
  const appFile = await readFile(
    path.join(
      appRoot,
      'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
    ),
    'utf8',
  );

  assert.match(appFile, /resolveConsoleEntryPath/);
  assert.match(appFile, /resolveLoginRedirectTarget/);
});

test('portal app syncs the current console route into persisted shell state on initial render', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/realtime',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const persistedShellState = JSON.parse(
      localStorage.getItem('craw-chat-portal.shell.v1'),
    );

    assert.equal(persistedShellState.lastConsolePath, '/console/realtime');
    assert.match(root.innerHTML, /实时链路/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a tenant-facing loading state while the public home snapshot is in flight', async () => {
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
  const homeGate = createDeferred();
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/',
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalHome() {
      return homeGate.promise;
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(2);

    assert.match(root.innerHTML, /正在准备租户门户/);
    assert.match(root.innerHTML, /租户概览/);
    assert.doesNotMatch(root.innerHTML, /正在准备租户工作区/);

    homeGate.resolve(await originalDataSource.getPortalHome());
    await flushAsyncWork(6);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a tenant-facing loading state while the login briefing snapshot is in flight', async () => {
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
  const authGate = createDeferred();
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/login',
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalAuth() {
      return authGate.promise;
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(2);

    assert.match(root.innerHTML, /正在准备租户入口/);
    assert.match(root.innerHTML, /租户登录说明/);
    assert.doesNotMatch(root.innerHTML, /正在准备租户工作区/);

    authGate.resolve(await originalDataSource.getPortalAuth());
    await flushAsyncWork(6);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a bootstrap state instead of a blank screen while session hydration is in flight', async () => {
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
  const hydrationGate = createDeferred();
  const localStorage = storageDouble();
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async bootstrapPortalSession() {
      return hydrationGate.promise;
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);

    assert.match(root.innerHTML, /正在恢复租户控制台/);
    assert.match(root.innerHTML, /工作区会话并重新接入操作台壳层/);

    hydrationGate.resolve({
      token: 'tenant-demo-session',
      user: {
        id: 'tenant-ops-01',
        name: 'Lin Tao',
        title: 'Tenant Operations Director',
      },
    });
    await flushAsyncWork(6);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app still renders the public home when session bootstrap fails', async () => {
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
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/',
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async bootstrapPortalSession() {
      throw new Error('session bootstrap unavailable');
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(8);

    assert.match(root.innerHTML, /Craw Chat 租户门户/);
    assert.match(root.innerHTML, /门户能力面/);
    assert.doesNotMatch(root.innerHTML, /租户控制台暂时不可用/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app still renders the public login briefing when session bootstrap fails', async () => {
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
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/login',
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async bootstrapPortalSession() {
      throw new Error('session bootstrap unavailable');
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(8);

    assert.match(root.innerHTML, /演示租户入口/);
    assert.match(root.innerHTML, /Nebula Commerce IM/);
    assert.doesNotMatch(root.innerHTML, /租户控制台暂时不可用/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a recovery state and can retry when console data loading fails', async () => {
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

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalDashboard() {
      throw new Error('dashboard upstream unavailable');
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(6);

    assert.match(root.innerHTML, /模块数据暂时不可用/);
    assert.match(root.innerHTML, /重试模块同步/);
    assert.match(root.innerHTML, /总览台/);

    dataSourceModule.resetActivePortalDataSource();

    const retryButton = {
      dataset: { command: 'retry-render' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: retryButton,
    });
    await flushAsyncWork(6);

    assert.match(root.innerHTML, /当班队列压力/);
    assert.doesNotMatch(root.innerHTML, /模块数据暂时不可用/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a tenant-facing recovery state when the public home snapshot fails', async () => {
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
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/',
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalHome() {
      throw new Error('home snapshot unavailable');
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(8);

    assert.match(root.innerHTML, /租户门户暂时不可用/);
    assert.match(root.innerHTML, /最新租户概览/);
    assert.match(root.innerHTML, /重试门户概览/);
    assert.doesNotMatch(root.innerHTML, /模块数据暂时不可用/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a tenant-facing recovery state when the login briefing snapshot fails', async () => {
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
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/login',
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalAuth() {
      throw new Error('auth briefing unavailable');
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(8);

    assert.match(root.innerHTML, /租户入口暂时不可用/);
    assert.match(root.innerHTML, /租户登录说明/);
    assert.match(root.innerHTML, /重试租户入口/);
    assert.doesNotMatch(root.innerHTML, /模块数据暂时不可用/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app retries auth hydration when console bootstrap recovery is requested', async () => {
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
  let bootstrapAttempts = 0;

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async bootstrapPortalSession() {
      bootstrapAttempts += 1;
      if (bootstrapAttempts === 1) {
        throw new Error('bootstrap upstream unavailable');
      }

      return {
        token: 'tenant-demo-session',
        user: {
          id: 'tenant-ops-01',
          name: 'Lin Tao',
          title: 'Tenant Operations Director',
        },
      };
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(6);

    assert.match(root.innerHTML, /租户控制台暂时不可用/);
    assert.match(root.innerHTML, /重试控制台恢复/);
    assert.equal(bootstrapAttempts, 1);

    const retryButton = {
      dataset: { command: 'retry-render' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: retryButton,
    });
    await flushAsyncWork(8);

    assert.equal(bootstrapAttempts, 2);
    assert.equal(global.window.location.pathname, '/console/dashboard');
    assert.match(root.innerHTML, /当班队列压力/);
    assert.doesNotMatch(root.innerHTML, /重试控制台恢复/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app renders a recovery state and can retry when demo tenant sign-in fails', async () => {
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
  let signInAttempts = 0;

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/login',
    search: '?redirect=%2Fconsole%2Fdashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[name="tenantId"]': { value: explicitCredentials.tenantId },
    '[name="login"]': { value: explicitCredentials.login },
    '[name="password"]': { value: explicitCredentials.password },
  });
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async loginPortalUser() {
      signInAttempts += 1;
      if (signInAttempts === 1) {
        throw new Error('sign-in upstream unavailable');
      }

      return originalDataSource.loginPortalUser();
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(6);

    assert.match(root.innerHTML, /Nebula Commerce IM/);

    const signInButton = {
      dataset: { command: 'portal-sign-in' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: signInButton,
    });
    await flushAsyncWork(6);

    assert.equal(signInAttempts, 1);
    assert.match(root.innerHTML, /租户登录暂时不可用/);
    assert.match(root.innerHTML, /重试登录/);

    const retryButton = {
      dataset: { command: 'retry-render' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: retryButton,
    });
    await flushAsyncWork(8);

    assert.equal(signInAttempts, 2);
    assert.equal(global.window.location.pathname, '/console/dashboard');
    assert.match(root.innerHTML, /当班队列压力/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app routes demo tenant sign-in to the pinned default module when login has no redirect', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem(
    'craw-chat-portal.shell.v1',
    JSON.stringify({
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
      lastConsolePath: '/console/dashboard',
      theme: 'signal',
      sidebarCollapsed: false,
    }),
  );

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/login',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[name="tenantId"]': { value: explicitCredentials.tenantId },
    '[name="login"]': { value: explicitCredentials.login },
    '[name="password"]': { value: explicitCredentials.password },
  });
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(6);

    assert.match(root.innerHTML, /Nebula Commerce IM/);

    const signInButton = {
      dataset: { command: 'portal-sign-in' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: signInButton,
    });
    await flushAsyncWork(8);

    assert.equal(global.window.location.pathname, '/console/media');
    assert.match(root.innerHTML, /媒体与 RTC/);
    assert.doesNotMatch(root.innerHTML, /当班队列压力/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app lets operators hide a sidebar module from settings while keeping the current module visible', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    assert.match(root.innerHTML, /data-route="\/console\/automation"/);

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    const hideAutomationButton = {
      dataset: { sidebarConsolePath: '/console/automation' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: hideAutomationButton,
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /data-route="\/console\/dashboard"/);
    assert.doesNotMatch(root.innerHTML, /data-route="\/console\/automation"/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app keeps current and pinned sidebar modules locked visible inside settings', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');
  localStorage.setItem(
    'craw-chat-portal.shell.v1',
    JSON.stringify({
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
      hiddenConsolePaths: ['/console/realtime', '/console/media', '/console/automation'],
    }),
  );

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/realtime',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.match(
      root.innerHTML,
      /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/realtime"[^>]*disabled[^>]*>/,
    );
    assert.match(root.innerHTML, /当前模块始终可见/);
    assert.match(
      root.innerHTML,
      /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/media"[^>]*disabled[^>]*>/,
    );
    assert.match(root.innerHTML, /固定入口始终可见/);

    await root.dispatchEvent('click', {
      target: {
        dataset: { sidebarConsolePath: '/console/realtime' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { sidebarConsolePath: '/console/media' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.match(
      root.innerHTML,
      /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/realtime"[^>]*disabled[^>]*>/,
    );
    assert.match(
      root.innerHTML,
      /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/media"[^>]*disabled[^>]*>/,
    );
    assert.deepEqual(JSON.parse(localStorage.getItem('craw-chat-portal.shell.v1')).hiddenConsolePaths, [
      '/console/realtime',
      '/console/media',
      '/console/automation',
    ]);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app reflects shell preference changes in the command deck posture summary', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    assert.match(root.innerHTML, /当前值守姿态/);
    assert.match(root.innerHTML, /继续上次模块/);
    assert.match(root.innerHTML, /跟随最近工作面/);
    assert.match(root.innerHTML, /全部模块可见/);
    assert.match(root.innerHTML, /侧栏展开/);

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { consoleEntryMode: 'pinned' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { defaultConsolePath: '/console/media' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { sidebarConsolePath: '/console/automation' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-sidebar' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /固定进入模块/);
    assert.match(root.innerHTML, /媒体与 RTC/);
    assert.match(root.innerHTML, /已收起 1 个模块/);
    assert.match(root.innerHTML, /侧栏已收起/);
    assert.match(root.innerHTML, /当前壳层摘要/);
    assert.match(root.innerHTML, /已同步 4 项值守偏好/);
    assert.match(root.innerHTML, /当前布局已同步 4 项值守偏好，进入策略、主题与侧栏焦点都会按当前值守方式恢复。/);
    assert.match(root.innerHTML, /固定入口：媒体与 RTC/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app resets customized shell posture from the command deck without dropping the current route', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/realtime',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    for (const dataset of [
      { theme: 'ember' },
      { consoleEntryMode: 'pinned' },
      { defaultConsolePath: '/console/media' },
      { sidebarConsolePath: '/console/automation' },
      { command: 'toggle-sidebar' },
    ]) {
      await root.dispatchEvent('click', {
        target: {
          dataset,
          closest() {
            return this;
          },
        },
      });
      await flushAsyncWork();
    }

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(global.window.location.pathname, '/console/realtime');
    assert.equal(global.document.documentElement.dataset.portalTheme, 'ember');
    assert.match(root.innerHTML, /接管方式/);
    assert.match(root.innerHTML, /固定入口值守/);
    assert.match(root.innerHTML, /布局状态/);
    assert.match(root.innerHTML, /已个性化布局 · 5 项偏好/);
    assert.match(root.innerHTML, /侧栏状态/);
    assert.match(root.innerHTML, /收起/);
    assert.match(root.innerHTML, /恢复 5 项偏好/);
    assert.match(root.innerHTML, /工作台主题/);
    assert.match(root.innerHTML, /余烬班次/);
    assert.match(root.innerHTML, /已启用个性化主题/);
    assert.match(root.innerHTML, /已收起 1 个模块/);
    assert.match(root.innerHTML, /侧栏已收起/);

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'reset-shell-preferences' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(global.window.location.pathname, '/console/realtime');
    assert.equal(global.document.documentElement.dataset.portalTheme, 'signal');
    assert.match(root.innerHTML, /接管方式/);
    assert.match(root.innerHTML, /继续上次模块/);
    assert.match(root.innerHTML, /布局状态/);
    assert.match(root.innerHTML, /标准布局/);
    assert.match(root.innerHTML, /侧栏状态/);
    assert.match(root.innerHTML, /展开/);
    assert.doesNotMatch(root.innerHTML, /恢复 \d+ 项偏好/);
    assert.match(root.innerHTML, /工作台主题/);
    assert.match(root.innerHTML, /信号台/);
    assert.match(root.innerHTML, /标准主题/);
    assert.doesNotMatch(root.innerHTML, /portal-shell is-rail-collapsed/);
    assert.match(root.innerHTML, /全部模块可见/);
    assert.match(root.innerHTML, /侧栏展开/);
    assert.match(root.innerHTML, /继续上次模块/);
    assert.match(root.innerHTML, /跟随最近工作面/);
    assert.match(root.innerHTML, /data-route="\/console\/automation"/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app resets shell preferences from settings without dropping the current workspace route', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.document.documentElement = {
    dataset: {},
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { theme: 'ember' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { consoleEntryMode: 'pinned' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { defaultConsolePath: '/console/media' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { sidebarConsolePath: '/console/automation' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-sidebar' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(global.document.documentElement.dataset.portalTheme, 'ember');
    assert.match(root.innerHTML, /portal-shell is-rail-collapsed/);
    assert.doesNotMatch(root.innerHTML, /data-route="\/console\/automation"/);

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'reset-shell-preferences' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(global.window.location.pathname, '/console/dashboard');
    assert.equal(global.document.documentElement.dataset.portalTheme, 'signal');
    assert.doesNotMatch(root.innerHTML, /portal-shell is-rail-collapsed/);
    assert.match(root.innerHTML, /data-route="\/console\/automation"/);
    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /<button[\s\S]*aria-pressed="true"[\s\S]*data-console-entry-mode="resume"[\s\S]*>/);
    assert.match(root.innerHTML, /<button[\s\S]*aria-pressed="true"[\s\S]*data-default-console-path="\/console\/dashboard"[\s\S]*>/);
    assert.match(root.innerHTML, /当前壳层摘要/);
    assert.match(root.innerHTML, /标准值守布局/);
    assert.match(root.innerHTML, /当前布局仍保持标准值守方式，可按班次调整进入策略、主题和侧栏聚焦。/);
    assert.match(
      root.innerHTML,
      /<button[^>]*data-command="reset-shell-preferences"[^>]*disabled[^>]*>[\s\S]*当前已是标准布局[\s\S]*<\/button>/,
    );

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app ignores disabled settings actions when click events are dispatched programmatically', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.document.documentElement = {
    dataset: {},
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);
    assert.match(root.innerHTML, /<button[^>]*data-command="reset-shell-preferences"[^>]*disabled[^>]*>/);

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'reset-shell-preferences' },
        disabled: true,
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);
    assert.match(root.innerHTML, /<button[^>]*data-command="reset-shell-preferences"[^>]*disabled[^>]*>/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app restores focus to the settings dialog after reset reopens the standard layout state', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[data-command="close-settings"]': closeSettingsButton,
  });
  global.document.documentElement = {
    dataset: {},
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 1);

    await root.dispatchEvent('click', {
      target: {
        dataset: { theme: 'ember' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'reset-shell-preferences' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.equal(closeSettingsButton.focusCalls, 2);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app ignores stale module renders when a slower route resolves after a newer route', async () => {
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
  const dashboardGate = createDeferred();
  const mediaGate = createDeferred();
  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble();
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalDashboard() {
      return dashboardGate.promise;
    },
    async getPortalMediaBoard() {
      return mediaGate.promise;
    },
  });

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork(6);

    const mediaRouteButton = {
      dataset: { route: '/console/media' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: mediaRouteButton,
    });
    await flushAsyncWork(2);

    mediaGate.resolve(await originalDataSource.getPortalMediaBoard());
    await flushAsyncWork(6);

    assert.equal(global.window.location.pathname, '/console/media');
    assert.match(root.innerHTML, /媒体与 RTC/);

    dashboardGate.resolve(await originalDataSource.getPortalDashboard());
    await flushAsyncWork(6);

    assert.equal(global.window.location.pathname, '/console/media');
    assert.match(root.innerHTML, /媒体与 RTC/);
    assert.doesNotMatch(root.innerHTML, /当班队列压力/);

    teardown();
  } finally {
    dataSourceModule.resetActivePortalDataSource();
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app closes the settings panel on Escape for operator workflows', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);

    global.window.dispatchEvent({
      type: 'keydown',
      key: 'Escape',
      preventDefault() {},
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="false"/);
    assert.doesNotMatch(root.innerHTML, /id="portal-settings-panel"/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app moves focus into the settings dialog when operators open it', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[data-command="close-settings"]': closeSettingsButton,
  });
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 1);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app restores focus to the selected theme control after settings rerender', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();
  const emberThemeButton = createFocusableDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[data-command="close-settings"]': closeSettingsButton,
    '#portal-settings-panel [data-theme="ember"]': emberThemeButton,
  });
  global.document.documentElement = {
    dataset: {},
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 1);

    await root.dispatchEvent('click', {
      target: {
        dataset: { theme: 'ember' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(global.document.documentElement.dataset.portalTheme, 'ember');
    assert.equal(emberThemeButton.focusCalls, 1);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app restores focus to the settings sidebar action instead of the rail action after rerender', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();
  const settingsToggleSidebarButton = createFocusableDouble();
  const railToggleSidebarButton = createFocusableDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[data-command="close-settings"]': closeSettingsButton,
    '#portal-settings-panel [data-command="toggle-sidebar"]': settingsToggleSidebarButton,
    '[data-command="toggle-sidebar"]': railToggleSidebarButton,
  });
  global.document.documentElement = {
    dataset: {},
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 1);

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-sidebar' },
        closest(selector) {
          if (selector === '#portal-settings-panel') {
            return { id: 'portal-settings-panel' };
          }

          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /portal-shell is-rail-collapsed/);
    assert.equal(settingsToggleSidebarButton.focusCalls, 1);
    assert.equal(railToggleSidebarButton.focusCalls, 0);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app restores focus to the settings trigger when operators close the dialog', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();
  const restoredSettingsTrigger = createFocusableDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = createDocumentDouble({
    '[data-command="close-settings"]': closeSettingsButton,
    '[data-command="toggle-settings"]': restoredSettingsTrigger,
  });
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    global.window.dispatchEvent({
      type: 'keydown',
      key: 'Escape',
      preventDefault() {},
    });
    await flushAsyncWork();

    assert.equal(restoredSettingsTrigger.focusCalls, 1);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app traps keyboard focus inside the settings dialog', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();
  const signalThemeButton = createFocusableDouble();
  const atlasThemeButton = createFocusableDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
    activeElement: closeSettingsButton,
    querySelector(selector) {
      if (selector === '[data-command="close-settings"]') {
        return closeSettingsButton;
      }

      return null;
    },
    querySelectorAll(selector) {
      if (selector === '#portal-settings-panel button') {
        return [closeSettingsButton, signalThemeButton, atlasThemeButton];
      }

      return [];
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 1);

    global.document.activeElement = atlasThemeButton;
    global.window.dispatchEvent({
      type: 'keydown',
      key: 'Tab',
      shiftKey: false,
      preventDefault() {},
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 2);

    global.document.activeElement = closeSettingsButton;
    global.window.dispatchEvent({
      type: 'keydown',
      key: 'Tab',
      shiftKey: true,
      preventDefault() {},
    });
    await flushAsyncWork();

    assert.equal(atlasThemeButton.focusCalls, 1);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app traps keyboard focus using only enabled settings controls', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();
  const closeSettingsButton = createFocusableDouble();
  const toggleSidebarButton = createFocusableDouble();
  const disabledResetButton = {
    disabled: true,
    focusCalls: 0,
    focus() {
      this.focusCalls += 1;
    },
  };

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
    activeElement: closeSettingsButton,
    querySelector(selector) {
      if (selector === '[data-command="close-settings"]') {
        return closeSettingsButton;
      }

      return null;
    },
    querySelectorAll(selector) {
      if (selector === '#portal-settings-panel button') {
        return [closeSettingsButton, toggleSidebarButton, disabledResetButton];
      }

      return [];
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    await root.dispatchEvent('click', {
      target: {
        dataset: { command: 'toggle-settings' },
        closest() {
          return this;
        },
      },
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 1);

    global.document.activeElement = toggleSidebarButton;
    global.window.dispatchEvent({
      type: 'keydown',
      key: 'Tab',
      shiftKey: false,
      preventDefault() {},
    });
    await flushAsyncWork();

    assert.equal(closeSettingsButton.focusCalls, 2);
    assert.equal(disabledResetButton.focusCalls, 0);

    global.document.activeElement = closeSettingsButton;
    global.window.dispatchEvent({
      type: 'keydown',
      key: 'Tab',
      shiftKey: true,
      preventDefault() {},
    });
    await flushAsyncWork();

    assert.equal(toggleSidebarButton.focusCalls, 1);
    assert.equal(disabledResetButton.focusCalls, 0);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app resets transient settings state across sign-out and next sign-in', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);

    const signOutButton = {
      dataset: { command: 'sign-out' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: signOutButton,
    });
    await flushAsyncWork();

    assert.equal(global.window.location.pathname, '/');
    assert.doesNotMatch(root.innerHTML, /id="portal-settings-panel"/);

    const loginRouteButton = {
      dataset: { route: '/login?redirect=%2Fconsole%2Fdashboard' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: loginRouteButton,
    });
    await flushAsyncWork();

    const demoSignInButton = {
      dataset: { command: 'portal-sign-in' },
      closest() {
        return this;
      },
    };

    global.document = createDocumentDouble({
      '[name="tenantId"]': { value: explicitCredentials.tenantId },
      '[name="login"]': { value: explicitCredentials.login },
      '[name="password"]': { value: explicitCredentials.password },
    });

    await root.dispatchEvent('click', {
      target: demoSignInButton,
    });
    await flushAsyncWork(6);

    assert.equal(global.window.location.pathname, '/console/dashboard');
    assert.match(root.innerHTML, /aria-expanded="false"/);
    assert.doesNotMatch(root.innerHTML, /id="portal-settings-panel"/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app closes the settings panel when navigating to another console module', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);

    const routeButton = {
      dataset: { route: '/console/media' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: routeButton,
    });
    await flushAsyncWork(6);

    assert.equal(global.window.location.pathname, '/console/media');
    assert.match(root.innerHTML, /aria-expanded="false"/);
    assert.doesNotMatch(root.innerHTML, /id="portal-settings-panel"/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app closes the settings panel when browser history changes route via popstate', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      closest() {
        return this;
      },
    };

    await root.dispatchEvent('click', {
      target: settingsButton,
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);

    global.window.location.pathname = '/console/media';
    global.window.dispatchEvent({
      type: 'popstate',
    });
    await flushAsyncWork(6);

    assert.match(root.innerHTML, /媒体与 RTC/);
    assert.match(root.innerHTML, /aria-expanded="false"/);
    assert.doesNotMatch(root.innerHTML, /id="portal-settings-panel"/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});

test('portal app resolves interactive clicks even when the event target is a nested non-element node', async () => {
  const appModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/app/PortalProductApp.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDocument = global.document;
  const originalEvent = global.Event;
  const localStorage = storageDouble();

  localStorage.setItem('craw-chat-portal.session.v1', 'tenant-demo-session');

  const root = createRootDouble();

  global.window = createWindowDouble({
    pathname: '/console/dashboard',
    localStorage,
  });
  global.document = {
    documentElement: {
      dataset: {},
    },
  };
  global.Event = class Event {
    constructor(type) {
      this.type = type;
    }
  };

  try {
    const teardown = appModule.createPortalProductApp(root);
    await flushAsyncWork();

    const settingsButton = {
      dataset: { command: 'toggle-settings' },
      matches(selector) {
        return selector === '[data-route],[data-command],[data-theme]';
      },
      parentElement: null,
    };

    const nestedTextNode = {
      parentElement: settingsButton,
    };

    await root.dispatchEvent('click', {
      target: nestedTextNode,
    });
    await flushAsyncWork();

    assert.match(root.innerHTML, /aria-expanded="true"/);
    assert.match(root.innerHTML, /id="portal-settings-panel"/);

    const routeButton = {
      dataset: { route: '/console/governance' },
      matches(selector) {
        return selector === '[data-route],[data-command],[data-theme]';
      },
      parentElement: null,
    };

    const nestedRouteText = {
      parentElement: routeButton,
    };

    await root.dispatchEvent('click', {
      target: nestedRouteText,
    });
    await flushAsyncWork(6);

    assert.equal(global.window.location.pathname, '/console/governance');
    assert.match(root.innerHTML, /治理/);

    teardown();
  } finally {
    global.window = originalWindow;
    global.document = originalDocument;
    global.Event = originalEvent;
  }
});
