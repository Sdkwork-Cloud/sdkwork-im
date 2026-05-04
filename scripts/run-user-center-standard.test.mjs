import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'run-user-center-standard.mjs'),
    ).href,
  );
}

test('craw-chat user-center standard runner exposes the canonical governed node plan', async () => {
  const module = await loadModule();

  assert.equal(typeof module.resolveUserCenterStandardTestFile, 'function');
  assert.equal(typeof module.resolveLocalMinimalContractTestFile, 'function');
  assert.equal(typeof module.resolveServerUserCenterEntrypointContractTestFile, 'function');
  assert.equal(typeof module.createUserCenterStandardTestPlan, 'function');
  assert.equal(typeof module.resolveSdkworkAppbaseContractsRunner, 'function');
  assert.equal(typeof module.createUserCenterStandardCommandPlan, 'function');

  const testFile = module.resolveUserCenterStandardTestFile({
    repoRoot: 'D:/workspace/craw-chat',
  });
  assert.equal(
    testFile,
    path.join('D:/workspace/craw-chat', 'apps', 'craw-chat-portal', 'tests', 'portal-user-center-standard.test.mjs'),
  );
  const localMinimalTestFile = module.resolveLocalMinimalContractTestFile({
    repoRoot: 'D:/workspace/craw-chat',
  });
  assert.equal(
    localMinimalTestFile,
    path.join('D:/workspace/craw-chat', 'scripts', 'run-local-minimal.test.mjs'),
  );
  const serverEntrypointContractTestFile = module.resolveServerUserCenterEntrypointContractTestFile({
    repoRoot: 'D:/workspace/craw-chat',
  });
  assert.equal(
    serverEntrypointContractTestFile,
    path.join(
      'D:/workspace/craw-chat',
      'scripts',
      'server-user-center-entrypoint-contract.test.mjs',
    ),
  );

  const appbaseRunner = module.resolveSdkworkAppbaseContractsRunner({
    repoRoot: 'D:/workspace/craw-chat',
  });
  assert.equal(
    appbaseRunner,
    path.join(
      'D:/workspace/craw-chat',
      '..',
      'sdkwork-appbase',
      'scripts',
      'run-user-center-standard-contracts.mjs',
    ),
  );

  const plan = module.createUserCenterStandardTestPlan({
    repoRoot: 'D:/workspace/craw-chat',
    cwd: 'D:/workspace/craw-chat',
    env: { SDKWORK_RELEASE_MODE: '1' },
    nodeExecutable: 'node-custom',
  });

  assert.equal(plan.command, 'node-custom');
  assert.deepEqual(plan.args, [testFile]);
  assert.equal(plan.cwd, 'D:/workspace/craw-chat');
  assert.deepEqual(plan.env, { SDKWORK_RELEASE_MODE: '1' });
  assert.equal(plan.shell, false);
  assert.equal(plan.windowsHide, process.platform === 'win32');

  const commandPlan = module.createUserCenterStandardCommandPlan({
    repoRoot: 'D:/workspace/craw-chat',
    cwd: 'D:/workspace/craw-chat',
    env: { SDKWORK_RELEASE_MODE: '1' },
    nodeExecutable: 'node-custom',
  });

  assert.deepEqual(commandPlan, [
    {
      label: 'sdkwork-appbase user-center standard contracts',
      command: 'node-custom',
      args: [appbaseRunner],
      cwd: 'D:/workspace/craw-chat',
      env: { SDKWORK_RELEASE_MODE: '1' },
      shell: false,
      windowsHide: process.platform === 'win32',
    },
    {
      label: 'craw-chat portal user-center standard',
      command: 'node-custom',
      args: [testFile],
      cwd: 'D:/workspace/craw-chat',
      env: { SDKWORK_RELEASE_MODE: '1' },
      shell: false,
      windowsHide: process.platform === 'win32',
    },
    {
      label: 'craw-chat server local-minimal user-center contract',
      command: 'node-custom',
      args: [localMinimalTestFile],
      cwd: 'D:/workspace/craw-chat',
      env: { SDKWORK_RELEASE_MODE: '1' },
      shell: false,
      windowsHide: process.platform === 'win32',
    },
    {
      label: 'craw-chat server deployment entrypoint user-center contract',
      command: 'node-custom',
      args: [serverEntrypointContractTestFile],
      cwd: 'D:/workspace/craw-chat',
      env: { SDKWORK_RELEASE_MODE: '1' },
      shell: false,
      windowsHide: process.platform === 'win32',
    },
  ]);
});

