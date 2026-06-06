#!/usr/bin/env node

export const SDKWORK_CHAT_IAM_DEPLOYMENT_MODES = Object.freeze([
  'desktop-local',
  'server-private',
  'cloud-saas',
]);

export const SDKWORK_CHAT_IAM_MODES = Object.freeze([
  'local',
  'private',
  'cloud',
]);

export const DEFAULT_SDKWORK_CHAT_LOCAL_APP_API_BASE_URL = 'http://127.0.0.1:18079';
export const DEFAULT_SDKWORK_CHAT_LOCAL_IM_API_BASE_URL = 'http://127.0.0.1:18079';
export const DEFAULT_SDKWORK_CHAT_LOCAL_IM_WEBSOCKET_BASE_URL = 'ws://127.0.0.1:18079';
export const DEFAULT_SDKWORK_IAM_LOCAL_BOOTSTRAP_EMAIL = 'local-default@sdkwork-iam.local';
export const DEFAULT_SDKWORK_IAM_LOCAL_BOOTSTRAP_PHONE = '13800000000';
export const DEFAULT_SDKWORK_IAM_LOCAL_BOOTSTRAP_PASSWORD = 'dev123456';
export const DEFAULT_SDKWORK_IAM_LOCAL_VERIFY_CODE = '123456';

const SDKWORK_CHAT_IAM_DEPLOYMENT_MODE_ENV = 'SDKWORK_CHAT_IAM_DEPLOYMENT_MODE';
const VITE_SDKWORK_CHAT_IAM_DEPLOYMENT_MODE_ENV = 'VITE_SDKWORK_CHAT_IAM_DEPLOYMENT_MODE';
const VITE_SDKWORK_DEPLOYMENT_MODE_ENV = 'VITE_SDKWORK_DEPLOYMENT_MODE';
const SDKWORK_IAM_MODE_ENV = 'SDKWORK_IAM_MODE';
const SDKWORK_IAM_LOCAL_BOOTSTRAP_EMAIL_ENV = 'SDKWORK_IAM_LOCAL_BOOTSTRAP_EMAIL';
const SDKWORK_IAM_LOCAL_BOOTSTRAP_PHONE_ENV = 'SDKWORK_IAM_LOCAL_BOOTSTRAP_PHONE';
const SDKWORK_IAM_LOCAL_BOOTSTRAP_PASSWORD_ENV = 'SDKWORK_IAM_LOCAL_BOOTSTRAP_PASSWORD';
const SDKWORK_IAM_LOCAL_VERIFY_CODE_FIXED_ENV = 'SDKWORK_IAM_LOCAL_VERIFY_CODE_FIXED';
const SDKWORK_IAM_APP_API_BASE_URL_ENV = 'SDKWORK_IAM_APP_API_BASE_URL';
const VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV = 'VITE_SDKWORK_IAM_APP_API_BASE_URL';
const CRAW_CHAT_APP_API_BASE_URL_ENV = 'CRAW_CHAT_APP_API_BASE_URL';
const VITE_CRAW_CHAT_APP_API_BASE_URL_ENV = 'VITE_CRAW_CHAT_APP_API_BASE_URL';
const CRAW_CHAT_IM_API_BASE_URL_ENV = 'CRAW_CHAT_IM_API_BASE_URL';
const VITE_CRAW_CHAT_IM_API_BASE_URL_ENV = 'VITE_CRAW_CHAT_IM_API_BASE_URL';
const CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV = 'CRAW_CHAT_IM_WEBSOCKET_BASE_URL';
const VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV = 'VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL';
const CRAW_CHAT_SERVER_BASE_URL_ENV = 'CRAW_CHAT_SERVER_BASE_URL';
const CRAW_CHAT_SERVER_API_BASE_URL_ENV = 'CRAW_CHAT_SERVER_API_BASE_URL';
const CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL_ENV = 'CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL';
const SDKWORK_CHAT_SERVER_BASE_URL_ENV = 'SDKWORK_CHAT_SERVER_BASE_URL';
const SDKWORK_CHAT_SERVER_API_BASE_URL_ENV = 'SDKWORK_CHAT_SERVER_API_BASE_URL';
const SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL_ENV = 'SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL';
const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';
const SDKWORK_IM_REALTIME_WEBSOCKET_PATH = '/im/v3/api/realtime/ws';

function readTrimmedValue(value) {
  const normalizedValue = String(value ?? '').trim();
  return normalizedValue || undefined;
}

