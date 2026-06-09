#!/usr/bin/env node

import { spawn, spawnSync } from 'node:child_process';
import fs from 'node:fs';
import net from 'node:net';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveSdkworkChatIamCommandEnv } from '../../apps/sdkwork-chat-pc/scripts/sdkwork-chat-iam-env.mjs';
import { resolveCrawChatSharedDatabaseConfig } from './craw-chat-shared-database.mjs';
import {
  createCrawChatServerCargoEnv,
  resolveCrawChatServerBindEnv,
} from './craw-chat-server-dev-runtime.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');
export const SDKWORK_CHAT_PC_DEV_HOST_ENV = 'SDKWORK_CHAT_PC_DEV_HOST';
export const SDKWORK_CHAT_PC_DEV_PORT_ENV = 'SDKWORK_CHAT_PC_DEV_PORT';
export const DEFAULT_SDKWORK_CHAT_PC_DEV_HOST = '127.0.0.1';
export const DEFAULT_SDKWORK_CHAT_PC_DEV_PORT = 4176;
const MAX_DEV_PORT_ATTEMPTS = 50;

const TARGETS = Object.freeze({
  browser: {
    label: 'sdkwork-chat-pc-browser',
    pnpmArgs: ['--dir', 'apps/sdkwork-chat-pc', 'dev'],
  },
  desktop: {
    label: 'sdkwork-chat-pc-desktop',
    pnpmArgs: ['--dir', 'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop', 'desktop:dev:local'],
  },
});

function pnpmCommand() {
  return process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

function pnpmShell() {
  return process.platform === 'win32';
}

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function normalizePort(value, label = 'port') {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }
  if (!/^\d+$/u.test(normalized)) {
    throw new Error(`${label} must be a TCP port number`);
  }
  const port = Number.parseInt(normalized, 10);
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`${label} must be between 1 and 65535`);
  }
  return port;
}

export function resolveSdkworkChatPcDevServer({
  env = process.env,
  host,
  port,
} = {}) {
  const resolvedHost = normalizeText(host)
    ?? normalizeText(env[SDKWORK_CHAT_PC_DEV_HOST_ENV])
    ?? DEFAULT_SDKWORK_CHAT_PC_DEV_HOST;
  const resolvedPort = normalizePort(
    port ?? env[SDKWORK_CHAT_PC_DEV_PORT_ENV] ?? DEFAULT_SDKWORK_CHAT_PC_DEV_PORT,
    SDKWORK_CHAT_PC_DEV_PORT_ENV,
  );
  return {
    host: resolvedHost,
    port: resolvedPort,
    url: `http://${resolvedHost}:${resolvedPort}`,
  };
}

export function createSdkworkChatBrowserOrigins({
  host = DEFAULT_SDKWORK_CHAT_PC_DEV_HOST,
  port = DEFAULT_SDKWORK_CHAT_PC_DEV_PORT,
} = {}) {
  const resolvedPort = normalizePort(port, SDKWORK_CHAT_PC_DEV_PORT_ENV);
  const originHosts = [host, 'localhost']
    .map((value) => normalizeText(value))
    .filter((value, index, values) => value && values.indexOf(value) === index);
  return originHosts
    .map((originHost) => `http://${originHost}:${resolvedPort}`)
    .join(',');
}

export function isTcpPortAvailable(port, host = DEFAULT_SDKWORK_CHAT_PC_DEV_HOST) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.unref();
    server.once('error', () => resolve(false));
    server.listen({ host, port }, () => {
      server.close(() => resolve(true));
    });
  });
}

export async function resolveAvailableSdkworkChatPcDevPort({
  env = process.env,
  host,
  startPort,
  maxAttempts = MAX_DEV_PORT_ATTEMPTS,
  isPortAvailable = isTcpPortAvailable,
} = {}) {
  const devServer = resolveSdkworkChatPcDevServer({
    env,
    host,
    port: startPort,
  });
  for (let offset = 0; offset < maxAttempts; offset += 1) {
    const candidatePort = devServer.port + offset;
    if (candidatePort > 65535) {
      break;
    }
    if (await isPortAvailable(candidatePort, devServer.host)) {
      return candidatePort;
    }
  }
  throw new Error(
    `No available SDKWork Chat PC dev port found from ${devServer.port} after ${maxAttempts} attempts`,
  );
}

function stripOptionalQuotes(value) {
  if (
    (value.startsWith('"') && value.endsWith('"'))
    || (value.startsWith("'") && value.endsWith("'"))
  ) {
    return value.slice(1, -1);
  }
  return value;
}

function parseEnvFileContent(content) {
  const values = {};
  for (const [lineIndex, rawLine] of String(content ?? '').split(/\r?\n/u).entries()) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    const normalizedLine = line.startsWith('export ') ? line.slice('export '.length).trim() : line;
    const separatorIndex = normalizedLine.indexOf('=');
    if (separatorIndex <= 0) {
      throw new Error(`Invalid env file line ${lineIndex + 1}: ${rawLine}`);
    }
    const name = normalizedLine.slice(0, separatorIndex).trim();
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name)) {
      throw new Error(`Invalid env variable name on line ${lineIndex + 1}: ${name}`);
    }
    const value = stripOptionalQuotes(normalizedLine.slice(separatorIndex + 1).trim());
    values[name] = value;
  }
  return values;
}

