import assert from 'node:assert/strict';
import { access, readdir, readFile } from 'node:fs/promises';
import { createServer } from 'node:net';
import path from 'node:path';
import { test } from 'node:test';
import * as buildDistModule from '../scripts/lib/build-dist.mjs';
import { rebuildDist } from '../scripts/lib/build-dist.mjs';
import { isPathInsideRoot, startPreviewServer } from '../scripts/lib/preview-server.mjs';
import { resolvePortalAppRoot } from './helpers/portal-paths.mjs';

const appRoot = resolvePortalAppRoot(import.meta.url);
const distIndexHtml = path.join(appRoot, 'dist/index.html');
const distVendorEntries = [
  path.join(
    appRoot,
    'dist/__vendor__/sdkwork-craw-chat-sdk/index.js',
  ),
  path.join(
    appRoot,
    'dist/__vendor__/sdkwork-craw-chat-backend-sdk/index.js',
  ),
  path.join(
    appRoot,
    'dist/__vendor__/sdkwork-sdk-common/index.js',
  ),
];
const forbiddenDistEntries = [
  path.join(
    appRoot,
    'dist/packages/craw-chat-portal-portal-api/src/mockData.js',
  ),
  path.join(
    appRoot,
    'dist/packages/craw-chat-portal-portal-api/src/runtime/dataSources/mockPortalDataSource.js',
  ),
];
const forbiddenDistMarkers = [
  /portalMockData/,
  /mockPortalDataSource/,
  /tenant-demo-session/,
  /value="t_demo"/,
  /value="ops_demo"/,
  /Portal#2026/,
];

function reservePort() {
  return new Promise((resolve, reject) => {
    const server = createServer();

    server.on('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const address = server.address();
      const port = typeof address === 'object' && address ? address.port : null;

      server.close((error) => {
        if (error) {
          reject(error);
          return;
        }

        if (typeof port !== 'number') {
          reject(new Error('Unable to reserve a preview port.'));
          return;
        }

        resolve(port);
      });
    });
  });
}

function stopPreviewServer(server) {
  return new Promise((resolve, reject) => {
    server.close((error) => {
      if (error) {
        reject(error);
        return;
      }

      resolve();
    });
  });
}

function assertSecurityHeaders(response) {
  assert.match(
    response.headers.get('content-security-policy') ?? '',
    /default-src 'self'/,
  );
  assert.equal(response.headers.get('x-content-type-options'), 'nosniff');
  assert.equal(response.headers.get('x-frame-options'), 'DENY');
  assert.equal(
    response.headers.get('referrer-policy'),
    'strict-origin-when-cross-origin',
  );
  assert.match(
    response.headers.get('permissions-policy') ?? '',
    /camera=\(\)/,
  );
}

async function collectFiles(root) {
  const entries = await readdir(root, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const entryPath = path.join(root, entry.name);

    if (entry.isDirectory()) {
      files.push(...(await collectFiles(entryPath)));
      continue;
    }

    files.push(entryPath);
  }

  return files;
}

test('portal builds into a standalone distributable without external installs', async () => {
  await rebuildDist();
  await access(distIndexHtml);
  for (const vendorEntry of distVendorEntries) {
    await access(vendorEntry);
  }
});

test('portal build tolerates concurrent invocations targeting the same dist directory', async () => {
  await Promise.all([rebuildDist(), rebuildDist(), rebuildDist(), rebuildDist()]);
  await access(distIndexHtml);
});

test('portal dist rebuild excludes forbidden mock and demo release artifacts', async () => {
  await rebuildDist();

  for (const forbiddenEntry of forbiddenDistEntries) {
    await assert.rejects(access(forbiddenEntry), { code: 'ENOENT' });
  }

  const distFiles = await collectFiles(path.join(appRoot, 'dist'));

  for (const distFile of distFiles) {
    const fileContents = await readFile(distFile, 'utf8');

    for (const forbiddenMarker of forbiddenDistMarkers) {
      assert.doesNotMatch(
        fileContents,
        forbiddenMarker,
        `${distFile} unexpectedly contains ${forbiddenMarker}`,
      );
    }
  }
});

test('portal release safety collects forbidden content markers from generated artifacts', () => {
  assert.equal(typeof buildDistModule.collectPortalReleaseContentViolations, 'function');

  const violations = buildDistModule.collectPortalReleaseContentViolations({
    'dist/packages/bad.js': `
      export const session = 'tenant-demo-session';
      export const dataSource = mockPortalDataSource;
    `,
  });

  assert.deepEqual(
    violations.map(({ fileName, match }) => [fileName, match]).sort(),
    [
      ['dist/packages/bad.js', 'mockPortalDataSource'],
      ['dist/packages/bad.js', 'tenant-demo-session'],
    ],
  );
});

test('portal preview serves vendored SDK modules and returns 404 for missing vendor assets', async () => {
  const port = await reservePort();
  const { server: previewServer } = await startPreviewServer({ port });

  try {
    const vendorResponse = await fetch(
      `http://127.0.0.1:${port}/__vendor__/sdkwork-craw-chat-sdk/index.js`,
      { signal: AbortSignal.timeout(5000) },
    );
    const vendorBody = await vendorResponse.text();

    assert.equal(vendorResponse.status, 200);
    assert.match(vendorResponse.headers.get('content-type') ?? '', /text\/javascript/);
    assertSecurityHeaders(vendorResponse);
    assert.match(vendorBody, /sdk\.js/);
    assert.match(vendorBody, /errors\.js/);
    assert.match(vendorBody, /types\.js/);

    const missingVendorResponse = await fetch(
      `http://127.0.0.1:${port}/__vendor__/sdkwork-craw-chat-sdk/missing-entry.js`,
      { signal: AbortSignal.timeout(5000) },
    );
    const missingVendorBody = await missingVendorResponse.text();

    assert.equal(missingVendorResponse.status, 404);
    assertSecurityHeaders(missingVendorResponse);
    assert.doesNotMatch(missingVendorBody, /<!doctype html>/i);
  } finally {
    await stopPreviewServer(previewServer);
  }
});

test('portal preview path containment rejects sibling directories that only share a root prefix', () => {
  const safeRoot = path.join(appRoot, 'dist');
  const siblingCandidate = path.join(appRoot, 'dist-evil', 'index.html');
  const nestedCandidate = path.join(
    appRoot,
    'dist',
    '__vendor__',
    'sdkwork-craw-chat-sdk',
    'index.js',
  );

  assert.equal(isPathInsideRoot(safeRoot, siblingCandidate), false);
  assert.equal(isPathInsideRoot(safeRoot, nestedCandidate), true);
});
