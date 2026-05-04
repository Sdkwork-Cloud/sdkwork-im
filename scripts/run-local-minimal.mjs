#!/usr/bin/env node

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { runCommandSequence } from '../../sdkwork-appbase/scripts/run-command-sequence.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..');

export const USER_CENTER_CANONICAL_ENV_NAMES = Object.freeze({
  accessTokenHeaderName: 'SDKWORK_USER_CENTER_ACCESS_TOKEN_HEADER_NAME',
  allowAuthorizationFallbackToAccessToken:
    'SDKWORK_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN',
  appApiBaseUrl: 'SDKWORK_USER_CENTER_APP_API_BASE_URL',
  appId: 'SDKWORK_USER_CENTER_APP_ID',
  authorizationHeaderName: 'SDKWORK_USER_CENTER_AUTHORIZATION_HEADER_NAME',
  authorizationScheme: 'SDKWORK_USER_CENTER_AUTHORIZATION_SCHEME',
  databaseUrl: 'SDKWORK_USER_CENTER_DATABASE_URL',
  externalBaseUrl: 'SDKWORK_USER_CENTER_EXTERNAL_BASE_URL',
  handshakeFreshnessWindowMs: 'SDKWORK_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS',
  localApiBasePath: 'SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH',
  mode: 'SDKWORK_USER_CENTER_MODE',
  providerKey: 'SDKWORK_USER_CENTER_PROVIDER_KEY',
  refreshTokenHeaderName: 'SDKWORK_USER_CENTER_REFRESH_TOKEN_HEADER_NAME',
  schemaName: 'SDKWORK_USER_CENTER_SCHEMA_NAME',
  secretId: 'SDKWORK_USER_CENTER_SECRET_ID',
  sessionHeaderName: 'SDKWORK_USER_CENTER_SESSION_HEADER_NAME',
  sharedSecret: 'SDKWORK_USER_CENTER_SHARED_SECRET',
  sqlitePath: 'SDKWORK_USER_CENTER_SQLITE_PATH',
  tablePrefix: 'SDKWORK_USER_CENTER_TABLE_PREFIX',
});

const USER_CENTER_APP_ENV_PREFIX = 'CRAW_CHAT_USER_CENTER_';
const DEFAULT_RUNTIME_DIR = './.runtime/local-minimal';
const DEFAULT_SQLITE_BASENAME = 'data/user-center.db';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized.length > 0 ? normalized : undefined;
}

function normalizeBooleanFlag(value) {
  const normalized = normalizeText(value)?.toLowerCase();
  if (!normalized) {
    return undefined;
  }

  if (['1', 'true', 'yes', 'on'].includes(normalized)) {
    return 'true';
  }

  if (['0', 'false', 'no', 'off'].includes(normalized)) {
    return 'false';
  }

  return normalized;
}

function normalizeUserCenterMode(value) {
  const normalized = normalizeText(value)?.toLowerCase();
  if (!normalized) {
    return undefined;
  }

  if (normalized === 'builtin-local') {
    return 'builtin-local';
  }

  if (
    [
      'sdkwork-cloud-app-api',
    ].includes(normalized)
  ) {
    return 'sdkwork-cloud-app-api';
  }

  if (normalized === 'external-user-center') {
    return 'external-user-center';
  }

  return undefined;
}

function createCrawChatUserCenterEnvName(canonicalName) {
  const suffix = String(canonicalName).replace(/^SDKWORK_USER_CENTER_/u, '');
  return `${USER_CENTER_APP_ENV_PREFIX}${suffix}`;
}

function defaultSqlitePath(runtimeDir = DEFAULT_RUNTIME_DIR) {
  const normalizedRuntimeDir = normalizeText(runtimeDir) ?? DEFAULT_RUNTIME_DIR;
  const portableRuntimeDir = normalizedRuntimeDir.replaceAll('\\', '/').replace(/\/+$/u, '');
  return `${portableRuntimeDir}/${DEFAULT_SQLITE_BASENAME}`;
}

