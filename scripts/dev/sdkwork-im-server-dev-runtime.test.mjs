import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  createSdkworkImServerCargoEnv,
  resolveSdkworkImServerBindEnv,
} from './sdkwork-im-server-dev-runtime.mjs';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

const defaultCargoEnv = createSdkworkImServerCargoEnv({
  env: {},
  repoRoot,
});
assert.equal(
  defaultCargoEnv.env.CARGO_TARGET_DIR,
  path.join(repoRoot, '.runtime', 'cargo-target', 'sdkwork-im-server-dev'),
  'server:dev must build into an isolated target dir so locked target/debug/sdkwork-im-server.exe cannot block rebuilds',
);
assert.equal(
  defaultCargoEnv.usingDefaultTargetDir,
  true,
  'default server:dev cargo target dir should be reported as an automatic dev fallback',
);

const explicitCargoEnv = createSdkworkImServerCargoEnv({
  env: {
    CARGO_TARGET_DIR: path.join(repoRoot, 'custom-target'),
  },
  repoRoot,
});
assert.equal(
  explicitCargoEnv.env.CARGO_TARGET_DIR,
  path.join(repoRoot, 'custom-target'),
  'server:dev must respect an explicitly configured CARGO_TARGET_DIR',
);
assert.equal(
  explicitCargoEnv.usingDefaultTargetDir,
  false,
  'explicit CARGO_TARGET_DIR must not be reported as the automatic dev fallback',
);

const fallbackBindEnv = await resolveSdkworkImServerBindEnv({
  env: {},
  isPortAvailable: async (port) => port === 18081,
  maxAttempts: 3,
});
assert.equal(
  fallbackBindEnv.bindAddr,
  '127.0.0.1:18081',
  'server:dev must choose the next available local gateway bind when 18079 is already occupied',
);
assert.equal(
  fallbackBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND,
  '127.0.0.1:18081',
  'server:dev must pass the selected bind to the Rust gateway',
);
assert.equal(
  fallbackBindEnv.env.SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  'http://127.0.0.1:18081',
  'server:dev must expose the selected gateway URL to browser SDK env resolution',
);
assert.equal(
  fallbackBindEnv.portChanged,
  true,
  'server:dev must report when it had to move off the default gateway port',
);

const reservedDrivePortBindEnv = await resolveSdkworkImServerBindEnv({
  env: {},
  isPortAvailable: async (port) => port >= 18080,
  maxAttempts: 3,
});
assert.equal(
  reservedDrivePortBindEnv.bindAddr,
  '127.0.0.1:18081',
  'server:dev must skip 18080 because the default Drive app-api dependency binds there',
);
assert.equal(
  reservedDrivePortBindEnv.portChanged,
  true,
  'server:dev must report the reserved Drive port skip as an automatic port fallback',
);

const explicitBindEnv = await resolveSdkworkImServerBindEnv({
  env: {
    SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND: '127.0.0.1:28079',
  },
  isPortAvailable: async () => false,
});
assert.equal(
  explicitBindEnv.bindAddr,
  '127.0.0.1:28079',
  'server:dev must respect an explicit SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND instead of auto-rotating it',
);
assert.equal(
  explicitBindEnv.portChanged,
  false,
  'explicit server binds must not be reported as automatic port fallback',
);

const startScript = fs.readFileSync(
  path.join(repoRoot, 'scripts/im-server-dev.mjs'),
  'utf8',
);
assert.match(
  startScript,
  /createSdkworkImServerCargoEnv/u,
  'server:dev startup must use the shared cargo target isolation helper',
);
assert.match(
  startScript,
  /resolveSdkworkImServerBindEnv/u,
  'server:dev startup must use the shared gateway bind resolver',
);
