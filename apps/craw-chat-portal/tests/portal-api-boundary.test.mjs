import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

import { resolvePortalAppRoot } from './helpers/portal-paths.mjs';

const appRoot = resolvePortalAppRoot(import.meta.url);

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

test('portal-api exposes an active data source that can be swapped later for SDK-backed clients', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const requiredMethods = [
    'bootstrapPortalSession',
    'getPortalAuth',
    'getPortalAutomationBoard',
    'getPortalConversationsBoard',
    'getPortalDashboard',
    'getPortalGovernanceBoard',
    'getPortalHome',
    'getPortalMediaBoard',
    'getPortalRealtimeBoard',
    'getPortalWorkspace',
    'loginPortalUser',
  ];

  for (const method of requiredMethods) {
    assert.equal(typeof dataSourceModule.activePortalDataSource[method], 'function');
  }
});

test('portal-api runtime can swap and reset the active data source for future SDK-backed wiring', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  assert.equal(typeof dataSourceModule.setActivePortalDataSource, 'function');
  assert.equal(typeof dataSourceModule.resetActivePortalDataSource, 'function');

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const replacementDataSource = {
    ...originalDataSource,
    async getPortalDashboard() {
      return { source: 'replacement-runtime' };
    },
  };

  try {
    dataSourceModule.setActivePortalDataSource(replacementDataSource);
    assert.deepEqual(
      await dataSourceModule.activePortalDataSource.getPortalDashboard(),
      { source: 'replacement-runtime' },
    );
  } finally {
    dataSourceModule.resetActivePortalDataSource();
  }

  assert.equal(
    dataSourceModule.activePortalDataSource.getPortalDashboard,
    originalDataSource.getPortalDashboard,
  );
  assert.notEqual(
    dataSourceModule.activePortalDataSource.getPortalDashboard,
    replacementDataSource.getPortalDashboard,
  );
});

test('portal-api runtime rejects invalid method overrides and preserves the active data source', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  assert.throws(
    () => {
      dataSourceModule.setActivePortalDataSource({
        getPortalDashboard: null,
      });
    },
    {
      name: 'TypeError',
    },
  );

  assert.equal(
    dataSourceModule.getActivePortalDataSource(),
    originalDataSource,
  );
  assert.equal(typeof dataSourceModule.activePortalDataSource.getPortalDashboard, 'function');
});

test('portal-api runtime rejects non-object override payloads and preserves the active data source', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  for (const invalidOverrides of [null, 5, 'portal', []]) {
    assert.throws(
      () => {
        dataSourceModule.setActivePortalDataSource(invalidOverrides);
      },
      {
        name: 'TypeError',
      },
    );
  }

  assert.equal(
    dataSourceModule.getActivePortalDataSource(),
    originalDataSource,
  );
  assert.equal(typeof dataSourceModule.activePortalDataSource.getPortalDashboard, 'function');
});

test('portal-api runtime rejects non-plain-object override payloads so prototype-backed adapters fail fast', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  class PrototypeBackedPortalSource {
    async getPortalDashboard() {
      return { source: 'prototype-backed-runtime' };
    }
  }

  assert.throws(
    () => {
      dataSourceModule.setActivePortalDataSource(new PrototypeBackedPortalSource());
    },
    {
      name: 'TypeError',
    },
  );

  assert.equal(
    dataSourceModule.getActivePortalDataSource(),
    originalDataSource,
  );
  assert.equal(
    dataSourceModule.activePortalDataSource.getPortalDashboard,
    originalDataSource.getPortalDashboard,
  );
  assert.notEqual(
    dataSourceModule.activePortalDataSource.getPortalDashboard,
    PrototypeBackedPortalSource.prototype.getPortalDashboard,
  );
});

test('portal-api runtime rejects unknown override keys so SDK seam typos fail fast', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  assert.throws(
    () => {
      dataSourceModule.setActivePortalDataSource({
        getPortalDashbord() {
          return { typo: true };
        },
      });
    },
    {
      name: 'TypeError',
    },
  );

  assert.equal(
    dataSourceModule.getActivePortalDataSource(),
    originalDataSource,
  );
  assert.equal(typeof dataSourceModule.activePortalDataSource.getPortalDashboard, 'function');
});

test('portal-api exposes an immutable active data source so consumers cannot mutate the runtime seam in place', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const activeDataSource = dataSourceModule.getActivePortalDataSource();

  assert.throws(
    () => {
      activeDataSource.getPortalDashboard = null;
    },
    {
      name: 'TypeError',
    },
  );

  assert.equal(typeof dataSourceModule.activePortalDataSource.getPortalDashboard, 'function');
});

