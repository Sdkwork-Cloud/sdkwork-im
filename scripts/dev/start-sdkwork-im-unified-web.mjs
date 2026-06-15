import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveSdkworkImSharedDatabaseConfig } from './sdkwork-im-shared-database.mjs';
import {
  createManagedSdkworkApiGatewayProcess,
  createSdkworkChatBrowserOrigins,
  resolveSdkworkApiGatewayBaseUrl,
  resolveSdkworkApiGatewayBind,
  resolveSdkworkChatPcDevServer,
} from './run-sdkwork-im-pc-dev.mjs';
import {
  createSdkworkImServerCargoEnv,
  resolveSdkworkImServerBindEnv,
} from './sdkwork-im-server-dev-runtime.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');
const runtimeSiteRoot = path.join(repoRoot, '.runtime', 'dev-sites');
const defaultBrowserOrigins = createSdkworkChatBrowserOrigins(resolveSdkworkChatPcDevServer());
const activeChildren = new Set();
let shuttingDown = false;

function createBaseEnv() {
  return {
    ...process.env,
  };
}

async function createRuntimeEnv(baseEnv) {
  const cargoEnv = createSdkworkImServerCargoEnv({
    env: baseEnv,
    repoRoot,
  });
  const bindEnv = await resolveSdkworkImServerBindEnv({
    env: cargoEnv.env,
  });
  const env = {
    ...bindEnv.env,
    ...resolveSdkworkImSharedDatabaseConfig({ env: bindEnv.env, repoRoot }).env,
  };
  env.SDKWORK_IM_WEB_GATEWAY_RUNTIME_MODE = env.SDKWORK_IM_WEB_GATEWAY_RUNTIME_MODE ?? 'split';
  env.SDKWORK_IM_BROWSER_ORIGINS = env.SDKWORK_IM_BROWSER_ORIGINS ?? defaultBrowserOrigins;
  if (!env.SDKWORK_API_GATEWAY_BIND?.trim()) {
    env.SDKWORK_API_GATEWAY_BIND = resolveSdkworkApiGatewayBind(env);
  }
  if (!env.SDKWORK_IM_FOUNDATION_API_GATEWAY_BASE_URL?.trim()) {
    env.SDKWORK_IM_FOUNDATION_API_GATEWAY_BASE_URL = resolveSdkworkApiGatewayBaseUrl(env);
  }

  if (
    !env.SDKWORK_ADMIN_PROXY_TARGET?.trim()
    && !env.SDKWORK_ADMIN_BIND?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX_MODE?.trim()
  ) {
    env.SDKWORK_ADMIN_SANDBOX = 'true';
    process.stdout.write('[sdkwork-im-server] SDKWORK_ADMIN_SANDBOX=true\n');
  }
  if (cargoEnv.usingDefaultTargetDir) {
    process.stdout.write(
      `[sdkwork-im-server] CARGO_TARGET_DIR=${path.relative(repoRoot, env.CARGO_TARGET_DIR)}\n`,
    );
  }
  if (bindEnv.portChanged) {
    process.stdout.write(
      `[sdkwork-im-server] 127.0.0.1:18079 is busy; using http://${bindEnv.bindAddr}\n`,
    );
  }

  return env;
}

function writeDevSiteFallback(siteDir, title) {
  fs.mkdirSync(siteDir, { recursive: true });
  fs.writeFileSync(
    path.join(siteDir, 'index.html'),
    [
      '<!doctype html>',
      '<html lang="en">',
      '<head>',
      '  <meta charset="utf-8">',
      `  <title>${title}</title>`,
      '</head>',
      '<body>',
      `  <main>${title}</main>`,
      '</body>',
      '</html>',
      '',
    ].join('\n'),
  );
}

function terminateProcessTree(childOrPid) {
  const pid = typeof childOrPid === 'number' ? childOrPid : childOrPid?.pid;
  if (!pid) {
    return;
  }

  if (process.platform === 'win32') {
    spawnSync('taskkill.exe', ['/PID', String(pid), '/T', '/F'], {
      stdio: 'ignore',
      windowsHide: true,
    });
    return;
  }

  try {
    if (typeof childOrPid !== 'number' && typeof childOrPid.kill === 'function') {
      childOrPid.kill('SIGTERM');
      return;
    }
    process.kill(pid, 'SIGTERM');
  } catch {
    // Process already exited.
  }
}

function shutdownActiveChildren() {
  if (shuttingDown) {
    return;
  }
  shuttingDown = true;
  for (const child of activeChildren) {
    terminateProcessTree(child);
  }
}

function escapePowerShellSingleQuotedString(value) {
  return String(value).replaceAll("'", "''");
}

