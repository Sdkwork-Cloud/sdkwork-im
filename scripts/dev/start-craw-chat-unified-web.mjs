import { spawn } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');

function createBaseEnv() {
  return {
    ...process.env,
  };
}

function createRuntimeEnv(baseEnv) {
  const env = {
    ...baseEnv,
  };

  if (
    !env.SDKWORK_ADMIN_PROXY_TARGET?.trim()
    && !env.SDKWORK_ADMIN_BIND?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX_MODE?.trim()
  ) {
    env.SDKWORK_ADMIN_SANDBOX = 'true';
    process.stdout.write('[craw-chat-server] SDKWORK_ADMIN_SANDBOX=true\n');
  }

  return env;
}

function runCommand(command, args, { env } = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: repoRoot,
      env,
      stdio: 'inherit',
      shell: process.platform === 'win32',
    });

    child.on('error', reject);

    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }

      reject(new Error(`${command} ${args.join(' ')} exited with code ${code ?? 'unknown'}`));
    });
  });
}

const buildEnv = createBaseEnv();
const runtimeEnv = createRuntimeEnv(buildEnv);

await runCommand('pnpm', ['--dir', 'apps/craw-chat-admin', 'build'], { env: buildEnv });
await runCommand(process.execPath, ['apps/craw-chat-portal/scripts/build.mjs'], { env: buildEnv });
await runCommand('cargo', ['run', '-p', 'web-gateway', '--bin', 'craw-chat-server'], {
  env: runtimeEnv,
});
