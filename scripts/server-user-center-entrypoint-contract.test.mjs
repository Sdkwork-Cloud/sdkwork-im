import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const repoRoot = path.resolve(import.meta.dirname, '..');

const serverEnvTemplatePath = path.join(
  repoRoot,
  'deployments',
  'templates',
  'server.env.example',
);
const initConfigServerPs1Path = path.join(repoRoot, 'bin', 'init-config-server.ps1');
const initConfigServerShPath = path.join(repoRoot, 'bin', 'init-config-server.sh');
const startServerPs1Path = path.join(repoRoot, 'bin', 'start-server.ps1');
const startServerShPath = path.join(repoRoot, 'bin', 'start-server.sh');

const serverEnvTemplateSource = readFileSync(serverEnvTemplatePath, 'utf8');
const initConfigServerPs1Source = readFileSync(initConfigServerPs1Path, 'utf8');
const initConfigServerShSource = readFileSync(initConfigServerShPath, 'utf8');
const startServerPs1Source = readFileSync(startServerPs1Path, 'utf8');
const startServerShSource = readFileSync(startServerShPath, 'utf8');

test('craw-chat server deployment templates publish the canonical user-center env vocabulary', () => {
  for (const pattern of [
    /CRAW_CHAT_SERVER_USER_CENTER_MODE=builtin-local/u,
    /CRAW_CHAT_SERVER_USER_CENTER_PROVIDER_KEY=craw-chat-server-local/u,
    /CRAW_CHAT_SERVER_USER_CENTER_LOCAL_API_BASE_PATH=\/api\/app\/v1\/user-center/u,
    /SDKWORK_USER_CENTER_MODE=builtin-local/u,
    /SDKWORK_USER_CENTER_PROVIDER_KEY=craw-chat-server-local/u,
    /SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH=\/api\/app\/v1\/user-center/u,
    /CRAW_CHAT_USER_CENTER_MODE=builtin-local/u,
    /CRAW_CHAT_USER_CENTER_PROVIDER_KEY=craw-chat-server-local/u,
    /CRAW_CHAT_USER_CENTER_LOCAL_API_BASE_PATH=\/api\/app\/v1\/user-center/u,
    /CRAW_CHAT_SERVER_USER_CENTER_AUTHORIZATION_HEADER_NAME=Authorization/u,
    /CRAW_CHAT_SERVER_USER_CENTER_ACCESS_TOKEN_HEADER_NAME=Access-Token/u,
    /CRAW_CHAT_SERVER_USER_CENTER_REFRESH_TOKEN_HEADER_NAME=Refresh-Token/u,
    /CRAW_CHAT_SERVER_USER_CENTER_SESSION_HEADER_NAME=x-sdkwork-user-center-session-id/u,
    /CRAW_CHAT_SERVER_USER_CENTER_AUTHORIZATION_SCHEME=Bearer/u,
    /CRAW_CHAT_SERVER_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN=true/u,
    /CRAW_CHAT_SERVER_USER_CENTER_APP_API_BASE_URL=/u,
    /CRAW_CHAT_SERVER_USER_CENTER_SECRET_ID=/u,
    /CRAW_CHAT_SERVER_USER_CENTER_SHARED_SECRET=/u,
    /CRAW_CHAT_SERVER_USER_CENTER_EXTERNAL_BASE_URL=/u,
  ]) {
    assert.match(serverEnvTemplateSource, pattern);
  }
});

test('craw-chat init-config-server entrypoints expose and render canonical user-center options', () => {
  for (const [label, source] of [
    ['powershell', initConfigServerPs1Source],
    ['shell', initConfigServerShSource],
  ]) {
    assert.match(
      source,
      /user-center-mode/u,
      `${label} init-config-server help must expose the user-center mode option.`,
    );
    assert.match(
      source,
      /user-center-provider-key/u,
      `${label} init-config-server help must expose the provider-key option.`,
    );
    assert.match(
      source,
      /user-center-local-api-base-path/u,
      `${label} init-config-server help must expose the local-api option.`,
    );
    assert.match(
      source,
      /user-center-app-api-base-url/u,
      `${label} init-config-server help must expose the app-api base-url option.`,
    );
    assert.match(
      source,
      /user-center-secret-id/u,
      `${label} init-config-server help must expose the secret-id option.`,
    );
    assert.match(
      source,
      /user-center-shared-secret/u,
      `${label} init-config-server help must expose the shared-secret option.`,
    );
    assert.match(
      source,
      /user-center-external-base-url/u,
      `${label} init-config-server help must expose the external authority option.`,
    );
    assert.match(
      source,
      /CRAW_CHAT_SERVER_USER_CENTER_MODE/u,
      `${label} init-config-server must render the canonical server user-center mode env.`,
    );
    assert.match(
      source,
      /CRAW_CHAT_SERVER_USER_CENTER_PROVIDER_KEY/u,
      `${label} init-config-server must render the canonical server provider-key env.`,
    );
    assert.match(
      source,
      /CRAW_CHAT_SERVER_USER_CENTER_SHARED_SECRET/u,
      `${label} init-config-server must render the private shared-secret env.`,
    );
    assert.match(
      source,
      /SDKWORK_USER_CENTER_MODE/u,
      `${label} init-config-server must render the canonical sdkwork user-center mode env.`,
    );
    assert.match(
      source,
      /CRAW_CHAT_USER_CENTER_MODE/u,
      `${label} init-config-server must render the app-scoped user-center mode alias env.`,
    );
  }
});

test('craw-chat start-server entrypoints load server env and map canonical user-center runtime variables before launching the service', () => {
  for (const [label, source] of [
    ['powershell', startServerPs1Source],
    ['shell', startServerShSource],
  ]) {
    assert.match(
      source,
      /server\.env/u,
      `${label} start-server must load the instance server.env file.`,
    );
    assert.match(
      source,
      /env-file/u,
      `${label} start-server help must expose an env-file override.`,
    );
    assert.match(
      source,
      /SDKWORK_USER_CENTER_MODE/u,
      `${label} start-server must export the canonical user-center mode env.`,
    );
    assert.match(
      source,
      /CRAW_CHAT_USER_CENTER_MODE/u,
      `${label} start-server must export the app-scoped user-center mode alias.`,
    );
    assert.match(
      source,
      /CRAW_CHAT_SERVER_USER_CENTER_MODE/u,
      `${label} start-server must source the instance-scoped server user-center mode env.`,
    );
    assert.match(
      source,
      /SDKWORK_USER_CENTER_SHARED_SECRET/u,
      `${label} start-server must map private shared-secret env into the canonical runtime bridge.`,
    );
    assert.match(
      source,
      /SDKWORK_USER_CENTER_EXTERNAL_BASE_URL/u,
      `${label} start-server must map external authority env into the canonical runtime bridge.`,
    );
  }
});