function defaultEnv(runtimeDir = DEFAULT_RUNTIME_DIR) {
  return {
    CRAW_CHAT_BIND_ADDR: '127.0.0.1:18090',
    CRAW_CHAT_RUNTIME_DIR: runtimeDir,
    CRAW_CHAT_USER_MODULE_PROVIDER: 'local',
    [USER_CENTER_CANONICAL_ENV_NAMES.accessTokenHeaderName]: 'Access-Token',
    [USER_CENTER_CANONICAL_ENV_NAMES.allowAuthorizationFallbackToAccessToken]: 'true',
    [USER_CENTER_CANONICAL_ENV_NAMES.appId]: 'craw-chat',
    [USER_CENTER_CANONICAL_ENV_NAMES.authorizationHeaderName]: 'Authorization',
    [USER_CENTER_CANONICAL_ENV_NAMES.authorizationScheme]: 'Bearer',
    [USER_CENTER_CANONICAL_ENV_NAMES.localApiBasePath]: '/api/app/v1/user-center',
    [USER_CENTER_CANONICAL_ENV_NAMES.mode]: 'builtin-local',
    [USER_CENTER_CANONICAL_ENV_NAMES.providerKey]: 'craw-chat-local',
    [USER_CENTER_CANONICAL_ENV_NAMES.refreshTokenHeaderName]: 'Refresh-Token',
    [USER_CENTER_CANONICAL_ENV_NAMES.sessionHeaderName]: 'x-sdkwork-user-center-session-id',
    [USER_CENTER_CANONICAL_ENV_NAMES.sqlitePath]: defaultSqlitePath(runtimeDir),
    [USER_CENTER_CANONICAL_ENV_NAMES.tablePrefix]: 'cc_uc_',
  };
}

function normalizeEnvValue(value) {
  const normalized = normalizeText(value);
  return normalized === undefined ? undefined : normalized;
}

