#!/usr/bin/env node

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { runCommandSequence } from '../../sdkwork-appbase/scripts/run-command-sequence.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..');
const DEFAULT_RUNTIME_DIR = './.runtime/local-minimal';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized.length > 0 ? normalized : undefined;
}

function parseKeyValueArgument(value) {
  const normalized = normalizeText(value);
  const separatorIndex = normalized?.indexOf('=') ?? -1;
  if (!normalized || separatorIndex <= 0) {
    throw new TypeError(`expected KEY=VALUE, received ${String(value ?? '')}`);
  }
  return {
    key: normalized.slice(0, separatorIndex).trim(),
    value: normalized.slice(separatorIndex + 1),
  };
}

function nextArgument(argv, index, flagName) {
  const value = argv[index + 1];
  if (value === undefined) {
    throw new TypeError(`${flagName} requires a value`);
  }
  return value;
}

function defaultEnv(runtimeDir = DEFAULT_RUNTIME_DIR) {
  return {
    CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
    CRAW_CHAT_RUNTIME_DIR: runtimeDir,
    CRAW_CHAT_USER_MODULE_PROVIDER: 'local',
  };
}

export function parseDotEnvContent(content = '') {
  const environment = {};
  for (const rawLine of String(content).split(/\r?\n/u)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    const normalizedLine = line.startsWith('export ') ? line.slice(7).trim() : line;
    const separatorIndex = normalizedLine.indexOf('=');
    if (separatorIndex <= 0) {
      continue;
    }
    const key = normalizedLine.slice(0, separatorIndex).trim();
    let value = normalizedLine.slice(separatorIndex + 1).trim();
    if (
      value.length >= 2
      && ((value.startsWith('"') && value.endsWith('"'))
        || (value.startsWith("'") && value.endsWith("'")))
    ) {
      value = value.slice(1, -1);
    }
    environment[key] = value;
  }
  return environment;
}

export function parseArgs(argv = process.argv.slice(2)) {
  const options = {
    bindAddr: undefined,
    browserOrigins: undefined,
    dryRun: false,
    envFile: undefined,
    extraEnv: {},
    help: false,
    noBuild: false,
    runtimeDir: undefined,
    userModuleExternalCatalogPath: undefined,
    userModuleExternalSystem: undefined,
    userModuleProvider: undefined,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    switch (argument) {
      case '--help':
      case '-h':
        options.help = true;
        break;
      case '--no-build':
        options.noBuild = true;
        break;
      case '--dry-run':
        options.dryRun = true;
        break;
      case '--env-file':
        options.envFile = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--bind-addr':
        options.bindAddr = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--runtime_dir':
        options.runtimeDir = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--browser-origins':
        options.browserOrigins = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-module-provider':
        options.userModuleProvider = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-module-external-catalog-path':
        options.userModuleExternalCatalogPath = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-module-external-system':
        options.userModuleExternalSystem = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--set-env': {
        const { key, value } = parseKeyValueArgument(nextArgument(argv, index, argument));
        options.extraEnv[key] = value;
        index += 1;
        break;
      }
      default:
        throw new TypeError(`unknown argument: ${argument}`);
    }
  }

  return options;
}

function resolveDefaultEnvCandidates(resolvedRepoRoot = repoRoot) {
  return [
    path.join(resolvedRepoRoot, '.env.local-minimal'),
    path.join(resolvedRepoRoot, '.env.local'),
    path.join(resolvedRepoRoot, '.env'),
  ];
}

export function resolveEnvFilePath({
  envFile,
  repoRoot: resolvedRepoRoot = repoRoot,
  existsSyncImpl = existsSync,
} = {}) {
  const explicitPath = normalizeText(envFile);
  if (explicitPath) {
    const resolvedPath = path.resolve(resolvedRepoRoot, explicitPath);
    if (!existsSyncImpl(resolvedPath)) {
      throw new Error(`env file does not exist: ${resolvedPath}`);
    }
    return resolvedPath;
  }
  return resolveDefaultEnvCandidates(resolvedRepoRoot).find((candidate) => existsSyncImpl(candidate));
}

