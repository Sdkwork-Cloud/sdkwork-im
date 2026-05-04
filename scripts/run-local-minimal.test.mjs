import assert from 'node:assert/strict';
import path from 'node:path';
import fs from 'node:fs';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'run-local-minimal.mjs'),
    ).href,
  );
}

test('craw-chat local-minimal runner exposes the canonical env-file and CLI parsing surface', async () => {
  const module = await loadModule();

  assert.equal(typeof module.parseArgs, 'function');
  assert.equal(typeof module.parseDotEnvContent, 'function');
  assert.equal(typeof module.resolveEnvFilePath, 'function');
  assert.equal(typeof module.assertRunLocalMinimalEnvironment, 'function');
  assert.equal(typeof module.createRunLocalMinimalEnvironment, 'function');
  assert.equal(typeof module.createRunLocalMinimalCommandPlan, 'function');
  assert.equal(typeof module.runLocalMinimal, 'function');

  assert.deepEqual(
    module.parseArgs([
      '--env-file',
      './config/local.env',
      '--bind-addr',
      '0.0.0.0:28080',
      '--runtime-dir',
      './.runtime/private-a',
      '--public-bearer-secret',
      'cli-public-secret',
      '--user-center-mode',
      'sdkwork-cloud-app-api',
      '--user-center-app-api-base-url',
      'https://app-api.sdkwork.local/craw',
      '--user-center-provider-key',
      'craw-app-api',
      '--user-center-secret-id',
      'secret-501',
      '--user-center-shared-secret',
      'shared-501',
      '--set-env',
      'EXTRA_FLAG=1',
      '--no-build',
      '--dry-run',
    ]),
    {
      bindAddr: '0.0.0.0:28080',
      browserOrigins: undefined,
      dryRun: true,
      envFile: './config/local.env',
      extraEnv: {
        EXTRA_FLAG: '1',
      },
      help: false,
      noBuild: true,
      publicBearerSecret: 'cli-public-secret',
      runtimeDir: './.runtime/private-a',
      userCenterAccessTokenHeaderName: undefined,
      userCenterAllowAuthorizationFallbackToAccessToken: undefined,
      userCenterAppApiBaseUrl: 'https://app-api.sdkwork.local/craw',
      userCenterAppId: undefined,
      userCenterAuthorizationHeaderName: undefined,
      userCenterAuthorizationScheme: undefined,
      userCenterDatabaseUrl: undefined,
      userCenterExternalBaseUrl: undefined,
      userCenterHandshakeFreshnessWindowMs: undefined,
      userCenterLocalApiBasePath: undefined,
      userCenterMode: 'sdkwork-cloud-app-api',
      userCenterProviderKey: 'craw-app-api',
      userCenterRefreshTokenHeaderName: undefined,
      userCenterSchemaName: undefined,
      userCenterSecretId: 'secret-501',
      userCenterSessionHeaderName: undefined,
      userCenterSharedSecret: 'shared-501',
      userCenterSqlitePath: undefined,
      userCenterTablePrefix: undefined,
      userModuleExternalCatalogPath: undefined,
      userModuleExternalSystem: undefined,
      userModuleProvider: undefined,
    },
  );
});
test('craw-chat local-minimal runner parses dotenv content and gives CLI precedence over env-file and process env', async () => {
  const module = await loadModule();

  assert.deepEqual(
    module.parseDotEnvContent(`
      # comment
      export SDKWORK_USER_CENTER_MODE=sdkwork-cloud-app-api
      SDKWORK_USER_CENTER_APP_API_BASE_URL="https://app-api.sdkwork.local/craw"
      CRAW_CHAT_BIND_ADDR=127.0.0.1:19090
      CRAW_CHAT_USER_CENTER_PROVIDER_KEY='craw-env-provider'
      SDKWORK_USER_CENTER_SHARED_SECRET=env-file-secret
    `),
    {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:19090',
      CRAW_CHAT_USER_CENTER_PROVIDER_KEY: 'craw-env-provider',
      SDKWORK_USER_CENTER_APP_API_BASE_URL: 'https://app-api.sdkwork.local/craw',
      SDKWORK_USER_CENTER_MODE: 'sdkwork-cloud-app-api',
      SDKWORK_USER_CENTER_SHARED_SECRET: 'env-file-secret',
    },
  );

  const env = module.createRunLocalMinimalEnvironment({
    baseEnv: {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
      SDKWORK_USER_CENTER_MODE: 'external-user-center',
      SDKWORK_USER_CENTER_PROVIDER_KEY: 'process-provider',
    },
    envFileEnv: {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:19090',
      CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET: 'env-secret',
      SDKWORK_USER_CENTER_APP_API_BASE_URL: 'https://app-api.sdkwork.local/craw',
      SDKWORK_USER_CENTER_MODE: 'sdkwork-cloud-app-api',
      SDKWORK_USER_CENTER_PROVIDER_KEY: 'env-provider',
      SDKWORK_USER_CENTER_SECRET_ID: 'secret-env',
      SDKWORK_USER_CENTER_SHARED_SECRET: 'shared-env',
    },
    options: {
      bindAddr: '0.0.0.0:28080',
      publicBearerSecret: 'cli-secret',
      userCenterMode: 'builtin-local',
      userCenterProviderKey: 'cli-provider',
      userCenterTablePrefix: 'cc_private_',
    },
    repoRoot: 'D:/workspace/craw-chat',
  });

  assert.equal(env.CRAW_CHAT_BIND_ADDR, '0.0.0.0:28080');
  assert.equal(env.CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET, 'cli-secret');
  assert.equal(env.SDKWORK_USER_CENTER_MODE, 'builtin-local');
  assert.equal(env.CRAW_CHAT_USER_CENTER_MODE, 'builtin-local');
  assert.equal(env.SDKWORK_USER_CENTER_PROVIDER_KEY, 'cli-provider');
  assert.equal(env.CRAW_CHAT_USER_CENTER_PROVIDER_KEY, 'cli-provider');
  assert.equal(env.SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH, '/api/app/v1/user-center');
  assert.equal(env.CRAW_CHAT_USER_CENTER_LOCAL_API_BASE_PATH, '/api/app/v1/user-center');
  assert.equal(env.SDKWORK_USER_CENTER_TABLE_PREFIX, 'cc_private_');
  assert.equal(env.CRAW_CHAT_USER_CENTER_TABLE_PREFIX, 'cc_private_');
  assert.equal(env.SDKWORK_USER_CENTER_SQLITE_PATH, './.runtime/local-minimal/data/user-center.db');
  assert.equal(env.CRAW_CHAT_USER_MODULE_PROVIDER, 'local');
});

