import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

test('auth package exists and exposes the IM operator login page', () => {
  assert.equal(
    existsSync(path.join(appRoot, 'packages', 'sdkwork-craw-chat-admin-auth', 'src', 'index.tsx')),
    true,
  );

  const auth = read('packages/sdkwork-craw-chat-admin-auth/src/index.tsx');

  assert.match(auth, /Craw Chat Admin|IM operator workspace/);
  assert.match(auth, /Welcome back|Sign in/);
  assert.match(auth, /QR login/);
  assert.match(auth, /Forgot password/);
  assert.match(auth, /request access|Request access/i);
  assert.match(auth, /Shift handoff|Trust and safety/);
  assert.match(auth, /Service continuity|Step-up enforced/);
  assert.doesNotMatch(auth, /CrawChat Admin|router control plane|gateway/);
});

test('shell routes keep auth outside the authenticated shell', () => {
  const routes = read('packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx');

  assert.match(routes, /AdminLoginPage/);
  assert.match(routes, /MainLayout/);
  assert.match(routes, /ROUTE_PATHS\.AUTH/);
  assert.match(routes, /ROUTE_PATHS\.LOGIN/);
  assert.match(routes, /ROUTE_PATHS\.REGISTER/);
  assert.match(routes, /ROUTE_PATHS\.FORGOT_PASSWORD/);
});