function readEnvFile({ envFilePath, readFileSyncImpl = readFileSync } = {}) {
  if (!envFilePath) {
    return {};
  }
  return parseDotEnvContent(readFileSyncImpl(envFilePath, 'utf8'));
}

function createCliEnvironmentOverrides(options) {
  return {
    ...(normalizeText(options.bindAddr)
      ? { CRAW_CHAT_BIND_ADDR: normalizeText(options.bindAddr) }
      : {}),
    ...(normalizeText(options.browserOrigins)
      ? { CRAW_CHAT_BROWSER_ORIGINS: normalizeText(options.browserOrigins) }
      : {}),
    ...(normalizeText(options.runtimeDir)
      ? { CRAW_CHAT_RUNTIME_DIR: normalizeText(options.runtimeDir) }
      : {}),
    ...(normalizeText(options.userModuleProvider)
      ? { CRAW_CHAT_USER_MODULE_PROVIDER: normalizeText(options.userModuleProvider) }
      : {}),
    ...(normalizeText(options.userModuleExternalCatalogPath)
      ? {
          CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH:
            normalizeText(options.userModuleExternalCatalogPath),
        }
      : {}),
    ...(normalizeText(options.userModuleExternalSystem)
      ? { CRAW_CHAT_USER_MODULE_EXTERNAL_SYSTEM: normalizeText(options.userModuleExternalSystem) }
      : {}),
    ...(options.extraEnv ?? {}),
  };
}

function maybeOverrideDerivedDefaults(environment) {
  const env = { ...environment };
  if (!normalizeText(env.CRAW_CHAT_RUNTIME_DIR)) {
    env.CRAW_CHAT_RUNTIME_DIR = DEFAULT_RUNTIME_DIR;
  }
  if (!normalizeText(env.CRAW_CHAT_USER_MODULE_PROVIDER)) {
    env.CRAW_CHAT_USER_MODULE_PROVIDER = 'local';
  }
  return env;
}

export function createRunLocalMinimalEnvironment({
  baseEnv = process.env,
  envFileEnv = {},
  options = {},
  repoRoot: resolvedRepoRoot = repoRoot,
} = {}) {
  const runtimeDir = normalizeText(options.runtimeDir)
    ?? normalizeText(envFileEnv.CRAW_CHAT_RUNTIME_DIR)
    ?? normalizeText(baseEnv.CRAW_CHAT_RUNTIME_DIR)
    ?? DEFAULT_RUNTIME_DIR;
  const mergedEnv = {
    ...defaultEnv(runtimeDir),
    ...baseEnv,
    ...envFileEnv,
    ...createCliEnvironmentOverrides(options),
  };
  const normalizedEnv = maybeOverrideDerivedDefaults(mergedEnv);
  normalizedEnv.PWD = normalizeText(baseEnv.PWD) ?? resolvedRepoRoot;
  return normalizedEnv;
}

function requireConfiguredValue(environment, envName) {
  const value = normalizeText(environment?.[envName]);
  if (!value) {
    throw new Error(`${envName} is required for the selected local-minimal deployment mode`);
  }
  return value;
}

export function assertRunLocalMinimalEnvironment(environment = {}) {
  const userModuleProvider = normalizeText(environment.CRAW_CHAT_USER_MODULE_PROVIDER)?.toLowerCase();
  if (userModuleProvider === 'external') {
    requireConfiguredValue(environment, 'CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH');
  }
  return environment;
}