function applyUserCenterAliases(environment) {
  const env = { ...environment };

  for (const canonicalName of Object.values(USER_CENTER_CANONICAL_ENV_NAMES)) {
    const appAlias = createCrawChatUserCenterEnvName(canonicalName);
    const resolvedValue = normalizeEnvValue(env[canonicalName]) ?? normalizeEnvValue(env[appAlias]);
    if (resolvedValue === undefined) {
      continue;
    }

    env[canonicalName] = resolvedValue;
    env[appAlias] = resolvedValue;
  }

  return env;
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

export function parseArgs(argv = process.argv.slice(2)) {
  const options = {
    bindAddr: undefined,
    browserOrigins: undefined,
    dryRun: false,
    envFile: undefined,
    extraEnv: {},
    help: false,
    noBuild: false,
    publicBearerSecret: undefined,
    runtimeDir: undefined,
    userCenterAccessTokenHeaderName: undefined,
    userCenterAllowAuthorizationFallbackToAccessToken: undefined,
    userCenterAppApiBaseUrl: undefined,
    userCenterAppId: undefined,
    userCenterAuthorizationHeaderName: undefined,
    userCenterAuthorizationScheme: undefined,
    userCenterDatabaseUrl: undefined,
    userCenterExternalBaseUrl: undefined,
    userCenterHandshakeFreshnessWindowMs: undefined,
    userCenterLocalApiBasePath: undefined,
    userCenterMode: undefined,
    userCenterProviderKey: undefined,
    userCenterRefreshTokenHeaderName: undefined,
    userCenterSchemaName: undefined,
    userCenterSecretId: undefined,
    userCenterSessionHeaderName: undefined,
    userCenterSharedSecret: undefined,
    userCenterSqlitePath: undefined,
    userCenterTablePrefix: undefined,
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
      case '--runtime-dir':
        options.runtimeDir = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--browser-origins':
        options.browserOrigins = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--public-bearer-secret':
        options.publicBearerSecret = nextArgument(argv, index, argument);
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
      case '--user-center-mode':
        options.userCenterMode = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-app-api-base-url':
        options.userCenterAppApiBaseUrl = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-external-base-url':
        options.userCenterExternalBaseUrl = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-provider-key':
        options.userCenterProviderKey = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-app-id':
        options.userCenterAppId = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-local-api-base-path':
        options.userCenterLocalApiBasePath = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-sqlite-path':
        options.userCenterSqlitePath = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-database-url':
        options.userCenterDatabaseUrl = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-schema-name':
        options.userCenterSchemaName = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-table-prefix':
        options.userCenterTablePrefix = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-handshake-freshness-window-ms':
        options.userCenterHandshakeFreshnessWindowMs = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-secret-id':
        options.userCenterSecretId = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-shared-secret':
        options.userCenterSharedSecret = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-authorization-header-name':
        options.userCenterAuthorizationHeaderName = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-access-token-header-name':
        options.userCenterAccessTokenHeaderName = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-refresh-token-header-name':
        options.userCenterRefreshTokenHeaderName = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-session-header-name':
        options.userCenterSessionHeaderName = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-authorization-scheme':
        options.userCenterAuthorizationScheme = nextArgument(argv, index, argument);
        index += 1;
        break;
      case '--user-center-allow-authorization-fallback-to-access-token':
        options.userCenterAllowAuthorizationFallbackToAccessToken = nextArgument(
          argv,
          index,
          argument,
        );
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

function readEnvFile({
  envFilePath,
  readFileSyncImpl = readFileSync,
} = {}) {
  if (!envFilePath) {
    return {};
  }

  return parseDotEnvContent(readFileSyncImpl(envFilePath, 'utf8'));
}

function createCliEnvironmentOverrides(options, runtimeDir) {
  const normalizedCliUserCenterMode = normalizeUserCenterMode(options.userCenterMode);
  if (normalizeText(options.userCenterMode) && !normalizedCliUserCenterMode) {
    throw new TypeError(
      '--user-center-mode must be one of: builtin-local, sdkwork-cloud-app-api, external-user-center',
    );
  }

  const cliEnv = {
    ...(normalizeText(options.bindAddr)
      ? { CRAW_CHAT_BIND_ADDR: normalizeText(options.bindAddr) }
      : {}),
    ...(normalizeText(options.browserOrigins)
      ? { CRAW_CHAT_BROWSER_ORIGINS: normalizeText(options.browserOrigins) }
      : {}),
    ...(normalizeText(options.publicBearerSecret)
      ? { CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET: normalizeText(options.publicBearerSecret) }
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
    ...(normalizeText(options.userCenterAccessTokenHeaderName)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.accessTokenHeaderName]:
            normalizeText(options.userCenterAccessTokenHeaderName),
        }
      : {}),
    ...(normalizeText(options.userCenterAppApiBaseUrl)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.appApiBaseUrl]:
            normalizeText(options.userCenterAppApiBaseUrl),
        }
      : {}),
    ...(normalizeText(options.userCenterAppId)
      ? { [USER_CENTER_CANONICAL_ENV_NAMES.appId]: normalizeText(options.userCenterAppId) }
      : {}),
    ...(normalizeText(options.userCenterAuthorizationHeaderName)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.authorizationHeaderName]:
            normalizeText(options.userCenterAuthorizationHeaderName),
        }
      : {}),
    ...(normalizeText(options.userCenterAuthorizationScheme)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.authorizationScheme]:
            normalizeText(options.userCenterAuthorizationScheme),
        }
      : {}),
    ...(normalizeText(options.userCenterDatabaseUrl)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.databaseUrl]:
            normalizeText(options.userCenterDatabaseUrl),
        }
      : {}),
    ...(normalizeText(options.userCenterExternalBaseUrl)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.externalBaseUrl]:
            normalizeText(options.userCenterExternalBaseUrl),
        }
      : {}),
    ...(normalizeText(options.userCenterHandshakeFreshnessWindowMs)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.handshakeFreshnessWindowMs]:
            normalizeText(options.userCenterHandshakeFreshnessWindowMs),
        }
      : {}),
    ...(normalizeText(options.userCenterLocalApiBasePath)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.localApiBasePath]:
            normalizeText(options.userCenterLocalApiBasePath),
        }
      : {}),
    ...(normalizedCliUserCenterMode
      ? { [USER_CENTER_CANONICAL_ENV_NAMES.mode]: normalizedCliUserCenterMode }
      : {}),
    ...(normalizeText(options.userCenterProviderKey)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.providerKey]:
            normalizeText(options.userCenterProviderKey),
        }
      : {}),
    ...(normalizeText(options.userCenterRefreshTokenHeaderName)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.refreshTokenHeaderName]:
            normalizeText(options.userCenterRefreshTokenHeaderName),
        }
      : {}),
    ...(normalizeText(options.userCenterSchemaName)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.schemaName]:
            normalizeText(options.userCenterSchemaName),
        }
      : {}),
    ...(normalizeText(options.userCenterSecretId)
      ? { [USER_CENTER_CANONICAL_ENV_NAMES.secretId]: normalizeText(options.userCenterSecretId) }
      : {}),
    ...(normalizeText(options.userCenterSessionHeaderName)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.sessionHeaderName]:
            normalizeText(options.userCenterSessionHeaderName),
        }
      : {}),
    ...(normalizeText(options.userCenterSharedSecret)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.sharedSecret]:
            normalizeText(options.userCenterSharedSecret),
        }
      : {}),
    ...(normalizeText(options.userCenterSqlitePath)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.sqlitePath]:
            normalizeText(options.userCenterSqlitePath),
        }
      : {}),
    ...(normalizeText(options.userCenterTablePrefix)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.tablePrefix]:
            normalizeText(options.userCenterTablePrefix),
        }
      : {}),
    ...(normalizeBooleanFlag(options.userCenterAllowAuthorizationFallbackToAccessToken)
      ? {
          [USER_CENTER_CANONICAL_ENV_NAMES.allowAuthorizationFallbackToAccessToken]:
            normalizeBooleanFlag(options.userCenterAllowAuthorizationFallbackToAccessToken),
        }
      : {}),
    ...(options.extraEnv ?? {}),
  };

  if (
    !Object.hasOwn(cliEnv, USER_CENTER_CANONICAL_ENV_NAMES.sqlitePath)
    && normalizeText(runtimeDir)
  ) {
    cliEnv[USER_CENTER_CANONICAL_ENV_NAMES.sqlitePath] = defaultSqlitePath(runtimeDir);
  }

  return cliEnv;
}