function terminateStaleSdkworkImServerProcesses(env) {
  if (process.platform !== 'win32') {
    return;
  }

  const serverBinaryPath = path.join(
    env.CARGO_TARGET_DIR ?? path.join(repoRoot, 'target'),
    'debug',
    process.platform === 'win32' ? 'sdkwork-im-server.exe' : 'sdkwork-im-server',
  );
  const normalizedServerBinaryPath = path.resolve(serverBinaryPath).toLowerCase();
  const escapedServerBinaryPath = escapePowerShellSingleQuotedString(normalizedServerBinaryPath);
  const command = [
    '$target = ' + `'${escapedServerBinaryPath}'`,
    '$all = Get-CimInstance Win32_Process',
    '$all | Where-Object { $_.Name -eq "sdkwork-im-server.exe" -and $_.ExecutablePath -and $_.ExecutablePath.ToLowerInvariant() -eq $target }',
    '| ForEach-Object {',
    '  $server = $_;',
    '  $parent = $all | Where-Object { $_.ProcessId -eq $server.ParentProcessId } | Select-Object -First 1;',
    '  if ($parent -and $parent.Name -eq "cargo.exe" -and $parent.CommandLine -match "run\\s+-p\\s+web-gateway\\s+--bin\\s+sdkwork-im-server") {',
    '    $parent.ProcessId',
    '  } else {',
    '    $server.ProcessId',
    '  }',
    '}',
  ].join(' ');
  const result = spawnSync(
    'powershell.exe',
    ['-NoProfile', '-NonInteractive', '-Command', command],
    {
      encoding: 'utf8',
      windowsHide: true,
    },
  );

  if (result.error) {
    process.stderr.write(
      `[sdkwork-im-server] failed to inspect stale sdkwork-im-server.exe processes: ${result.error.message}\n`,
    );
    return;
  }

  const pids = String(result.stdout ?? '')
    .split(/\r?\n/u)
    .map((line) => Number.parseInt(line.trim(), 10))
    .filter((pid) => Number.isInteger(pid) && pid > 0 && pid !== process.pid);

  for (const pid of new Set(pids)) {
    process.stdout.write(
      `[sdkwork-im-server] terminating stale ${path.relative(repoRoot, serverBinaryPath)} process ${pid}\n`,
    );
    terminateProcessTree(pid);
  }
}

function spawnCommand(command, args, { cwd = repoRoot, env, label, shell = process.platform === 'win32' } = {}) {
  const child = spawn(command, args, {
    cwd,
    env,
    stdio: 'inherit',
    shell,
  });
  activeChildren.add(child);

  child.on('error', (error) => {
    process.stderr.write(`[${label}] ${error instanceof Error ? error.message : String(error)}\n`);
  });
  child.on('exit', () => {
    activeChildren.delete(child);
  });

  return child;
}

function runCommand(command, args, { env } = {}) {
  return new Promise((resolve, reject) => {
    const child = spawnCommand(command, args, { env, label: command });

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

async function ensureDevSiteDist({
  build,
  distDir,
  fallbackDir,
  label,
  sourceDir,
  title,
  env,
}) {
  if (fs.existsSync(sourceDir)) {
    await build();
    return distDir;
  }

  writeDevSiteFallback(fallbackDir, title);
  process.stdout.write(
    `[sdkwork-im-server] ${label} source not found at ${path.relative(repoRoot, sourceDir)}; using ${path.relative(repoRoot, fallbackDir)}\n`,
  );
  env[`SDKWORK_IM_${label.toUpperCase()}_SITE_DIR`] = fallbackDir;
  return fallbackDir;
}

const buildEnv = createBaseEnv();
const runtimeEnv = await createRuntimeEnv(buildEnv);

process.once('SIGINT', () => {
  shutdownActiveChildren();
  process.exitCode = 130;
});
process.once('SIGTERM', () => {
  shutdownActiveChildren();
  process.exitCode = 143;
});

const adminSiteDir = await ensureDevSiteDist({
  build: () => runCommand('pnpm', ['--dir', 'apps/sdkwork-im-admin', 'build'], { env: buildEnv }),
  distDir: path.join(repoRoot, 'apps', 'sdkwork-im-admin', 'dist'),
  fallbackDir: path.join(runtimeSiteRoot, 'admin'),
  label: 'admin',
  sourceDir: path.join(repoRoot, 'apps', 'sdkwork-im-admin'),
  title: 'Sdkwork IM Admin Dev Placeholder',
  env: runtimeEnv,
});
const portalSiteDir = await ensureDevSiteDist({
  build: () => runCommand(process.execPath, ['apps/sdkwork-im-portal/scripts/build.mjs'], { env: buildEnv }),
  distDir: path.join(repoRoot, 'apps', 'sdkwork-im-portal', 'dist'),
  fallbackDir: path.join(runtimeSiteRoot, 'portal'),
  label: 'portal',
  sourceDir: path.join(repoRoot, 'apps', 'sdkwork-im-portal'),
  title: 'Sdkwork IM Portal Dev Placeholder',
  env: runtimeEnv,
});
runtimeEnv.SDKWORK_IM_ADMIN_SITE_DIR = adminSiteDir;
runtimeEnv.SDKWORK_IM_PORTAL_SITE_DIR = portalSiteDir;

const managedSdkworkApiGatewayProcess = createManagedSdkworkApiGatewayProcess({
  env: runtimeEnv,
  repoRoot,
});
if (managedSdkworkApiGatewayProcess) {
  spawnCommand(managedSdkworkApiGatewayProcess.command, managedSdkworkApiGatewayProcess.args, {
    cwd: managedSdkworkApiGatewayProcess.cwd,
    env: managedSdkworkApiGatewayProcess.env,
    label: managedSdkworkApiGatewayProcess.label,
    shell: managedSdkworkApiGatewayProcess.shell,
  });
}

terminateStaleSdkworkImServerProcesses(runtimeEnv);
await runCommand('cargo', ['run', '-p', 'web-gateway', '--bin', 'sdkwork-im-server'], {
  env: runtimeEnv,
});