test('portal-api exposes a stable enumerable seam surface for future SDK-backed consumers', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const stableSeamKeys = Object.keys(dataSourceModule.activePortalDataSource).sort();
  const activeDataSourceKeys = Object.keys(dataSourceModule.getActivePortalDataSource()).sort();

  assert.deepEqual(stableSeamKeys, activeDataSourceKeys);

  const descriptor = Object.getOwnPropertyDescriptor(
    dataSourceModule.activePortalDataSource,
    'getPortalDashboard',
  );

  assert.equal(typeof descriptor?.get, 'function');
  assert.equal(descriptor?.enumerable, true);
});

test('portal-api defaults to the HTTP-backed runtime seam even without browser window globals', async () => {
  const createDataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/createPortalDataSource.js',
      ),
    ).href
  );
  const mockDataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/dataSources/mockPortalDataSource.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  const originalDefaultPortalDataSource = globalThis.__CRAW_CHAT_PORTAL_DEFAULT_DATA_SOURCE__;
  global.window = undefined;
  delete globalThis.__CRAW_CHAT_PORTAL_DEFAULT_DATA_SOURCE__;

  try {
    const dataSource = createDataSourceModule.createPortalDataSource();

    assert.equal(typeof dataSource.getPortalHome, 'function');
    assert.equal(typeof dataSource.loginPortalUser, 'function');
    assert.notEqual(dataSource.getPortalHome, mockDataSourceModule.mockPortalDataSource.getPortalHome);
    assert.notEqual(
      dataSource.loginPortalUser,
      mockDataSourceModule.mockPortalDataSource.loginPortalUser,
    );
  } finally {
    global.window = originalWindow;
    if (originalDefaultPortalDataSource === undefined) {
      delete globalThis.__CRAW_CHAT_PORTAL_DEFAULT_DATA_SOURCE__;
    } else {
      globalThis.__CRAW_CHAT_PORTAL_DEFAULT_DATA_SOURCE__ = originalDefaultPortalDataSource;
    }
  }
});

test('portal README describes an HTTP-backed default runtime and keeps mock usage explicit-only', async () => {
  const readme = await readFile(path.join(appRoot, 'README.md'), 'utf8');

  assert.doesNotMatch(readme, /mock-backed active data source/i);
  assert.match(readme, /HTTP-backed default data source/i);
  assert.match(readme, /mockPortalDataSource/i);
  assert.match(readme, /explicit override|isolated tests/i);
});

