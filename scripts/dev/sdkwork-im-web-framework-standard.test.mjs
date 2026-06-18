#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const rootCargo = read('Cargo.toml');
for (const dependencyKey of [
  'sdkwork_web_core',
  'sdkwork_web_axum',
  'sdkwork_web_bootstrap',
  'sdkwork_iam_web_adapter',
]) {
  assert.match(
    rootCargo,
    new RegExp(`${dependencyKey}\\s*=\\s*\\{[^}]*sdkwork-web-framework|sdkwork-appbase`, 'u'),
    `Cargo.toml must declare workspace dependency ${dependencyKey} for sdkwork-web-framework integration`,
  );
}

const gatewayCargo = read('services/sdkwork-im-gateway/Cargo.toml');
assert.match(gatewayCargo, /sdkwork_web_axum\.workspace\s*=\s*true/u);
assert.match(gatewayCargo, /sdkwork_web_bootstrap\.workspace\s*=\s*true/u);
assert.match(gatewayCargo, /sdkwork_iam_web_adapter\.workspace\s*=\s*true/u);

const gatewayLib = read('services/sdkwork-im-gateway/src/lib.rs');
const gatewayWebFramework = read('services/sdkwork-im-gateway/src/web_framework.rs');
assert.match(gatewayLib, /mod web_framework;/u);
assert.match(gatewayLib, /web_framework::wrap_gateway_router/u);
assert.match(gatewayWebFramework, /WebFramework::builder/u);
assert.match(gatewayWebFramework, /with_web_request_context/u);
assert.match(gatewayWebFramework, /service_router/u);
assert.match(gatewayWebFramework, /IamDatabaseWebRequestContextResolver/u);
assert.match(gatewayWebFramework, /IM_APP_API_PREFIX/u);
assert.match(gatewayWebFramework, /IM_REALTIME_WEBSOCKET_PATH/u);
assert.match(gatewayWebFramework, /\/ws\//u);

const specsReadme = read('specs/README.md');
assert.match(specsReadme, /WEB_FRAMEWORK_SPEC\.md/u);
assert.match(specsReadme, /WEB_BACKEND_SPEC\.md/u);

process.stdout.write('sdkwork-im web framework standard contract passed\n');