export function createRunLocalMinimalCommandPlan({
  cargoExecutable = 'cargo',
  env,
  noBuild = false,
  repoRoot: resolvedRepoRoot = repoRoot,
} = {}) {
  const commands = [];
  if (!noBuild) {
    commands.push({
      args: ['build', '-p', 'local-minimal-node', '--offline'],
      command: cargoExecutable,
      cwd: resolvedRepoRoot,
      env,
      label: 'build local-minimal-node',
    });
  }
  commands.push({
    args: ['run', '-p', 'local-minimal-node', '--offline'],
    command: cargoExecutable,
    cwd: resolvedRepoRoot,
    env,
    label: 'run local-minimal-node',
  });
  return commands;
}

function formatCommandPlan(plan) {
  return plan.map((step) => {
    const bindAddr = normalizeText(step.env?.CRAW_CHAT_BIND_ADDR);
    const runtimeDir = normalizeText(step.env?.CRAW_CHAT_RUNTIME_DIR);
    return [
      `[${step.label}] ${step.command} ${step.args.join(' ')}`,
      ...(bindAddr ? [`  CRAW_CHAT_BIND_ADDR=${bindAddr}`] : []),
      ...(runtimeDir ? [`  CRAW_CHAT_RUNTIME_DIR=${runtimeDir}`] : []),
    ].join('\n');
  }).join('\n');
}

function createUsageText() {
  return [
    'Usage: node scripts/run-local-minimal.mjs [options]',
    '',
    'Options:',
    '  --env-file <path>                                   Load an env file before applying CLI overrides',
    '  --bind-addr <host:port>                            Override CRAW_CHAT_BIND_ADDR',
    '  --runtime_dir <path>                               Override CRAW_CHAT_RUNTIME_DIR',
    '  --browser-origins <csv>                            Override CRAW_CHAT_BROWSER_ORIGINS',
    '  --user-module-provider <local|external>            Override CRAW_CHAT_USER_MODULE_PROVIDER',
    '  --user-module-external-catalog-path <path>         Override CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH',
    '  --user-module-external-system <value>              Override CRAW_CHAT_USER_MODULE_EXTERNAL_SYSTEM',
    '  --set-env KEY=VALUE                                Inject an additional environment variable',
    '  --no-build                                         Skip cargo build and only run the service',
    '  --dry-run                                          Print the execution plan without starting the service',
    '  --help                                             Show this help text',
    '',
    'Precedence: CLI > env file > process.env > defaults',
  ].join('\n');
}

export function runLocalMinimal({
  argv = process.argv.slice(2),
  baseEnv = process.env,
  cargoExecutable = baseEnv.CARGO ?? 'cargo',
  existsSyncImpl = existsSync,
  readFileSyncImpl = readFileSync,
  repoRoot: resolvedRepoRoot = repoRoot,
  runCommandSequenceImpl = runCommandSequence,
  stdout = process.stdout,
} = {}) {
  const options = parseArgs(argv);
  if (options.help) {
    stdout.write(`${createUsageText()}\n`);
    return 0;
  }
  const envFilePath = resolveEnvFilePath({
    envFile: options.envFile,
    existsSyncImpl,
    repoRoot: resolvedRepoRoot,
  });
  const envFileEnv = readEnvFile({ envFilePath, readFileSyncImpl });
  const environment = createRunLocalMinimalEnvironment({
    baseEnv,
    envFileEnv,
    options,
    repoRoot: resolvedRepoRoot,
  });
  assertRunLocalMinimalEnvironment(environment);
  const plan = createRunLocalMinimalCommandPlan({
    cargoExecutable,
    env: environment,
    noBuild: options.noBuild,
    repoRoot: resolvedRepoRoot,
  });

  if (options.dryRun) {
    stdout.write(`${formatCommandPlan(plan)}\n`);
    return 0;
  }

  return runCommandSequenceImpl(plan);
}

const isDirectExecution = process.argv[1] && path.resolve(process.argv[1]) === __filename;
if (isDirectExecution) {
  try {
    const exitCode = await runLocalMinimal();
    process.exitCode = Number.isInteger(exitCode) ? exitCode : 0;
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  }
}