test('portal-api runtime implementation does not handcraft fetch calls or Authorization headers', async () => {
  const runtimeSource = await readFile(
    path.join(
      appRoot,
      'packages/craw-chat-portal-portal-api/src/runtime/dataSources/httpPortalDataSource.js',
    ),
    'utf8',
  );

  assert.doesNotMatch(runtimeSource, /\bfetch\s*\(/);
  assert.doesNotMatch(runtimeSource, /\bAuthorization\b/);
  assert.doesNotMatch(runtimeSource, /\bAccess-Token\b/);
});

test('feature packages do not bypass the portal-api boundary with raw HTTP calls', async () => {
  const packageFiles = [
    'packages/craw-chat-portal-dashboard/src/index.js',
    'packages/craw-chat-portal-conversations/src/index.js',
    'packages/craw-chat-portal-realtime/src/index.js',
    'packages/craw-chat-portal-media/src/index.js',
    'packages/craw-chat-portal-automation/src/index.js',
    'packages/craw-chat-portal-governance/src/index.js',
  ];

  for (const relativePath of packageFiles) {
    const contents = await readFile(path.join(appRoot, relativePath), 'utf8');
    assert.doesNotMatch(contents, /\bfetch\s*\(/);
    assert.doesNotMatch(contents, /XMLHttpRequest/);
  }
});

test('portal-api storage helpers fail closed when session storage is unavailable', async () => {
  const apiModule = await import(
    pathToFileURL(path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/index.js')).href
  );

  const originalWindow = global.window;
  global.window = {
    sessionStorage: throwingStorageDouble(),
    localStorage: throwingStorageDouble(),
  };

  try {
    assert.equal(apiModule.readPortalSessionToken(), null);
    assert.doesNotThrow(() => apiModule.persistPortalSessionToken('tenant-demo-session'));
    assert.doesNotThrow(() => apiModule.clearPortalSessionToken());

    const session = await apiModule.bootstrapPortalSession();
    assert.equal(session, null);
  } finally {
    global.window = originalWindow;
  }
});

test('portal-api rejects malformed persisted session tokens before touching browser storage', async () => {
  const apiModule = await import(
    pathToFileURL(path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/index.js')).href
  );

  const originalWindow = global.window;
  const localStorage = storageDouble();
  const sessionStorage = storageDouble();
  global.window = {
    sessionStorage,
    localStorage,
  };

  try {
    assert.throws(
      () => apiModule.persistPortalSessionToken(''),
      {
        name: 'TypeError',
      },
    );
    assert.throws(
      () => apiModule.persistPortalSessionToken(null),
      {
        name: 'TypeError',
      },
    );

    assert.equal(sessionStorage.getItem('craw-chat-portal.session.v1'), null);
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    global.window = originalWindow;
  }
});

test('portal-api clears malformed persisted session tokens when reading browser session storage', async () => {
  const apiModule = await import(
    pathToFileURL(path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/index.js')).href
  );

  const originalWindow = global.window;
  const localStorage = storageDouble();
  const sessionStorage = storageDouble();
  sessionStorage.setItem('craw-chat-portal.session.v1', '   ');

  global.window = {
    sessionStorage,
    localStorage,
  };

  try {
    assert.equal(apiModule.readPortalSessionToken(), null);
    assert.equal(sessionStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    global.window = originalWindow;
  }
});

test('portal-api migrates legacy local storage session tokens into session storage on read', async () => {
  const apiModule = await import(
    pathToFileURL(path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/index.js')).href
  );

  const originalWindow = global.window;
  const localStorage = storageDouble();
  const sessionStorage = storageDouble();
  localStorage.setItem('craw-chat-portal.session.v1', 'legacy-session-token');

  global.window = {
    sessionStorage,
    localStorage,
  };

  try {
    assert.equal(apiModule.readPortalSessionToken(), 'legacy-session-token');
    assert.equal(sessionStorage.getItem('craw-chat-portal.session.v1'), 'legacy-session-token');
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    global.window = originalWindow;
  }
});

test('portal-api persists new session tokens in session storage and clears legacy local storage', async () => {
  const apiModule = await import(
    pathToFileURL(path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/index.js')).href
  );

  const originalWindow = global.window;
  const localStorage = storageDouble();
  const sessionStorage = storageDouble();
  localStorage.setItem('craw-chat-portal.session.v1', 'legacy-session-token');

  global.window = {
    sessionStorage,
    localStorage,
  };

  try {
    apiModule.persistPortalSessionToken('fresh-session-token');

    assert.equal(sessionStorage.getItem('craw-chat-portal.session.v1'), 'fresh-session-token');
    assert.equal(localStorage.getItem('craw-chat-portal.session.v1'), null);
  } finally {
    global.window = originalWindow;
  }
});

test('portal sdk browser runtime defaults backend calls to the current origin', async () => {
  const sdkClientModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/sdk/createPortalSdkClient.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    location: {
      origin: 'http://127.0.0.1:4176',
      hostname: '127.0.0.1',
    },
  };

  try {
    assert.deepEqual(
      sdkClientModule.resolvePortalBackendConfig(),
      { baseUrl: 'http://127.0.0.1:4176' },
    );
  } finally {
    global.window = originalWindow;
  }
});

test('portal sdk browser runtime honors an explicit API base URL override before same-origin fallback', async () => {
  const sdkClientModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/sdk/createPortalSdkClient.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  global.window = {
    __CRAW_CHAT_PORTAL_API_BASE_URL__: ' https://api.example.com/tenant-edge/ ',
    location: {
      origin: 'https://portal.example.com',
      hostname: 'portal.example.com',
    },
  };

  try {
    assert.deepEqual(
      sdkClientModule.resolvePortalBackendConfig(),
      { baseUrl: 'https://api.example.com/tenant-edge' },
    );
  } finally {
    global.window = originalWindow;
  }
});

test('portal sdk node runtime retains the explicit localhost fallback for non-browser execution', async () => {
  const sdkClientModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/sdk/createPortalSdkClient.js',
      ),
    ).href
  );

  const originalWindow = global.window;
  global.window = undefined;

  try {
    assert.deepEqual(
      sdkClientModule.resolvePortalBackendConfig(),
      { baseUrl: 'http://127.0.0.1:18124' },
    );
  } finally {
    global.window = originalWindow;
  }
});