function maybeOverrideDerivedDefaults(environment) {
  const env = { ...environment };
  const runtimeDir = normalizeText(env.CRAW_CHAT_RUNTIME_DIR) ?? DEFAULT_RUNTIME_DIR;
  const configuredSqlitePath = normalizeText(env[USER_CENTER_CANONICAL_ENV_NAMES.sqlitePath]);
  if (!configuredSqlitePath) {
    env[USER_CENTER_CANONICAL_ENV_NAMES.sqlitePath] = defaultSqlitePath(runtimeDir);
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
    ...createCliEnvironmentOverrides(options, runtimeDir),
  };

  const normalizedEnv = applyUserCenterAliases(maybeOverrideDerivedDefaults(mergedEnv));
  if (!normalizeText(normalizedEnv.CRAW_CHAT_RUNTIME_DIR)) {
    normalizedEnv.CRAW_CHAT_RUNTIME_DIR = DEFAULT_RUNTIME_DIR;
  }

  normalizedEnv.PWD = normalizeText(baseEnv.PWD) ?? resolvedRepoRoot;
  return normalizedEnv;
}

function resolveConfiguredUserCenterMode(environment = {}) {
  const rawMode =
    normalizeText(environment[USER_CENTER_CANONICAL_ENV_NAMES.mode])
    ?? normalizeText(
      environment[createCrawChatUserCenterEnvName(USER_CENTER_CANONICAL_ENV_NAMES.mode)],
    );
  if (!rawMode) {
    return 'builtin-local';
  }

  const normalizedMode = normalizeUserCenterMode(rawMode);
  if (!normalizedMode) {
    throw new Error(
      `${USER_CENTER_CANONICAL_ENV_NAMES.mode} must be one of: builtin-local, sdkwork-cloud-app-api, external-user-center`,
    );
  }

  return normalizedMode;
}

