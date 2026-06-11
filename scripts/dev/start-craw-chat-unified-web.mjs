import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveCrawChatSharedDatabaseConfig } from './craw-chat-shared-database.mjs';
import {
  createSdkworkChatBrowserOrigins,
  resolveDriveAppApiUpstream,
  resolveNotaryAppApiUpstream,
  resolveSdkworkChatPcDevServer,
} from './run-sdkwork-chat-pc-dev.mjs';
import {
  createCrawChatServerCargoEnv,
  resolveCrawChatServerBindEnv,
} from './craw-chat-server-dev-runtime.mjs';

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
  const cargoEnv = createCrawChatServerCargoEnv({
    env: baseEnv,
    repoRoot,
  });
  const bindEnv = await resolveCrawChatServerBindEnv({
    env: cargoEnv.env,
  });
  const env = {
    ...bindEnv.env,
    ...resolveCrawChatSharedDatabaseConfig({ env: bindEnv.env, repoRoot }).env,
  };
  env.CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE = env.CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE ?? 'embedded';
  env.CRAW_CHAT_BROWSER_ORIGINS = env.CRAW_CHAT_BROWSER_ORIGINS ?? defaultBrowserOrigins;
  env.CRAW_CHAT_DRIVE_APP_API_UPSTREAM = resolveDriveAppApiUpstream(env);
  env.CRAW_CHAT_NOTARY_APP_API_UPSTREAM = resolveNotaryAppApiUpstream(env);

  if (
    !env.SDKWORK_ADMIN_PROXY_TARGET?.trim()
    && !env.SDKWORK_ADMIN_BIND?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX?.trim()
    && !env.SDKWORK_ADMIN_SANDBOX_MODE?.trim()
  ) {
    env.SDKWORK_ADMIN_SANDBOX = 'true';
    process.stdout.write('[craw-chat-server] SDKWORK_ADMIN_SANDBOX=true\n');
  }
  if (cargoEnv.usingDefaultTargetDir) {
    process.stdout.write(
      `[craw-chat-server] CARGO_TARGET_DIR=${path.relative(repoRoot, env.CARGO_TARGET_DIR)}\n`,
    );
  }
  if (bindEnv.portChanged) {
    process.stdout.write(
      `[craw-chat-server] 127.0.0.1:18079 is busy; using http://${bindEnv.bindAddr}\n`,
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

function terminateStaleCrawChatServerProcesses(env) {
  if (process.platform !== 'win32') {
    return;
  }

  const serverBinaryPath = path.join(
    env.CARGO_TARGET_DIR ?? path.join(repoRoot, 'target'),
    'debug',
    process.platform === 'win32' ? 'craw-chat-server.exe' : 'craw-chat-server',
  );
  const normalizedServerBinaryPath = path.resolve(serverBinaryPath).toLowerCase();
  const escapedServerBinaryPath = escapePowerShellSingleQuotedString(normalizedServerBinaryPath);
  const command = [
    '$target = ' + `'${escapedServerBinaryPath}'`,
    '$all = Get-CimInstance Win32_Process',
    '$all | Where-Object { $_.Name -eq "craw-chat-server.exe" -and $_.ExecutablePath -and $_.ExecutablePath.ToLowerInvariant() -eq $target }',
    '| ForEach-Object {',
    '  $server = $_;',
    '  $parent = $all | Where-Object { $_.ProcessId -eq $server.ParentProcessId } | Select-Object -First 1;',
    '  if ($parent -and $parent.Name -eq "cargo.exe" -and $parent.CommandLine -match "run\\s+-p\\s+web-gateway\\s+--bin\\s+craw-chat-server") {',
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
      `[craw-chat-server] failed to inspect stale craw-chat-server.exe processes: ${result.error.message}\n`,
    );
    return;
  }

  const pids = String(result.stdout ?? '')
    .split(/\r?\n/u)
    .map((line) => Number.parseInt(line.trim(), 10))
    .filter((pid) => Number.isInteger(pid) && pid > 0 && pid !== process.pid);

  for (const pid of new Set(pids)) {
    process.stdout.write(
      `[craw-chat-server] terminating stale ${path.relative(repoRoot, serverBinaryPath)} process ${pid}\n`,
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
    `[craw-chat-server] ${label} source not found at ${path.relative(repoRoot, sourceDir)}; using ${path.relative(repoRoot, fallbackDir)}\n`,
  );
  env[`CRAW_CHAT_${label.toUpperCase()}_SITE_DIR`] = fallbackDir;
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
  build: () => runCommand('pnpm', ['--dir', 'apps/craw-chat-admin', 'build'], { env: buildEnv }),
  distDir: path.join(repoRoot, 'apps', 'craw-chat-admin', 'dist'),
  fallbackDir: path.join(runtimeSiteRoot, 'admin'),
  label: 'admin',
  sourceDir: path.join(repoRoot, 'apps', 'craw-chat-admin'),
  title: 'Craw Chat Admin Dev Placeholder',
  env: runtimeEnv,
});
const portalSiteDir = await ensureDevSiteDist({
  build: () => runCommand(process.execPath, ['apps/craw-chat-portal/scripts/build.mjs'], { env: buildEnv }),
  distDir: path.join(repoRoot, 'apps', 'craw-chat-portal', 'dist'),
  fallbackDir: path.join(runtimeSiteRoot, 'portal'),
  label: 'portal',
  sourceDir: path.join(repoRoot, 'apps', 'craw-chat-portal'),
  title: 'Craw Chat Portal Dev Placeholder',
  env: runtimeEnv,
});
runtimeEnv.CRAW_CHAT_ADMIN_SITE_DIR = adminSiteDir;
runtimeEnv.CRAW_CHAT_PORTAL_SITE_DIR = portalSiteDir;

terminateStaleCrawChatServerProcesses(runtimeEnv);
await runCommand('cargo', ['run', '-p', 'web-gateway', '--bin', 'craw-chat-server'], {
  env: runtimeEnv,
});