function resolveEnvFilePath(envFile, root) {
  const normalized = normalizeText(envFile);
  if (!normalized) {
    return undefined;
  }
  return path.isAbsolute(normalized) ? normalized : path.resolve(root, normalized);
}

function resolveDefaultPostgresEnvFile(root) {
  const localEnvFile = path.resolve(root, '.env.postgres');
  if (fs.existsSync(localEnvFile)) {
    return localEnvFile;
  }
  return path.resolve(root, '.env.postgres.example');
}

export function loadSdkworkChatPcDevEnvFile(envFile, {
  repoRoot: resolvedRepoRoot = repoRoot,
} = {}) {
  const envFilePath = resolveEnvFilePath(envFile, resolvedRepoRoot);
  if (!envFilePath) {
    return {};
  }
  if (!fs.existsSync(envFilePath)) {
    throw new Error(`SDKWork Chat PC dev env file does not exist: ${envFilePath}`);
  }
  return parseEnvFileContent(fs.readFileSync(envFilePath, 'utf8'));
}

export function parseSdkworkChatPcDevArgs(argv = []) {
  const options = {
    database: undefined,
    dryRun: false,
    envFile: undefined,
    target: 'browser',
  };
  const tokens = Array.isArray(argv) ? [...argv] : [];
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token === '--dry-run') {
      options.dryRun = true;
      continue;
    }
    if (token === '--database') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--database requires postgres or sqlite');
      }
      options.database = value;
      index += 1;
      continue;
    }
    if (token === '--target') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--target requires browser or desktop');
      }
      options.target = value;
      index += 1;
      continue;
    }
    if (token === '--dev-env-file') {
      const value = normalizeText(tokens[index + 1]);
      if (!value) {
        throw new Error('--dev-env-file requires a path');
      }
      options.envFile = value;
      index += 1;
      continue;
    }
    throw new Error(`Unknown sdkwork-chat-pc dev argument: ${token}`);
  }
  if (!['postgres', 'postgresql', 'sqlite'].includes(options.database)) {
    if (options.database === undefined) {
      return options;
    }
    throw new Error(`Unsupported sdkwork-chat-pc dev database: ${options.database}`);
  }
  return options;
}

export function createSdkworkChatPcDevPlan({
  argv = [],
  devServerHost,
  devServerPort,
  env = process.env,
  repoRoot: resolvedRepoRoot = repoRoot,
  serverEnv = {},
} = {}) {
  const options = parseSdkworkChatPcDevArgs(argv);
  const target = TARGETS[options.target];
  if (!target) {
    throw new Error(`Unsupported sdkwork-chat-pc dev target: ${options.target}`);
  }
  const defaultDatabaseProfile = options.target === 'desktop' ? 'sqlite' : 'postgres';
  const databaseProfile = options.database === 'postgresql'
    ? 'postgres'
    : options.database ?? defaultDatabaseProfile;
  const defaultEnvFile = databaseProfile === 'postgres'
    ? resolveDefaultPostgresEnvFile(resolvedRepoRoot)
    : undefined;
  const cargoEnv = createCrawChatServerCargoEnv({
    env: {
      ...env,
      ...loadSdkworkChatPcDevEnvFile(options.envFile ?? defaultEnvFile, { repoRoot: resolvedRepoRoot }),
      ...serverEnv,
    },
    repoRoot: resolvedRepoRoot,
  });
  const mergedEnv = {
    ...cargoEnv.env,
  };
  if (databaseProfile === 'sqlite') {
    delete mergedEnv.SDKWORK_CHAT_DATABASE_ENGINE;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_HOST;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_PORT;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_NAME;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_SCHEMA;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_USERNAME;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_PASSWORD;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_SSL_MODE;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_URL;
    delete mergedEnv.SDKWORK_CHAT_DATABASE_MAX_CONNECTIONS;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_PROVIDER;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_HOST;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_PORT;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_NAME;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_SCHEMA;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_USERNAME;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_PASSWORD;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_SSLMODE;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_URL;
    delete mergedEnv.SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS;
    mergedEnv.SDKWORK_CHAT_DEPLOYMENT_MODE = 'desktop';
    mergedEnv.SDKWORK_CHAT_DATABASE_ENGINE = 'sqlite';
  }
  const devServer = resolveSdkworkChatPcDevServer({
    env: mergedEnv,
    host: devServerHost,
    port: devServerPort,
  });
  mergedEnv[SDKWORK_CHAT_PC_DEV_HOST_ENV] = devServer.host;
  mergedEnv[SDKWORK_CHAT_PC_DEV_PORT_ENV] = String(devServer.port);
  const command = pnpmCommand();
  const shared = {
    command,
    cwd: resolvedRepoRoot,
    env: mergedEnv,
    shell: pnpmShell(),
  };
  const resolvedRendererEnv = resolveSdkworkChatIamCommandEnv({
    env: mergedEnv,
    iamMode: 'desktop-local',
    target: 'desktop-dev',
  });
  if (resolvedRendererEnv.errors.length > 0) {
    throw new Error(resolvedRendererEnv.errors.join('\n'));
  }
  const unifiedServerEnv = {
    ...mergedEnv,
    ...resolveCrawChatSharedDatabaseConfig({ env: mergedEnv, repoRoot: resolvedRepoRoot }).env,
    CRAW_CHAT_BROWSER_ORIGINS: mergedEnv.CRAW_CHAT_BROWSER_ORIGINS
      ?? createSdkworkChatBrowserOrigins(devServer),
    CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE: 'embedded',
  };
  return {
    devServer,
    dryRun: options.dryRun,
    target: options.target,
    processes: [
      {
        ...shared,
        args: ['server:dev'],
        env: unifiedServerEnv,
        label: 'craw-chat-server',
      },
      {
        ...shared,
        env: resolvedRendererEnv.env,
        args: target.pnpmArgs,
        label: target.label,
      },
    ],
  };
}