function requireConfiguredValue(environment, envName) {
  const value = normalizeText(environment?.[envName]);
  if (!value) {
    throw new Error(`${envName} is required for the selected local-minimal deployment mode`);
  }

  return value;
}

export function assertRunLocalMinimalEnvironment(environment = {}) {
  const mode = resolveConfiguredUserCenterMode(environment);
  const userModuleProvider = normalizeText(environment.CRAW_CHAT_USER_MODULE_PROVIDER)?.toLowerCase();

  if (mode === 'sdkwork-cloud-app-api') {
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.appApiBaseUrl);
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.providerKey);
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.secretId);
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.sharedSecret);
  }

  if (mode === 'external-user-center') {
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.externalBaseUrl);
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.providerKey);
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.secretId);
    requireConfiguredValue(environment, USER_CENTER_CANONICAL_ENV_NAMES.sharedSecret);
  }

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
    const mode = normalizeText(step.env?.[USER_CENTER_CANONICAL_ENV_NAMES.mode]);
    return [
      `[${step.label}] ${step.command} ${step.args.join(' ')}`,
      ...(bindAddr ? [`  CRAW_CHAT_BIND_ADDR=${bindAddr}`] : []),
      ...(runtimeDir ? [`  CRAW_CHAT_RUNTIME_DIR=${runtimeDir}`] : []),
      ...(mode ? [`  SDKWORK_USER_CENTER_MODE=${mode}`] : []),
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
    '  --runtime-dir <path>                               Override CRAW_CHAT_RUNTIME_DIR',
    '  --browser-origins <csv>                            Override CRAW_CHAT_BROWSER_ORIGINS',
    '  --public-bearer-secret <secret>                    Override CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET',
    '  --user-module-provider <local|external>            Override CRAW_CHAT_USER_MODULE_PROVIDER',
    '  --user-module-external-catalog-path <path>         Override CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH',
    '  --user-module-external-system <value>              Override CRAW_CHAT_USER_MODULE_EXTERNAL_SYSTEM',
    '  --user-center-mode <builtin-local|sdkwork-cloud-app-api|external-user-center>',
    '  --user-center-app-api-base-url <url>',
    '  --user-center-external-base-url <url>',
    '  --user-center-provider-key <value>',
    '  --user-center-app-id <value>',
    '  --user-center-local-api-base-path <path>',
    '  --user-center-sqlite-path <path>',
    '  --user-center-database-url <url>',
    '  --user-center-schema-name <value>',
    '  --user-center-table-prefix <value>',
    '  --user-center-handshake-freshness-window-ms <ms>',
    '  --user-center-secret-id <value>',
    '  --user-center-shared-secret <value>',
    '  --user-center-authorization-header-name <value>',
    '  --user-center-access-token-header-name <value>',
    '  --user-center-refresh-token-header-name <value>',
    '  --user-center-session-header-name <value>',
    '  --user-center-authorization-scheme <value>',
    '  --user-center-allow-authorization-fallback-to-access-token <true|false>',
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
  const envFileEnv = readEnvFile({
    envFilePath,
    readFileSyncImpl,
  });
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

  return runCommandSequenceImpl({
    commands: plan.map((step) => ({
      command: step.command,
      args: step.args,
    })),
    cwd: resolvedRepoRoot,
    env: environment,
  });
}

function isDirectExecution({
  argv1 = process.argv[1] ?? '',
  moduleFile = __filename,
  platform = process.platform,
} = {}) {
  if (!argv1) {
    return false;
  }

  const resolvedArgv1 = path.resolve(argv1);
  const resolvedModuleFile = path.resolve(moduleFile);
  if (platform === 'win32') {
    return resolvedArgv1.toLowerCase() === resolvedModuleFile.toLowerCase();
  }

  return resolvedArgv1 === resolvedModuleFile;
}

if (isDirectExecution()) {
  process.exitCode = runLocalMinimal();
}
