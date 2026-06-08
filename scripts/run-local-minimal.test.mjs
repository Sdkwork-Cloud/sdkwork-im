import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');
const fixtureRepoRoot = repoRoot;

async function loadModule() {
  return import(pathToFileURL(path.join(repoRoot, 'scripts', 'run-local-minimal.mjs')).href);
}

test('craw-chat local-minimal runner exposes only backend-owned runtime options', async () => {
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
      '--runtime_dir',
      './.runtime/private-a',
      '--user-module-provider',
      'external',
      '--user-module-external-catalog-path',
      './config/users.json',
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
      runtimeDir: './.runtime/private-a',
      userModuleExternalCatalogPath: './config/users.json',
      userModuleExternalSystem: undefined,
      userModuleProvider: 'external',
    },
  );

  assert.throws(
    () => module.parseArgs(['--user-center-mode', 'builtin-local']),
    /unknown argument: --user-center-mode/u,
  );
});

test('craw-chat local-minimal runner parses dotenv content and gives CLI precedence', async () => {
  const module = await loadModule();

  assert.deepEqual(
    module.parseDotEnvContent(`
      # comment
      CRAW_CHAT_BIND_ADDR=127.0.0.1:19090
      CRAW_CHAT_USER_MODULE_PROVIDER='local'
    `),
    {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:19090',
      CRAW_CHAT_USER_MODULE_PROVIDER: 'local',
    },
  );

  const env = module.createRunLocalMinimalEnvironment({
    baseEnv: {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
      CRAW_CHAT_USER_MODULE_PROVIDER: 'local',
    },
    envFileEnv: {
      CRAW_CHAT_BIND_ADDR: '127.0.0.1:19090',
    },
    options: {
      bindAddr: '0.0.0.0:28080',
    },
    repoRoot: fixtureRepoRoot,
  });

  assert.equal(env.CRAW_CHAT_BIND_ADDR, '0.0.0.0:28080');
  assert.equal(env.CRAW_CHAT_RUNTIME_DIR, './.runtime/local-minimal');
  assert.equal(env.CRAW_CHAT_USER_MODULE_PROVIDER, 'local');
  assert.equal(env.PWD, fixtureRepoRoot);
  assert.equal(
    Object.keys(env).some((key) => key.includes('USER_CENTER')),
    false,
    'local runner must not materialize user-center environment variables',
  );
});

test('craw-chat local-minimal runner validates external user-module configuration only', async () => {
  const module = await loadModule();

  assert.throws(
    () =>
      module.assertRunLocalMinimalEnvironment({
        CRAW_CHAT_USER_MODULE_PROVIDER: 'external',
      }),
    /CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH/u,
  );

  assert.doesNotThrow(() =>
    module.assertRunLocalMinimalEnvironment({
      CRAW_CHAT_USER_MODULE_PROVIDER: 'external',
      CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH: './users.json',
    }),
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
    repoRoot: fixtureRepoRoot,
  });

  assert.deepEqual(plan, [
    {
      args: ['build', '-p', 'local-minimal-node', '--offline'],
      command: 'cargo-custom',
      cwd: fixtureRepoRoot,
      env: {
        CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
      },
      label: 'build local-minimal-node',
    },
    {
      args: ['run', '-p', 'local-minimal-node', '--offline'],
      command: 'cargo-custom',
      cwd: fixtureRepoRoot,
      env: {
        CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
      },
      label: 'run local-minimal-node',
    },
  ]);

  const output = [];
  const exitCode = module.runLocalMinimal({
    argv: ['--dry-run', '--no-build', '--bind-addr', '127.0.0.1:28080'],
    baseEnv: {},
    cargoExecutable: 'cargo-custom',
    existsSyncImpl() {
      return false;
    },
    repoRoot: fixtureRepoRoot,
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
  assert.doesNotMatch(output.join(''), /USER_CENTER|user-center/u);
});

test('craw-chat local-minimal runner invokes the shared command sequence with root cwd and merged env', async () => {
  const module = await loadModule();

  let receivedSequenceArgument;
  const exitCode = module.runLocalMinimal({
    argv: [
      '--no-build',
      '--bind-addr',
      '127.0.0.1:28080',
      '--browser-origins',
      'http://127.0.0.1:4176,http://localhost:4176',
    ],
    baseEnv: {
      PATH: 'demo-path',
    },
    cargoExecutable: 'cargo-custom',
    existsSyncImpl() {
      return false;
    },
    repoRoot: fixtureRepoRoot,
    runCommandSequenceImpl(argument) {
      receivedSequenceArgument = argument;
      return 0;
    },
    stdout: {
      write() {},
    },
  });

  assert.equal(exitCode, 0);
  assert.ok(receivedSequenceArgument, 'shared runner must be invoked');
  assert.ok(
    Array.isArray(receivedSequenceArgument.commands),
    'shared runner expects a { commands, cwd, env } argument, not a bare command array',
  );
  assert.equal(receivedSequenceArgument.cwd, fixtureRepoRoot);
  assert.equal(receivedSequenceArgument.env.CRAW_CHAT_BIND_ADDR, '127.0.0.1:28080');
  assert.equal(
    receivedSequenceArgument.env.CRAW_CHAT_BROWSER_ORIGINS,
    'http://127.0.0.1:4176,http://localhost:4176',
  );
  assert.deepEqual(receivedSequenceArgument.commands, [
    {
      args: ['run', '-p', 'local-minimal-node', '--offline'],
      command: 'cargo-custom',
      cwd: fixtureRepoRoot,
      env: receivedSequenceArgument.env,
      label: 'run local-minimal-node',
    },
  ]);
});