test('craw-chat local-minimal runner fails closed when remote user-center modes or external user-module modes are under-configured', async () => {
  const module = await loadModule();

  assert.throws(
    () =>
      module.assertRunLocalMinimalEnvironment({
        SDKWORK_USER_CENTER_APP_API_BASE_URL: 'https://app-api.sdkwork.local/craw',
        SDKWORK_USER_CENTER_MODE: 'sdkwork-cloud-app-api',
        SDKWORK_USER_CENTER_PROVIDER_KEY: 'craw-app-api',
        SDKWORK_USER_CENTER_SECRET_ID: 'secret-501',
      }),
    /SDKWORK_USER_CENTER_SHARED_SECRET/u,
  );

  assert.throws(
    () =>
      module.assertRunLocalMinimalEnvironment({
        CRAW_CHAT_USER_MODULE_PROVIDER: 'external',
      }),
    /CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH/u,
  );
});

test('craw-chat local-minimal runner rejects legacy user-center mode aliases and keeps only canonical remote identifiers', async () => {
  const module = await loadModule();
  const rustUserCenterSource = fs.readFileSync(
    path.join(repoRoot, 'services', 'local-minimal-node', 'src', 'node', 'user_center.rs'),
    'utf8',
  );

  assert.throws(
    () =>
      module.createRunLocalMinimalEnvironment({
        baseEnv: {},
        envFileEnv: {},
        options: {
          userCenterMode: 'local',
        },
        repoRoot: 'D:/workspace/craw-chat',
      }),
    /builtin-local|sdkwork-cloud-app-api|external-user-center/u,
  );

  assert.throws(
    () =>
      module.createRunLocalMinimalEnvironment({
        baseEnv: {},
        envFileEnv: {},
        options: {
          userCenterMode: 'local-native',
        },
        repoRoot: 'D:/workspace/craw-chat',
      }),
    /builtin-local|sdkwork-cloud-app-api|external-user-center/u,
  );

  assert.throws(
    () =>
      module.createRunLocalMinimalEnvironment({
        baseEnv: {},
        envFileEnv: {},
        options: {
          userCenterMode: 'spring-ai-plus-app-api',
        },
        repoRoot: 'D:/workspace/craw-chat',
      }),
    /builtin-local|sdkwork-cloud-app-api|external-user-center/u,
  );

  assert.throws(
    () =>
      module.assertRunLocalMinimalEnvironment({
        SDKWORK_USER_CENTER_MODE: 'local',
      }),
    /builtin-local|sdkwork-cloud-app-api|external-user-center/u,
  );

  assert.throws(
    () =>
      module.assertRunLocalMinimalEnvironment({
        SDKWORK_USER_CENTER_MODE: 'local-native',
      }),
    /builtin-local|sdkwork-cloud-app-api|external-user-center/u,
  );

  assert.throws(
    () =>
      module.assertRunLocalMinimalEnvironment({
        SDKWORK_USER_CENTER_MODE: 'sdkwork-app-api',
      }),
    /builtin-local|sdkwork-cloud-app-api|external-user-center/u,
  );

  assert.doesNotMatch(
    rustUserCenterSource,
    /spring-ai-plus-app-api|sdkwork-app-api/u,
  );
});

test('craw-chat local-minimal runner materializes build and run plans and supports dry-run execution', async () => {
  const module = await loadModule();

  const plan = module.createRunLocalMinimalCommandPlan({
    cargoExecutable: 'cargo-custom',
    env: {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
    },
    noBuild: false,
    repoRoot: 'D:/workspace/craw-chat',
  });

  assert.deepEqual(plan, [
    {
      args: ['build', '-p', 'local-minimal-node', '--offline'],
      command: 'cargo-custom',
      cwd: 'D:/workspace/craw-chat',
      env: {
        CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
      },
      label: 'build local-minimal-node',
    },
    {
      args: ['run', '-p', 'local-minimal-node', '--offline'],
      command: 'cargo-custom',
      cwd: 'D:/workspace/craw-chat',
      env: {
        CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
      },
      label: 'run local-minimal-node',
    },
  ]);

  const output = [];
  const exitCode = module.runLocalMinimal({
    argv: [
      '--dry-run',
      '--no-build',
      '--bind-addr',
      '127.0.0.1:28080',
    ],
    baseEnv: {},
    cargoExecutable: 'cargo-custom',
    existsSyncImpl(filePath) {
      return String(filePath).endsWith('.env') ? false : false;
    },
    repoRoot: 'D:/workspace/craw-chat',
    runCommandSequenceImpl() {
      throw new Error('dry-run must not execute command sequence');
    },
    stdout: {
      write(value) {
        output.push(String(value));
      },
    },
  });

  assert.equal(exitCode, 0);
  assert.match(output.join(''), /run local-minimal-node/u);
  assert.match(output.join(''), /127\.0\.0\.1:28080/u);
});