function formatPlan(plan) {
  return plan.processes
    .map((entry) => `[${entry.label}] ${entry.command} ${entry.args.join(' ')}`)
    .join('\n');
}

function prefixOutput(label, stream, chunk) {
  const text = String(chunk ?? '');
  for (const line of text.split(/\r?\n/u)) {
    if (line.length > 0) {
      stream.write(`[${label}] ${line}\n`);
    }
  }
}

function terminateProcessTree(child) {
  if (!child?.pid) {
    return;
  }

  if (process.platform === 'win32') {
    spawnSync('taskkill.exe', ['/PID', String(child.pid), '/T', '/F'], {
      stdio: 'ignore',
      windowsHide: true,
    });
    return;
  }

  child.kill();
}

export async function runSdkworkChatPcDev({
  argv = process.argv.slice(2),
  env = process.env,
  findAvailableDevPort = resolveAvailableSdkworkChatPcDevPort,
  repoRoot: resolvedRepoRoot = repoRoot,
  resolveServerBindEnv = resolveCrawChatServerBindEnv,
  spawnImpl = spawn,
  stdout = process.stdout,
  stderr = process.stderr,
} = {}) {
  const initialPlan = createSdkworkChatPcDevPlan({
    argv,
    env,
    repoRoot: resolvedRepoRoot,
  });
  const resolvedDevPort = await findAvailableDevPort({
    env: initialPlan.processes[1].env,
    host: initialPlan.devServer.host,
    startPort: initialPlan.devServer.port,
  });
  const serverPortPlan = createSdkworkChatPcDevPlan({
    argv,
    devServerHost: initialPlan.devServer.host,
    devServerPort: resolvedDevPort,
    env,
    repoRoot: resolvedRepoRoot,
  });
  const resolvedServerBind = await resolveServerBindEnv({
    env: serverPortPlan.processes[0].env,
  });
  const plan = createSdkworkChatPcDevPlan({
    argv,
    devServerHost: initialPlan.devServer.host,
    devServerPort: resolvedDevPort,
    env,
    repoRoot: resolvedRepoRoot,
    serverEnv: resolvedServerBind.env,
  });
  if (plan.dryRun) {
    stdout.write(`${formatPlan(plan)}\n`);
    return 0;
  }

  const children = [];
  let shuttingDown = false;

  function shutdown(exceptChild) {
    if (shuttingDown) {
      return;
    }
    shuttingDown = true;
    for (const child of children) {
      if (child !== exceptChild && child.exitCode == null && child.signalCode == null) {
        terminateProcessTree(child);
      }
    }
  }

  for (const entry of plan.processes) {
    const child = spawnImpl(entry.command, entry.args, {
      cwd: entry.cwd,
      env: entry.env,
      shell: entry.shell,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    children.push(child);

    child.stdout?.on('data', (chunk) => prefixOutput(entry.label, stdout, chunk));
    child.stderr?.on('data', (chunk) => prefixOutput(entry.label, stderr, chunk));
    child.on('error', (error) => {
      stderr.write(`[${entry.label}] ${error instanceof Error ? error.message : String(error)}\n`);
      shutdown(child);
      process.exitCode = 1;
    });
    child.on('exit', (code, signal) => {
      if (shuttingDown) {
        return;
      }
      shutdown(child);
      if (code && code !== 0) {
        stderr.write(`[${entry.label}] exited with code ${code}\n`);
        process.exitCode = code;
        return;
      }
      if (signal) {
        stderr.write(`[${entry.label}] exited with signal ${signal}\n`);
        process.exitCode = 1;
      }
    });
  }

  const stop = () => shutdown();
  process.once('SIGINT', stop);
  process.once('SIGTERM', stop);
  return undefined;
}

if (path.resolve(process.argv[1] ?? '') === __filename) {
  try {
    const exitCode = await runSdkworkChatPcDev();
    if (Number.isInteger(exitCode)) {
      process.exitCode = exitCode;
    }
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  }
}
