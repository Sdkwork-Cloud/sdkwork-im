import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

test('auth package exists and exposes the control-plane login page', () => {
  assert.equal(
    existsSync(path.join(appRoot, 'packages', 'sdkwork-control-plane-auth', 'src', 'index.tsx')),
    true,
  );

  const auth = read('packages/sdkwork-control-plane-auth/src/index.tsx');

  assert.match(auth, /Craw Chat Admin|control-plane workspace/);
  assert.match(auth, /Welcome back|Sign in/);
  assert.match(auth, /Protected operator access|Password-first access/);
  assert.match(auth, /Identity provider availability|External identity providers remain disabled/);
  assert.doesNotMatch(auth, /QR login/);
  assert.doesNotMatch(auth, /Open the SDKWork app and scan this code/);
  assert.doesNotMatch(auth, /Open app to scan/);
  assert.doesNotMatch(auth, /Continue with/);
  assert.match(auth, /Forgot password/);
  assert.match(auth, /request access|Request access/i);
  assert.match(auth, /Request operator access/);
  assert.match(auth, /Need access\?/);
  assert.match(auth, /Already provisioned\?/);
  assert.match(auth, /Operations lead/);
  assert.match(auth, /ops@workspace\.example/);
  assert.match(auth, /Shift handoff|Trust and safety/);
  assert.match(auth, /Service continuity|Step-up enforced/);
  assert.doesNotMatch(auth, /Create account/);
  assert.doesNotMatch(auth, /No account\?/);
  assert.doesNotMatch(auth, /Already have an account\?/);
  assert.doesNotMatch(auth, /Workspace owner/);
  assert.doesNotMatch(auth, /name@example\.com/);
  assert.doesNotMatch(auth, /CrawChat Admin|router control plane|gateway/);
});

test('shell routes keep auth outside the authenticated shell', () => {
  const routes = read('packages/sdkwork-control-plane-shell/src/application/router/AppRoutes.tsx');

  assert.match(routes, /AdminLoginPage/);
  assert.match(routes, /MainLayout/);
  assert.match(routes, /ROUTE_PATHS\.AUTH/);
  assert.match(routes, /ROUTE_PATHS\.LOGIN/);
  assert.match(routes, /ROUTE_PATHS\.REGISTER/);
  assert.match(routes, /ROUTE_PATHS\.FORGOT_PASSWORD/);
});