function stripSdkOwnedPathSuffix(pathname, suffixes = []) {
  const normalizedPathname = pathname.replace(/\/+$/u, '');
  if (!normalizedPathname || normalizedPathname === '/') {
    return '';
  }

  for (const suffix of suffixes) {
    const normalizedSuffix = `/${String(suffix).replace(/^\/+|\/+$/gu, '')}`;
    if (normalizedPathname === normalizedSuffix) {
      return '';
    }
    if (normalizedPathname.endsWith(normalizedSuffix)) {
      return normalizedPathname.slice(0, -normalizedSuffix.length) || '';
    }
  }

  return normalizedPathname;
}

function normalizeHttpBaseUrl(value, sdkOwnedPathSuffixes = []) {
  const normalizedValue = readTrimmedValue(value);
  if (!normalizedValue) {
    return undefined;
  }

  try {
    const parsedUrl = new URL(normalizedValue);
    if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
      return undefined;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(
      parsedUrl.pathname,
      sdkOwnedPathSuffixes,
    );
    return `${parsedUrl.origin}${normalizedPathname === '/' ? '' : normalizedPathname}`;
  } catch {
    return undefined;
  }
}

function normalizeWebSocketBaseUrl(value, sdkOwnedPathSuffixes = []) {
  const normalizedValue = readTrimmedValue(value);
  if (!normalizedValue) {
    return undefined;
  }

  try {
    const parsedUrl = new URL(normalizedValue);
    if (parsedUrl.protocol !== 'ws:' && parsedUrl.protocol !== 'wss:') {
      return undefined;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(
      parsedUrl.pathname,
      sdkOwnedPathSuffixes,
    );
    return `${parsedUrl.origin}${normalizedPathname === '/' ? '' : normalizedPathname}`;
  } catch {
    return undefined;
  }
}

function deriveWebSocketBaseUrlFromHttpBaseUrl(value) {
  const normalizedValue = normalizeHttpBaseUrl(value, [
    SDKWORK_APP_API_PREFIX,
    SDKWORK_IM_API_PREFIX,
  ]);
  if (!normalizedValue) {
    return undefined;
  }

  const parsedUrl = new URL(normalizedValue);
  parsedUrl.protocol = parsedUrl.protocol === 'https:' ? 'wss:' : 'ws:';
  return normalizeWebSocketBaseUrl(parsedUrl.toString());
}

function setEnvValue(env, key, value) {
  const normalizedValue = readTrimmedValue(value);
  if (!normalizedValue) {
    delete env[key];
    return;
  }

  env[key] = normalizedValue;
}

function setEnvDefault(env, key, value) {
  if (!readTrimmedValue(env[key])) {
    setEnvValue(env, key, value);
  }
}

function firstConfiguredEnvKey(env, keys) {
  return keys.find((key) => readTrimmedValue(env[key]));
}

function validateConfiguredHttpEnv(env, keys, errors) {
  const configuredKey = firstConfiguredEnvKey(env, keys);
  if (!configuredKey) {
    return;
  }
  if (!normalizeHttpBaseUrl(env[configuredKey], [
    SDKWORK_APP_API_PREFIX,
    SDKWORK_IM_API_PREFIX,
  ])) {
    errors.push(`${configuredKey} must be a valid absolute http(s) URL.`);
  }
}

function validateConfiguredWebSocketEnv(env, keys, errors) {
  const configuredKey = firstConfiguredEnvKey(env, keys);
  if (!configuredKey) {
    return;
  }
  if (!normalizeWebSocketBaseUrl(env[configuredKey], [
    SDKWORK_IM_REALTIME_WEBSOCKET_PATH,
    SDKWORK_IM_API_PREFIX,
  ])) {
    errors.push(`${configuredKey} must be a valid absolute ws(s) URL.`);
  }
}

function normalizeDeploymentMode(value, fallback = 'desktop-local') {
  const normalizedValue = readTrimmedValue(value)?.toLowerCase();
  if (!normalizedValue) {
    return fallback;
  }
  return SDKWORK_CHAT_IAM_DEPLOYMENT_MODES.includes(normalizedValue)
    ? normalizedValue
    : fallback;
}

function resolveSdkworkIamMode(iamMode) {
  if (iamMode === 'cloud-saas') {
    return 'cloud';
  }
  if (iamMode === 'server-private') {
    return 'private';
  }
  return 'local';
}

function resolvePublicDeploymentMode(iamMode) {
  if (iamMode === 'cloud-saas') {
    return 'saas';
  }
  if (iamMode === 'server-private') {
    return 'private';
  }
  return 'local';
}

function resolveDefaultDeploymentMode(target) {
  if (target === 'web-build' || target === 'server-build' || target === 'server-dev') {
    return 'server-private';
  }
  return 'desktop-local';
}

function shouldUseLocalEndpointDefaults(target, resolvedDeploymentMode) {
  if (resolvedDeploymentMode === 'desktop-local') {
    return true;
  }
  return target === 'desktop-dev' || target === 'desktop-build' || target === 'browser-dev';
}

function resolveConfiguredApiBaseUrl(env) {
  return normalizeHttpBaseUrl(
    env[VITE_CRAW_CHAT_APP_API_BASE_URL_ENV]
      ?? env[CRAW_CHAT_APP_API_BASE_URL_ENV]
      ?? env[VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV]
      ?? env[SDKWORK_IAM_APP_API_BASE_URL_ENV]
      ?? env[SDKWORK_CHAT_SERVER_API_BASE_URL_ENV]
      ?? env[SDKWORK_CHAT_SERVER_BASE_URL_ENV]
      ?? env[CRAW_CHAT_SERVER_API_BASE_URL_ENV]
      ?? env[CRAW_CHAT_SERVER_BASE_URL_ENV],
    [SDKWORK_APP_API_PREFIX, SDKWORK_IM_API_PREFIX],
  );
}

function resolveConfiguredImApiBaseUrl(env) {
  return normalizeHttpBaseUrl(
    env[VITE_CRAW_CHAT_IM_API_BASE_URL_ENV]
      ?? env[CRAW_CHAT_IM_API_BASE_URL_ENV]
      ?? env[SDKWORK_CHAT_SERVER_API_BASE_URL_ENV]
      ?? env[SDKWORK_CHAT_SERVER_BASE_URL_ENV]
      ?? env[CRAW_CHAT_SERVER_API_BASE_URL_ENV]
      ?? env[CRAW_CHAT_SERVER_BASE_URL_ENV],
    [SDKWORK_APP_API_PREFIX, SDKWORK_IM_API_PREFIX],
  );
}

function resolveConfiguredImWebSocketBaseUrl(env) {
  return normalizeWebSocketBaseUrl(
    env[VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV]
      ?? env[CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV]
      ?? env[SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL_ENV]
      ?? env[CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL_ENV],
    [SDKWORK_IM_REALTIME_WEBSOCKET_PATH, SDKWORK_IM_API_PREFIX],
  );
}

export function resolveSdkworkChatIamCommandEnv({
  env = process.env,
  iamMode,
  target = 'desktop-dev',
} = {}) {
  const nextEnv = { ...env };
  const resolvedDeploymentMode = normalizeDeploymentMode(
    iamMode ?? nextEnv[SDKWORK_CHAT_IAM_DEPLOYMENT_MODE_ENV],
    resolveDefaultDeploymentMode(target),
  );
  const errors = [];
  const sdkworkIamMode = resolveSdkworkIamMode(resolvedDeploymentMode);
  validateConfiguredHttpEnv(nextEnv, [
    VITE_CRAW_CHAT_APP_API_BASE_URL_ENV,
    CRAW_CHAT_APP_API_BASE_URL_ENV,
    VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV,
    SDKWORK_IAM_APP_API_BASE_URL_ENV,
    SDKWORK_CHAT_SERVER_API_BASE_URL_ENV,
    SDKWORK_CHAT_SERVER_BASE_URL_ENV,
    CRAW_CHAT_SERVER_API_BASE_URL_ENV,
    CRAW_CHAT_SERVER_BASE_URL_ENV,
  ], errors);
  validateConfiguredHttpEnv(nextEnv, [
    VITE_CRAW_CHAT_IM_API_BASE_URL_ENV,
    CRAW_CHAT_IM_API_BASE_URL_ENV,
    SDKWORK_CHAT_SERVER_API_BASE_URL_ENV,
    SDKWORK_CHAT_SERVER_BASE_URL_ENV,
    CRAW_CHAT_SERVER_API_BASE_URL_ENV,
    CRAW_CHAT_SERVER_BASE_URL_ENV,
  ], errors);
  validateConfiguredWebSocketEnv(nextEnv, [
    VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV,
    CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV,
    SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL_ENV,
    CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL_ENV,
  ], errors);
  const useLocalEndpointDefaults = shouldUseLocalEndpointDefaults(target, resolvedDeploymentMode);
  const configuredApiBaseUrl = resolveConfiguredApiBaseUrl(nextEnv);
  const configuredImApiBaseUrl = resolveConfiguredImApiBaseUrl(nextEnv);
  const apiBaseUrl = configuredApiBaseUrl
    ?? (useLocalEndpointDefaults ? DEFAULT_SDKWORK_CHAT_LOCAL_APP_API_BASE_URL : undefined);
  const imApiBaseUrl = configuredImApiBaseUrl
    ?? (useLocalEndpointDefaults ? DEFAULT_SDKWORK_CHAT_LOCAL_IM_API_BASE_URL : undefined);
  const websocketBaseUrl = resolveConfiguredImWebSocketBaseUrl(nextEnv)
    ?? deriveWebSocketBaseUrlFromHttpBaseUrl(configuredImApiBaseUrl)
    ?? (useLocalEndpointDefaults ? DEFAULT_SDKWORK_CHAT_LOCAL_IM_WEBSOCKET_BASE_URL : undefined);

  setEnvValue(nextEnv, SDKWORK_CHAT_IAM_DEPLOYMENT_MODE_ENV, resolvedDeploymentMode);
  setEnvValue(nextEnv, VITE_SDKWORK_CHAT_IAM_DEPLOYMENT_MODE_ENV, resolvedDeploymentMode);
  setEnvValue(nextEnv, VITE_SDKWORK_DEPLOYMENT_MODE_ENV, resolvePublicDeploymentMode(resolvedDeploymentMode));
  setEnvValue(nextEnv, SDKWORK_IAM_MODE_ENV, sdkworkIamMode);

  setEnvValue(nextEnv, SDKWORK_IAM_APP_API_BASE_URL_ENV, apiBaseUrl);
  setEnvValue(nextEnv, VITE_SDKWORK_IAM_APP_API_BASE_URL_ENV, apiBaseUrl);
  setEnvValue(nextEnv, CRAW_CHAT_APP_API_BASE_URL_ENV, apiBaseUrl);
  setEnvValue(nextEnv, VITE_CRAW_CHAT_APP_API_BASE_URL_ENV, apiBaseUrl);
  setEnvValue(nextEnv, CRAW_CHAT_IM_API_BASE_URL_ENV, imApiBaseUrl);
  setEnvValue(nextEnv, VITE_CRAW_CHAT_IM_API_BASE_URL_ENV, imApiBaseUrl);
  setEnvValue(nextEnv, CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV, websocketBaseUrl);
  setEnvValue(nextEnv, VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV, websocketBaseUrl);

  if (sdkworkIamMode === 'local' || sdkworkIamMode === 'private') {
    setEnvDefault(nextEnv, SDKWORK_IAM_LOCAL_BOOTSTRAP_EMAIL_ENV, DEFAULT_SDKWORK_IAM_LOCAL_BOOTSTRAP_EMAIL);
    setEnvDefault(nextEnv, SDKWORK_IAM_LOCAL_BOOTSTRAP_PHONE_ENV, DEFAULT_SDKWORK_IAM_LOCAL_BOOTSTRAP_PHONE);
    setEnvDefault(nextEnv, SDKWORK_IAM_LOCAL_BOOTSTRAP_PASSWORD_ENV, DEFAULT_SDKWORK_IAM_LOCAL_BOOTSTRAP_PASSWORD);
    setEnvDefault(nextEnv, SDKWORK_IAM_LOCAL_VERIFY_CODE_FIXED_ENV, DEFAULT_SDKWORK_IAM_LOCAL_VERIFY_CODE);
  }

  if (!SDKWORK_CHAT_IAM_MODES.includes(sdkworkIamMode)) {
    errors.push(`${SDKWORK_IAM_MODE_ENV} must be one of: ${SDKWORK_CHAT_IAM_MODES.join(', ')}.`);
  }
  if (apiBaseUrl && !normalizeHttpBaseUrl(apiBaseUrl)) {
    errors.push(`${VITE_CRAW_CHAT_APP_API_BASE_URL_ENV} must be a valid http(s) URL.`);
  }
  if (imApiBaseUrl && !normalizeHttpBaseUrl(imApiBaseUrl)) {
    errors.push(`${VITE_CRAW_CHAT_IM_API_BASE_URL_ENV} must be a valid http(s) URL.`);
  }
  if (websocketBaseUrl && !normalizeWebSocketBaseUrl(websocketBaseUrl)) {
    errors.push(`${VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL_ENV} must be a valid ws(s) URL.`);
  }

  return {
    env: nextEnv,
    errors,
    iamMode: resolvedDeploymentMode,
    sdkworkIamMode,
  };
}