test('craw-chat user-center standard runner executes the shared appbase contracts before the portal contract through spawnSync', async () => {
  const module = await loadModule();

  const calls = [];
  const results = module.runUserCenterStandardTest({
    repoRoot: 'D:/workspace/craw-chat',
    cwd: 'D:/workspace/craw-chat',
    env: { SDKWORK_ENV: '1' },
    nodeExecutable: 'node-custom',
    spawnSyncImpl(command, args, options) {
      calls.push({ command, args, options });
      return {
        status: 0,
        stdout: '',
        stderr: '',
      };
    },
  });

  assert.equal(results.length, 4);
  assert.equal(results[0].status, 0);
  assert.equal(results[1].status, 0);
  assert.equal(results[2].status, 0);
  assert.equal(results[3].status, 0);
  assert.deepEqual(calls, [
    {
      command: 'node-custom',
      args: [
        path.join(
          'D:/workspace/craw-chat',
          '..',
          'sdkwork-appbase',
          'scripts',
          'run-user-center-standard-contracts.mjs',
        ),
      ],
      options: {
        cwd: 'D:/workspace/craw-chat',
        env: { SDKWORK_ENV: '1' },
        shell: false,
        stdio: 'inherit',
        windowsHide: process.platform === 'win32',
      },
    },
    {
      command: 'node-custom',
      args: [
        path.join('D:/workspace/craw-chat', 'apps', 'craw-chat-portal', 'tests', 'portal-user-center-standard.test.mjs'),
      ],
      options: {
        cwd: 'D:/workspace/craw-chat',
        env: { SDKWORK_ENV: '1' },
        shell: false,
        stdio: 'inherit',
        windowsHide: process.platform === 'win32',
      },
    },
    {
      command: 'node-custom',
      args: [
        path.join('D:/workspace/craw-chat', 'scripts', 'run-local-minimal.test.mjs'),
      ],
      options: {
        cwd: 'D:/workspace/craw-chat',
        env: { SDKWORK_ENV: '1' },
        shell: false,
        stdio: 'inherit',
        windowsHide: process.platform === 'win32',
      },
    },
    {
      command: 'node-custom',
      args: [
        path.join(
          'D:/workspace/craw-chat',
          'scripts',
          'server-user-center-entrypoint-contract.test.mjs',
        ),
      ],
      options: {
        cwd: 'D:/workspace/craw-chat',
        env: { SDKWORK_ENV: '1' },
        shell: false,
        stdio: 'inherit',
        windowsHide: process.platform === 'win32',
      },
    },
  ]);
});

test('craw-chat user-center standard runner allows overriding the sdkwork-appbase root for portable CI sync workflows', async () => {
  const module = await loadModule();

  const appbaseRunner = module.resolveSdkworkAppbaseContractsRunner({
    repoRoot: 'D:/workspace/craw-chat',
    sdkworkAppbaseRoot: 'D:/workspace/external/sdkwork-appbase',
  });
  assert.equal(
    appbaseRunner,
    path.join(
      'D:/workspace/external/sdkwork-appbase',
      'scripts',
      'run-user-center-standard-contracts.mjs',
    ),
  );

  const commandPlan = module.createUserCenterStandardCommandPlan({
    repoRoot: 'D:/workspace/craw-chat',
    sdkworkAppbaseRoot: 'D:/workspace/external/sdkwork-appbase',
    cwd: 'D:/workspace/craw-chat',
    env: { SDKWORK_APPBASE_ROOT: 'D:/workspace/external/sdkwork-appbase' },
    nodeExecutable: 'node-custom',
  });

  assert.deepEqual(commandPlan[0], {
    label: 'sdkwork-appbase user-center standard contracts',
    command: 'node-custom',
    args: [
      path.join(
        'D:/workspace/external/sdkwork-appbase',
        'scripts',
        'run-user-center-standard-contracts.mjs',
      ),
    ],
    cwd: 'D:/workspace/craw-chat',
    env: { SDKWORK_APPBASE_ROOT: 'D:/workspace/external/sdkwork-appbase' },
    shell: false,
    windowsHide: process.platform === 'win32',
  });
});
